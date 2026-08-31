use core::sync::atomic::Ordering;

use arbitrary_int::u5;
use embassy_sync::waitqueue::AtomicWaker;
use portable_atomic::{AtomicBool, AtomicPtr, AtomicU8, AtomicUsize};

use crate::{
    shared::{FifoClear, TriggerLevel},
    spi::{
        BMSKIPDATA_MASK, BMSTART_BMSTOP_MASK, FIFO_DEPTH,
        regs::{Data, InterruptClear, InterruptControl},
    },
};

/// Builds a FIFO data word, marking the last word of the transfer with the BMSTART_BMSTOP bit.
///
/// That bit is what makes the peripheral end the frame and deassert a hardware chip select after
/// the word. Without it, blockmode keeps CS asserted and stalls the clock after the transfer.
#[inline]
fn data_word(value: u32, is_last: bool) -> Data {
    if is_last {
        Data::new_with_raw_value(value | BMSTART_BMSTOP_MASK)
    } else {
        Data::new_with_raw_value(value)
    }
}

#[cfg(feature = "vor1x")]
pub const NUM_SPIS: usize = 3;
#[cfg(feature = "vor4x")]
pub const NUM_SPIS: usize = 4;

static WAKERS: [AtomicWaker; NUM_SPIS] = [const { AtomicWaker::new() }; NUM_SPIS];
static TRANSFER_CONTEXTS: [TransferContext; NUM_SPIS] =
    [const { TransferContext::new() }; NUM_SPIS];
// Completion flag. Kept outside of the context structure as an atomic to avoid
// critical section.
static DONE: [AtomicBool; NUM_SPIS] = [const { AtomicBool::new(false) }; NUM_SPIS];

#[derive(Debug, Clone, Copy, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("SPI RX FIFO overrun")]
pub struct RxOverrunError;

impl embedded_hal_async::spi::Error for RxOverrunError {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        embedded_hal::spi::ErrorKind::Overrun
    }
}

fn on_interrupt(peripheral: super::Bank) {
    let mut spi = unsafe { peripheral.steal_regs() };
    let index = peripheral as usize;
    let enabled_irqs = spi.read_interrupt_control();
    let interrupt_status = spi.read_interrupt_status();
    spi.write_interrupt_clear(InterruptClear::ALL);
    // Prevent spurious interrupts from messing with out logic here.
    spi.write_interrupt_control(InterruptControl::DISABLE_ALL);
    // IRQ is not related.
    if enabled_irqs.raw_value() == 0 {
        reset_trigger_levels(&mut spi);
        spi.write_fifo_clear(FifoClear::ALL);
        return;
    }
    let context = &TRANSFER_CONTEXTS[index];
    // `Acquire` pairs with the `Release` store in the future constructors: observing an active
    // transfer type here guarantees the buffers and counters read below belong to that transfer.
    let Some(transfer_type) = context.active_transfer_type() else {
        // No transfer active.
        return;
    };
    if interrupt_status.rx_overrun() {
        // Not sure how to otherwise handle this cleanly..
        return handle_rx_overrun(&mut spi, index);
    }
    match transfer_type {
        TransferType::Read => on_interrupt_read(index, context, &mut spi, enabled_irqs),
        TransferType::Write => on_interrupt_write(index, context, &mut spi, enabled_irqs),
        TransferType::Transfer => on_interrupt_transfer(index, context, &mut spi, enabled_irqs),
        TransferType::TransferInPlace => {
            on_interrupt_transfer_in_place(index, context, &mut spi, enabled_irqs)
        }
    };
}

fn handle_rx_overrun(spi: &mut super::regs::MmioSpi<'static>, idx: usize) {
    TRANSFER_CONTEXTS[idx]
        .rx_overrun
        .store(true, Ordering::Relaxed);
    // Clean up, restore clean state.
    reset_trigger_levels(spi);
    spi.write_fifo_clear(FifoClear::ALL);
    // Interrupts were already disabled and cleared.
    // `Release` publishes `rx_overrun` to whichever context observes `DONE` in `poll`.
    DONE[idx].store(true, Ordering::Release);
    WAKERS[idx].wake();
}

fn on_interrupt_read(
    idx: usize,
    context: &TransferContext,
    spi: &mut super::regs::MmioSpi<'static>,
    enabled_irqs: InterruptControl,
) {
    // Safety: The gate was observed active, so the slice published with it is still valid.
    let read_slice = unsafe { context.rx_slice() };
    let transfer_len = read_slice.len();
    let mut rx_progress = context.rx_progress.load(Ordering::Relaxed);
    let mut tx_progress = context.tx_progress.load(Ordering::Relaxed);

    // Read data from RX FIFO first.
    while spi.read_status().rx_not_empty() {
        let data = spi.read_data();
        if rx_progress < transfer_len {
            read_slice[rx_progress] = (data.data() & 0xFF) as u8;
            rx_progress += 1;
        }
    }

    // The FIFO still needs to be pumped.
    while tx_progress < transfer_len && spi.read_status().tx_not_full() {
        spi.write_data(data_word(0, tx_progress == transfer_len - 1));
        tx_progress += 1;
    }

    isr_finish_handler(
        idx,
        spi,
        context,
        Progress::new(tx_progress, rx_progress),
        transfer_len,
        enabled_irqs,
    )
}

fn on_interrupt_write(
    idx: usize,
    context: &TransferContext,
    spi: &mut super::regs::MmioSpi<'static>,
    enabled_irqs: InterruptControl,
) {
    // Safety: The gate was observed active, so the slice published with it is still valid.
    let write_slice = unsafe { context.tx_slice() };
    let transfer_len = write_slice.len();
    let mut rx_progress = context.rx_progress.load(Ordering::Relaxed);
    let mut tx_progress = context.tx_progress.load(Ordering::Relaxed);

    // Read data from RX FIFO first.
    while spi.read_status().rx_not_empty() {
        spi.read_data();
        if rx_progress < transfer_len {
            rx_progress += 1;
        }
    }

    // Data still needs to be sent
    while tx_progress < transfer_len && spi.read_status().tx_not_full() {
        spi.write_data(data_word(
            write_slice[tx_progress] as u32,
            tx_progress == transfer_len - 1,
        ));
        tx_progress += 1;
    }

    isr_finish_handler(
        idx,
        spi,
        context,
        Progress::new(tx_progress, rx_progress),
        transfer_len,
        enabled_irqs,
    )
}

fn on_interrupt_transfer(
    idx: usize,
    context: &TransferContext,
    spi: &mut super::regs::MmioSpi<'static>,
    enabled_irqs: InterruptControl,
) {
    // Safety: The gate was observed active, so the slices published with it are still valid.
    let read_slice = unsafe { context.rx_slice() };
    let read_len = read_slice.len();
    let write_slice = unsafe { context.tx_slice() };
    let transfer_len = core::cmp::max(read_len, write_slice.len());
    let mut rx_progress = context.rx_progress.load(Ordering::Relaxed);
    let mut tx_progress = context.tx_progress.load(Ordering::Relaxed);

    // Send data first to avoid overwriting data that still needs to be sent.
    while tx_progress < transfer_len && spi.read_status().tx_not_full() {
        spi.write_data(data_word(
            write_slice.get(tx_progress).copied().unwrap_or(0) as u32,
            tx_progress == transfer_len - 1,
        ));
        // Always increment this.
        tx_progress += 1;
    }

    // Read data from RX FIFO.
    while spi.read_status().rx_not_empty() {
        let data = spi.read_data();
        if rx_progress < read_len {
            read_slice[rx_progress] = (data.data() & 0xFF) as u8;
        }
        // Always increment this.
        rx_progress += 1;
    }

    isr_finish_handler(
        idx,
        spi,
        context,
        Progress::new(tx_progress, rx_progress),
        transfer_len,
        enabled_irqs,
    )
}

fn on_interrupt_transfer_in_place(
    idx: usize,
    context: &TransferContext,
    spi: &mut super::regs::MmioSpi<'static>,
    enabled_irqs: InterruptControl,
) {
    // Safety: The gate was observed active, so the slice published with it is still valid.
    let transfer_slice = unsafe { context.rx_slice() };
    let transfer_len = transfer_slice.len();
    let mut rx_progress = context.rx_progress.load(Ordering::Relaxed);
    let mut tx_progress = context.tx_progress.load(Ordering::Relaxed);

    // Send data first to avoid overwriting data that still needs to be sent.
    while tx_progress < transfer_len && spi.read_status().tx_not_full() {
        spi.write_data(data_word(
            transfer_slice[tx_progress] as u32,
            tx_progress == transfer_len - 1,
        ));
        tx_progress += 1;
    }
    // Read data from RX FIFO.
    while spi.read_status().rx_not_empty() {
        let data = spi.read_data();
        if rx_progress < transfer_len {
            transfer_slice[rx_progress] = (data.data() & 0xFF) as u8;
            rx_progress += 1;
        }
    }

    isr_finish_handler(
        idx,
        spi,
        context,
        Progress::new(tx_progress, rx_progress),
        transfer_len,
        enabled_irqs,
    )
}

/// TX and RX progress of the running transfer, as tracked inside one interrupt handler run.
///
/// The handlers keep the counters in locals and only write them back once, so the shared state
/// is touched a single time per interrupt instead of on every FIFO word.
#[derive(Debug, Clone, Copy)]
struct Progress {
    tx: usize,
    rx: usize,
}

impl Progress {
    #[inline]
    const fn new(tx: usize, rx: usize) -> Self {
        Self { tx, rx }
    }
}

/// Generic handler after RX FIFO and TX FIFO were handled. Checks and handles finished
/// and unfinished conditions.
fn isr_finish_handler(
    idx: usize,
    spi: &mut super::regs::MmioSpi<'static>,
    context: &TransferContext,
    progress: Progress,
    transfer_len: usize,
    enabled: InterruptControl,
) {
    // Transfer finish condition.
    if progress.rx == progress.tx && progress.rx == transfer_len {
        finish_transfer(spi, idx);
        return;
    }
    // Write back the updated counters. The gate stays open, so the ISR keeps servicing this
    // transfer on the following interrupts.
    context.tx_progress.store(progress.tx, Ordering::Relaxed);
    context.rx_progress.store(progress.rx, Ordering::Relaxed);
    unfinished_transfer(spi, transfer_len, progress, enabled);
}

fn finish_transfer(spi: &mut super::regs::MmioSpi<'static>, idx: usize) {
    // Clean up, restore clean state.
    reset_trigger_levels(spi);
    spi.write_fifo_clear(FifoClear::ALL);
    // Interrupts were already disabled and cleared.
    // `Release` publishes the completed transfer state to whichever context observes `DONE`
    // via the `Acquire` swap in `poll`.
    DONE[idx].store(true, Ordering::Release);
    WAKERS[idx].wake();
}

#[inline]
fn unfinished_transfer(
    spi: &mut super::regs::MmioSpi<'static>,
    transfer_len: usize,
    progress: Progress,
    enabled_irqs: InterruptControl,
) {
    // Take 8 as a conservative value to make sure that the FIFO does not overflow even if there
    // is a significant delay between the interrupt being triggered and the handler being executed.
    let new_trig_level = core::cmp::min(8, transfer_len - progress.rx);
    spi.write_rx_fifo_trigger(TriggerLevel::new(u5::new(new_trig_level as u8)));

    // If TX was already enabled and the transfer is finished, stop enabling it. Otherwise, we can
    // become stuck in an interrupt loop. In any other case, enable it. I am not fully sure
    // why this is necessary and why we can not stop interrupts as soon as we have the full
    // TX progress, but tests with ADCs have shown that not doing this causes timeouts.
    let enable_tx = !(enabled_irqs.tx() && progress.tx == transfer_len);

    // Re-enable interrupts with the new RX FIFO trigger level.
    spi.write_interrupt_control(
        InterruptControl::builder()
            .with_tx(enable_tx)
            .with_rx(true)
            .with_rx_timeout(true)
            .with_rx_overrun(true)
            .build(),
    );
}

#[inline]
fn reset_trigger_levels(spi: &mut super::regs::MmioSpi<'static>) {
    spi.write_rx_fifo_trigger(TriggerLevel::new(u5::new(0x08)));
    spi.write_tx_fifo_trigger(TriggerLevel::new(u5::new(0x00)));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TransferType {
    Read = 0,
    Write = 1,
    Transfer = 2,
    TransferInPlace = 3,
}

impl TransferType {
    /// Stored in [TransferContext::transfer_type] while no transfer is active.
    const NONE: u8 = 0xff;

    const fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            0 => Some(Self::Read),
            1 => Some(Self::Write),
            2 => Some(Self::Transfer),
            3 => Some(Self::TransferInPlace),
            _ => None,
        }
    }
}

/// Transfer context structure. Plain atomics rather than a `critical_section::Mutex<RefCell<_>>`
/// so it can live in a `static` array directly and the interrupt handler does not need a
/// critical section.
///
/// `transfer_type` doubles as the "transfer active" flag: it is always published last
/// (`Release`) after the buffers and progress counters, and read first (`Acquire`) before them.
/// A reader which observes an active transfer type is therefore guaranteed to see the matching
/// buffers and counters, rather than stale ones from a previous transfer.
pub struct TransferContext {
    transfer_type: AtomicU8,
    tx_progress: AtomicUsize,
    rx_progress: AtomicUsize,
    tx_ptr: AtomicPtr<u8>,
    tx_len: AtomicUsize,
    rx_ptr: AtomicPtr<u8>,
    rx_len: AtomicUsize,
    rx_overrun: AtomicBool,
}

impl TransferContext {
    const fn new() -> Self {
        Self {
            transfer_type: AtomicU8::new(TransferType::NONE),
            tx_progress: AtomicUsize::new(0),
            rx_progress: AtomicUsize::new(0),
            tx_ptr: AtomicPtr::new(core::ptr::null_mut()),
            tx_len: AtomicUsize::new(0),
            rx_ptr: AtomicPtr::new(core::ptr::null_mut()),
            rx_len: AtomicUsize::new(0),
            rx_overrun: AtomicBool::new(false),
        }
    }

    /// Arms the gate. Must be called after all other fields were stored.
    #[inline]
    fn arm(&self, transfer_type: TransferType) {
        self.transfer_type
            .store(transfer_type as u8, Ordering::Release);
    }

    /// Disarms the gate, so a spurious interrupt can never observe the stale buffer pointers.
    #[inline]
    fn disarm(&self) {
        self.transfer_type
            .store(TransferType::NONE, Ordering::Release);
    }

    /// Reads the gate. All other fields may only be read if this returns [Some].
    #[inline]
    fn active_transfer_type(&self) -> Option<TransferType> {
        TransferType::from_raw(self.transfer_type.load(Ordering::Acquire))
    }

    /// # Safety
    ///
    /// The caller must ensure the slice outlives the transfer.
    #[inline]
    unsafe fn set_tx_slice(&self, data: &[u8]) {
        self.tx_ptr
            .store(data.as_ptr().cast_mut(), Ordering::Relaxed);
        self.tx_len.store(data.len(), Ordering::Relaxed);
    }

    /// # Safety
    ///
    /// The caller must ensure the slice outlives the transfer.
    #[inline]
    unsafe fn set_rx_slice(&self, data: &mut [u8]) {
        self.rx_ptr.store(data.as_mut_ptr(), Ordering::Relaxed);
        self.rx_len.store(data.len(), Ordering::Relaxed);
    }

    #[inline]
    fn clear_tx_slice(&self) {
        self.tx_ptr.store(core::ptr::null_mut(), Ordering::Relaxed);
        self.tx_len.store(0, Ordering::Relaxed);
    }

    #[inline]
    fn clear_rx_slice(&self) {
        self.rx_ptr.store(core::ptr::null_mut(), Ordering::Relaxed);
        self.rx_len.store(0, Ordering::Relaxed);
    }

    /// # Safety
    ///
    /// Only valid while the transfer which published the slice is still active.
    #[inline]
    unsafe fn tx_slice(&self) -> &'static [u8] {
        let ptr = self.tx_ptr.load(Ordering::Relaxed);
        if ptr.is_null() {
            return &[];
        }
        unsafe {
            core::slice::from_raw_parts(ptr as *const u8, self.tx_len.load(Ordering::Relaxed))
        }
    }

    /// # Safety
    ///
    /// Only valid while the transfer which published the slice is still active. The caller must
    /// not create a second alias to the same buffer.
    #[inline]
    #[allow(clippy::mut_from_ref)]
    unsafe fn rx_slice(&self) -> &'static mut [u8] {
        let ptr = self.rx_ptr.load(Ordering::Relaxed);
        if ptr.is_null() {
            return &mut [];
        }
        unsafe { core::slice::from_raw_parts_mut(ptr, self.rx_len.load(Ordering::Relaxed)) }
    }

    /// Closes the gate and restores the initial state, so the slot can be reused.
    #[inline]
    fn reset(&self) {
        // Must come first in program order: a live interrupt (e.g. during Drop's
        // cancellation path) must never see the fields below cleared while still
        // observing an armed transfer.
        self.disarm();
        self.tx_progress.store(0, Ordering::Relaxed);
        self.rx_progress.store(0, Ordering::Relaxed);
        self.clear_tx_slice();
        self.clear_rx_slice();
        self.rx_overrun.store(false, Ordering::Relaxed);
    }
}

pub struct SpiFuture<'spi, 'read, 'write> {
    bank: super::Bank,
    spi: &'spi mut super::Spi<u8>,
    empty_buffer: bool,
    finished_regularly: core::cell::Cell<bool>,
    phantom_read: core::marker::PhantomData<(&'read (), &'write ())>,
}

impl<'spi, 'read, 'write> SpiFuture<'spi, 'read, 'write> {
    fn new_for_read(
        spi: &'spi mut super::Spi<u8>,
        bank: super::Bank,
        words: &'read mut [u8],
    ) -> Self {
        if words.is_empty() {
            return Self {
                bank,
                spi,
                empty_buffer: true,
                finished_regularly: core::cell::Cell::new(false),
                phantom_read: core::marker::PhantomData,
            };
        }
        Self::generic_init_transfer(spi, bank);

        let len = words.len();
        let write_index = core::cmp::min(super::FIFO_DEPTH, len);
        // Send dummy bytes.
        (0..write_index).for_each(|idx| {
            spi.regs.write_data(data_word(0, idx == len - 1));
        });

        Self::set_triggers(spi, write_index, len);

        let context = &TRANSFER_CONTEXTS[bank as usize];
        // Publish the guarded fields before opening the gate, see [TransferContext].
        // Safety: The future borrows `words` for its lifetime and the `Drop` impl closes the gate.
        unsafe { context.set_rx_slice(words) };
        context.clear_tx_slice();
        context.tx_progress.store(write_index, Ordering::Relaxed);
        context.rx_progress.store(0, Ordering::Relaxed);
        context.rx_overrun.store(false, Ordering::Relaxed);
        context.arm(TransferType::Read);

        spi.regs.write_interrupt_clear(InterruptClear::ALL);
        spi.regs
            .write_interrupt_control(InterruptControl::ENABLE_ALL.with_tx(len > FIFO_DEPTH));
        spi.regs.modify_ctrl1(|v| v.with_mtxpause(false));
        Self {
            bank,
            spi,
            empty_buffer: false,
            finished_regularly: core::cell::Cell::new(false),
            phantom_read: core::marker::PhantomData,
        }
    }

    fn new_for_write(
        spi: &'spi mut super::Spi<u8>,
        bank: super::Bank,
        words: &'write [u8],
    ) -> Self {
        if words.is_empty() {
            return Self {
                bank,
                spi,
                empty_buffer: true,
                finished_regularly: core::cell::Cell::new(false),
                phantom_read: core::marker::PhantomData,
            };
        }
        let index = bank as usize;
        let write_index = Self::generic_init_transfer_write_transfer_in_place(spi, bank, words);
        let context = &TRANSFER_CONTEXTS[index];
        // Publish the guarded fields before opening the gate, see [TransferContext].
        // Safety: The future borrows `words` for its lifetime and the `Drop` impl closes the gate.
        unsafe { context.set_tx_slice(words) };
        context.clear_rx_slice();
        context.tx_progress.store(write_index, Ordering::Relaxed);
        context.rx_progress.store(0, Ordering::Relaxed);
        context.rx_overrun.store(false, Ordering::Relaxed);
        context.arm(TransferType::Write);

        spi.regs.write_interrupt_clear(InterruptClear::ALL);
        spi.regs.write_interrupt_control(
            InterruptControl::ENABLE_ALL.with_tx(words.len() > FIFO_DEPTH),
        );
        spi.regs.modify_ctrl1(|v| v.with_mtxpause(false));
        Self {
            bank,
            spi,
            empty_buffer: false,
            finished_regularly: core::cell::Cell::new(false),
            phantom_read: core::marker::PhantomData,
        }
    }

    fn new_for_transfer(
        spi: &'spi mut super::Spi<u8>,
        bank: super::Bank,
        read: &'read mut [u8],
        write: &'write [u8],
    ) -> Self {
        if read.is_empty() || write.is_empty() {
            return Self {
                bank,
                spi,
                empty_buffer: true,
                finished_regularly: core::cell::Cell::new(false),
                phantom_read: core::marker::PhantomData,
            };
        }
        let index = bank as usize;
        let full_write_len = core::cmp::max(read.len(), write.len());
        let fifo_prefill = core::cmp::min(super::FIFO_DEPTH, full_write_len);

        Self::generic_init_transfer(spi, bank);

        for write_index in 0..fifo_prefill {
            let value = write.get(write_index).copied().unwrap_or(0);
            spi.regs
                .write_data(data_word(value as u32, write_index == full_write_len - 1));
        }

        Self::set_triggers(spi, fifo_prefill, full_write_len);

        let context = &TRANSFER_CONTEXTS[index];
        // Publish the guarded fields before opening the gate, see [TransferContext].
        // Safety: The future borrows both buffers for its lifetime and the `Drop` impl closes
        // the gate.
        unsafe {
            context.set_tx_slice(write);
            context.set_rx_slice(read);
        }
        context.tx_progress.store(fifo_prefill, Ordering::Relaxed);
        context.rx_progress.store(0, Ordering::Relaxed);
        context.rx_overrun.store(false, Ordering::Relaxed);
        context.arm(TransferType::Transfer);

        spi.regs.write_interrupt_clear(InterruptClear::ALL);
        spi.regs.write_interrupt_control(
            InterruptControl::ENABLE_ALL.with_tx(fifo_prefill > FIFO_DEPTH),
        );
        spi.regs.modify_ctrl1(|v| v.with_mtxpause(false));
        Self {
            bank,
            spi,
            empty_buffer: false,
            finished_regularly: core::cell::Cell::new(false),
            phantom_read: core::marker::PhantomData,
        }
    }

    fn new_for_transfer_in_place(
        spi: &'spi mut super::Spi<u8>,
        bank: super::Bank,
        words: &'read mut [u8],
    ) -> Self {
        if words.is_empty() {
            return Self {
                bank,
                spi,
                empty_buffer: true,
                finished_regularly: core::cell::Cell::new(false),
                phantom_read: core::marker::PhantomData,
            };
        }
        let write_idx = Self::generic_init_transfer_write_transfer_in_place(spi, bank, words);
        let len = words.len();
        let context = &TRANSFER_CONTEXTS[bank as usize];
        // Publish the guarded fields before opening the gate, see [TransferContext].
        // Safety: The future borrows `words` for its lifetime and the `Drop` impl closes the gate.
        unsafe { context.set_rx_slice(words) };
        context.clear_tx_slice();
        context.tx_progress.store(write_idx, Ordering::Relaxed);
        context.rx_progress.store(0, Ordering::Relaxed);
        context.rx_overrun.store(false, Ordering::Relaxed);
        context.arm(TransferType::TransferInPlace);

        spi.regs.write_interrupt_clear(InterruptClear::ALL);
        spi.regs
            .write_interrupt_control(InterruptControl::ENABLE_ALL.with_tx(len > FIFO_DEPTH));
        spi.regs.modify_ctrl1(|v| v.with_mtxpause(false));
        Self {
            bank,
            spi,
            empty_buffer: false,
            finished_regularly: core::cell::Cell::new(false),
            phantom_read: core::marker::PhantomData,
        }
    }

    fn generic_init_transfer(spi: &mut super::Spi<u8>, bank: super::Bank) {
        let idx = bank as usize;
        DONE[idx].store(false, core::sync::atomic::Ordering::Relaxed);
        spi.regs
            .write_interrupt_control(InterruptControl::DISABLE_ALL);
        spi.regs.write_fifo_clear(FifoClear::ALL);
        spi.regs.modify_ctrl1(|v| v.with_mtxpause(true));
    }

    // Returns amount of bytes written to FIFO.
    fn generic_init_transfer_write_transfer_in_place(
        spi: &mut super::Spi<u8>,
        bank: super::Bank,
        write: &[u8],
    ) -> usize {
        Self::generic_init_transfer(spi, bank);

        let write_idx = core::cmp::min(super::FIFO_DEPTH, write.len());
        (0..write_idx).for_each(|idx| {
            spi.regs
                .write_data(data_word(write[idx] as u32, idx == write.len() - 1));
        });

        Self::set_triggers(spi, write_idx, write.len());
        write_idx
    }

    fn set_triggers(spi: &mut super::Spi<u8>, fifo_prefill: usize, write_len: usize) {
        spi.regs
            .write_rx_fifo_trigger(TriggerLevel::new(u5::new(core::cmp::min(
                fifo_prefill,
                FIFO_DEPTH / 2,
            ) as u8)));
        // We want to re-fill the TX FIFO before it is completely empty if the full transfer size
        // is larger than the FIFO depth. Otherwise, set it to 0. Not exactly sure what that does,
        // but we do not enable interrupts anyway.
        if write_len > super::FIFO_DEPTH {
            spi.regs
                .write_tx_fifo_trigger(TriggerLevel::new(u5::new(8)));
        } else {
            spi.regs
                .write_tx_fifo_trigger(TriggerLevel::new(u5::new(0)));
        }
    }
}

impl<'spi> Future for SpiFuture<'spi, '_, '_> {
    type Output = Result<(), RxOverrunError>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        if self.empty_buffer {
            return core::task::Poll::Ready(Ok(()));
        }
        WAKERS[self.bank as usize].register(cx.waker());
        // `Acquire` pairs with the `Release` store in `finish_transfer`/`handle_rx_overrun`.
        if DONE[self.bank as usize].swap(false, Ordering::Acquire) {
            let context = &TRANSFER_CONTEXTS[self.bank as usize];
            let rx_overrun = context.rx_overrun.load(Ordering::Relaxed);
            // Closes the gate, so a spurious interrupt for this bank can never dereference the
            // buffer pointers of the transfer which just finished.
            context.reset();
            self.finished_regularly.set(true);
            if rx_overrun {
                return core::task::Poll::Ready(Err(RxOverrunError));
            }
            return core::task::Poll::Ready(Ok(()));
        }
        core::task::Poll::Pending
    }
}

impl<'spi> Drop for SpiFuture<'spi, '_, '_> {
    fn drop(&mut self) {
        if !self.finished_regularly.get() && !self.empty_buffer {
            // On cancellation, close the gate so a spurious or late interrupt for this bank can
            // never dereference the buffer pointers of the cancelled transfer. `finished_regularly`
            // is what distinguishes cancellation from completion here, since `poll` already swapped
            // `DONE` back to `false` by the time a completed future is dropped.
            TRANSFER_CONTEXTS[self.bank as usize].reset();
            // It might be sufficient to disable and enable the SPI.. But this definitely
            // ensures the SPI is fully reset.
            self.spi.regs.write_interrupt_clear(InterruptClear::ALL);
            self.spi
                .regs
                .write_interrupt_control(InterruptControl::DISABLE_ALL);
            self.spi.regs.write_fifo_clear(FifoClear::ALL);
            // Clearing the FIFO does not end the blockmode frame, so the chip select would stay
            // asserted for a cancelled transfer. BMSKIPDATA together with BMSTOP ends the frame
            // without clocking out another data word. This has to happen after the FIFO clear,
            // otherwise the word is discarded again.
            self.spi.regs.write_data(Data::new_with_raw_value(
                BMSTART_BMSTOP_MASK | BMSKIPDATA_MASK,
            ));
        }
    }
}

/// Asynchronous SPI driver.
///
/// This is the primary data structure used to perform non-blocking SPI operations.
/// It implements the [embedded_hal_async::spi::SpiBus] as well.
pub struct Spi(pub super::Spi<u8>);

impl Spi {
    /// Construct an asynchronous SPI driver for the given SPI peripheral.
    ///
    /// # Safety
    ///
    /// The user MUST ensure that the `Drop` method of all futures generated with this driver
    /// is called on transfer cancellation. By default, this does not require any special handling.
    /// This case was considered exotic enough to justify not making the function `unsafe`.
    pub fn new(
        mut spi: super::Spi<u8>,
        #[cfg(feature = "vor1x")] opt_irq_cfg: Option<crate::InterruptConfig>,
    ) -> Self {
        #[cfg(feature = "vor1x")]
        if let Some(irq_cfg) = opt_irq_cfg {
            spi.regs
                .write_interrupt_control(InterruptControl::DISABLE_ALL);
            spi.regs.write_interrupt_clear(InterruptClear::ALL);
            if irq_cfg.route {
                crate::enable_peripheral_clock(crate::PeripheralSelect::Irqsel);
                unsafe { va108xx::Irqsel::steal() }
                    .spi(spi.id as usize)
                    .write(|w| unsafe { w.bits(irq_cfg.id as u32) });
            }
            if irq_cfg.enable_in_nvic {
                // Safety: User has specifically configured this.
                unsafe { crate::enable_nvic_interrupt(irq_cfg.id) };
            }
        }
        // The async driver always drives blockmode frames explicitly by setting the
        // BMSTART_BMSTOP bit on the last word of each transfer, which is what deasserts a
        // hardware chip select at the end of the transfer.
        spi.regs
            .modify_ctrl1(|v| v.with_bm_stall(true).with_blockmode(true));
        Self(spi)
    }

    /// Token identifying the SPI peripheral driven by this instance.
    ///
    /// Pass this to [Self::on_interrupt] to service the peripheral's interrupts. Since it is
    /// `Copy`, it can be stashed in a `Mutex<Cell<_>>` or similar and handed to the interrupt
    /// handler without needing access to the [Spi] instance itself.
    #[inline]
    pub fn bank_id(&self) -> super::Bank {
        self.0.id
    }

    /// Generic interrupt handler to handle asynchronous SPI operations for a given SPI
    /// peripheral.
    ///
    /// The user has to call this once in the interrupt handler responsible for the SPI
    /// interrupts on the given SPI bank. Takes the token returned by [Self::bank_id] rather than
    /// the [Spi] instance itself, so it can be called from interrupt context without needing
    /// access to the driver.
    pub fn on_interrupt(bank_id: super::Bank) {
        on_interrupt(bank_id);
    }

    /// Future which read `words` from the slave.
    ///
    /// Returns [None] if the provided buffer is empty.
    pub fn read<'read>(&mut self, words: &'read mut [u8]) -> SpiFuture<'_, 'read, '_> {
        let id = self.0.id;
        SpiFuture::new_for_read(&mut self.0, id, words)
    }

    /// Future which writes `words` to the slave, ignoring all the incoming words.
    ///
    /// Returns [None] if the provided buffer is empty.
    pub fn write<'write>(&mut self, words: &'write [u8]) -> SpiFuture<'_, '_, 'write> {
        let id = self.0.id;
        SpiFuture::new_for_write(&mut self.0, id, words)
    }

    /// Future which writes and reads simultaneously. `write` is written to the slave on MOSI and
    /// words received on MISO are stored in `read`.
    ///
    /// It is allowed for `read` and `write` to have different lengths, even zero length.
    /// The transfer runs for `max(read.len(), write.len())` words. If `read` is shorter,
    /// incoming words after `read` has been filled will be discarded. If `write` is shorter,
    /// the value of words sent in MOSI after all `write` has been sent is 0.
    ///
    /// Returns [None] if either of the provided buffers is empty.
    pub fn transfer<'read, 'write>(
        &mut self,
        read: &'read mut [u8],
        write: &'write [u8],
    ) -> SpiFuture<'_, 'read, 'write> {
        let id = self.0.id;
        SpiFuture::new_for_transfer(&mut self.0, id, read, write)
    }

    /// Future which writes and reads simultaneously. The contents of `words` are
    /// written to the slave, and the received words are stored into the same
    /// `words` buffer, overwriting it.
    ///
    /// Returns [None] if the provided buffer is empty.
    pub fn transfer_in_place<'read>(&mut self, words: &'read mut [u8]) -> SpiFuture<'_, 'read, '_> {
        let id = self.0.id;
        SpiFuture::new_for_transfer_in_place(&mut self.0, id, words)
    }
}

impl embedded_hal_async::spi::ErrorType for Spi {
    type Error = RxOverrunError;
}

impl embedded_hal_async::spi::SpiBus for Spi {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        if words.is_empty() {
            return Ok(());
        }
        self.read(words).await
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        if words.is_empty() {
            return Ok(());
        }
        self.write(words).await
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        if read.is_empty() && write.is_empty() {
            return Ok(());
        }
        self.transfer(read, write).await
    }

    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        if words.is_empty() {
            return Ok(());
        }
        self.transfer_in_place(words).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
