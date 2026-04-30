use core::cell::RefCell;

use arbitrary_int::u5;
use critical_section::Mutex;
use embassy_sync::waitqueue::AtomicWaker;
use portable_atomic::AtomicBool;
use raw_slice::{RawBufSlice, RawBufSliceMut};

use crate::{
    shared::{FifoClear, TriggerLevel},
    spi::{
        FIFO_DEPTH,
        regs::{Data, InterruptClear, InterruptControl},
    },
};

#[cfg(feature = "vor1x")]
pub const NUM_SPIS: usize = 3;
#[cfg(feature = "vor4x")]
pub const NUM_SPIS: usize = 4;

static WAKERS: [AtomicWaker; NUM_SPIS] = [const { AtomicWaker::new() }; NUM_SPIS];
static TRANSFER_CONTEXTS: [Mutex<RefCell<TransferContext>>; NUM_SPIS] =
    [const { Mutex::new(RefCell::new(TransferContext::new())) }; NUM_SPIS];
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

/// This is a generic interrupt handler to handle asynchronous SPI  operations for a given
/// SPI peripheral.
///
/// The user has to call this once in the interrupt handler responsible for the SPI interrupts on
/// the given SPI bank.
pub fn on_interrupt(peripheral: super::Bank) {
    let mut spi = unsafe { peripheral.steal_regs() };
    let index = peripheral as usize;
    let enabled_irqs = spi.read_interrupt_control();
    spi.write_interrupt_clear(InterruptClear::ALL);
    // Prevent spurious interrupts from messing with out logic here.
    spi.write_interrupt_control(InterruptControl::DISABLE_ALL);
    // IRQ is not related.
    if enabled_irqs.raw_value() == 0 {
        reset_trigger_levels(&mut spi);
        spi.write_fifo_clear(FifoClear::ALL);
        return;
    }
    if spi.read_interrupt_status().rx_overrun() {
        // Not sure how to otherwise handle this cleanly..
        return handle_rx_overrun(&mut spi, index);
    }
    let mut context = critical_section::with(|cs| {
        let context_ref = TRANSFER_CONTEXTS[index].borrow(cs);
        *context_ref.borrow()
    });
    // No transfer active.
    if context.transfer_type.is_none() {
        return;
    }
    let transfer_type = context.transfer_type.unwrap();
    match transfer_type {
        TransferType::Read => on_interrupt_read(index, &mut context, &mut spi, enabled_irqs),
        TransferType::Write => on_interrupt_write(index, &mut context, &mut spi, enabled_irqs),
        TransferType::Transfer => {
            on_interrupt_transfer(index, &mut context, &mut spi, enabled_irqs)
        }
        TransferType::TransferInPlace => {
            on_interrupt_transfer_in_place(index, &mut context, &mut spi, enabled_irqs)
        }
    };
}

fn handle_rx_overrun(spi: &mut super::regs::MmioSpi<'static>, idx: usize) {
    critical_section::with(|cs| {
        let context_ref = TRANSFER_CONTEXTS[idx].borrow(cs);
        context_ref.borrow_mut().rx_overrun = true;
    });
    // Clean up, restore clean state.
    reset_trigger_levels(spi);
    spi.write_fifo_clear(FifoClear::ALL);
    // Interrupts were already disabled and cleared.
    DONE[idx].store(true, core::sync::atomic::Ordering::Relaxed);
    WAKERS[idx].wake();
}

fn on_interrupt_read(
    idx: usize,
    context: &mut TransferContext,
    spi: &mut super::regs::MmioSpi<'static>,
    enabled_irqs: InterruptControl,
) {
    let read_slice = unsafe { context.rx_slice.get_mut().unwrap() };
    let transfer_len = read_slice.len();

    // Read data from RX FIFO first.
    while spi.read_status().rx_not_empty() {
        let data = spi.read_data();
        if context.rx_progress < transfer_len {
            read_slice[context.rx_progress] = (data.data() & 0xFF) as u8;
            context.rx_progress += 1;
        }
    }

    // The FIFO still needs to be pumped.
    while context.tx_progress < read_slice.len() && spi.read_status().tx_not_full() {
        spi.write_data(Data::new_with_raw_value(0));
        context.tx_progress += 1;
    }

    isr_finish_handler(idx, spi, context, transfer_len, enabled_irqs)
}

fn on_interrupt_write(
    idx: usize,
    context: &mut TransferContext,
    spi: &mut super::regs::MmioSpi<'static>,
    enabled_irqs: InterruptControl,
) {
    let write_slice = unsafe { context.tx_slice.get().unwrap() };
    let transfer_len = write_slice.len();

    // Read data from RX FIFO first.
    while spi.read_status().rx_not_empty() {
        spi.read_data();
        if context.rx_progress < transfer_len {
            context.rx_progress += 1;
        }
    }

    // Data still needs to be sent
    while context.tx_progress < transfer_len && spi.read_status().tx_not_full() {
        spi.write_data(Data::new_with_raw_value(
            write_slice[context.tx_progress] as u32,
        ));
        context.tx_progress += 1;
    }

    isr_finish_handler(idx, spi, context, transfer_len, enabled_irqs)
}

fn on_interrupt_transfer(
    idx: usize,
    context: &mut TransferContext,
    spi: &mut super::regs::MmioSpi<'static>,
    enabled_irqs: InterruptControl,
) {
    let read_slice = unsafe { context.rx_slice.get_mut().unwrap() };
    let read_len = read_slice.len();
    let write_slice = unsafe { context.tx_slice.get().unwrap() };
    let write_len = write_slice.len();
    let transfer_len = core::cmp::max(read_len, write_len);

    // Send data first to avoid overwriting data that still needs to be sent.
    while context.tx_progress < transfer_len && spi.read_status().tx_not_full() {
        spi.write_data(Data::new_with_raw_value(
            write_slice.get(context.tx_progress).copied().unwrap_or(0) as u32,
        ));
        // Always increment this.
        context.tx_progress += 1;
    }

    // Read data from RX FIFO.
    while spi.read_status().rx_not_empty() {
        let data = spi.read_data();
        if context.rx_progress < read_len {
            read_slice[context.rx_progress] = (data.data() & 0xFF) as u8;
        }
        // Always increment this.
        context.rx_progress += 1;
    }

    isr_finish_handler(idx, spi, context, transfer_len, enabled_irqs)
}

fn on_interrupt_transfer_in_place(
    idx: usize,
    context: &mut TransferContext,
    spi: &mut super::regs::MmioSpi<'static>,
    enabled_irqs: InterruptControl,
) {
    let transfer_slice = unsafe { context.rx_slice.get_mut().unwrap() };
    let transfer_len = transfer_slice.len();
    // Send data first to avoid overwriting data that still needs to be sent.
    while context.tx_progress < transfer_len && spi.read_status().tx_not_full() {
        spi.write_data(Data::new_with_raw_value(
            transfer_slice[context.tx_progress] as u32,
        ));
        context.tx_progress += 1;
    }
    // Read data from RX FIFO.
    while spi.read_status().rx_not_empty() {
        let data = spi.read_data();
        if context.rx_progress < transfer_len {
            transfer_slice[context.rx_progress] = (data.data() & 0xFF) as u8;
            context.rx_progress += 1;
        }
    }

    isr_finish_handler(idx, spi, context, transfer_len, enabled_irqs)
}

/// Generic handler after RX FIFO and TX FIFO were handled. Checks and handles finished
/// and unfinished conditions.
fn isr_finish_handler(
    idx: usize,
    spi: &mut super::regs::MmioSpi<'static>,
    context: &mut TransferContext,
    transfer_len: usize,
    enabled: InterruptControl,
) {
    // Transfer finish condition.
    if context.rx_progress == context.tx_progress && context.rx_progress == transfer_len {
        finish_transfer(spi, idx, context);
        return;
    }
    // If the transfer is done, the context structure was already written back.
    // Write back updated context structure.
    critical_section::with(|cs| {
        let context_ref = TRANSFER_CONTEXTS[idx].borrow(cs);
        *context_ref.borrow_mut() = *context;
    });
    unfinished_transfer(spi, transfer_len, context, enabled);
}

fn finish_transfer(
    spi: &mut super::regs::MmioSpi<'static>,
    idx: usize,
    context: &mut TransferContext,
) {
    // Write back updated context structure.
    critical_section::with(|cs| {
        let context_ref = TRANSFER_CONTEXTS[idx].borrow(cs);
        *context_ref.borrow_mut() = *context;
    });
    // Clean up, restore clean state.
    reset_trigger_levels(spi);
    spi.write_fifo_clear(FifoClear::ALL);
    // Interrupts were already disabled and cleared.
    DONE[idx].store(true, core::sync::atomic::Ordering::Relaxed);
    WAKERS[idx].wake();
}

#[inline]
fn unfinished_transfer(
    spi: &mut super::regs::MmioSpi<'static>,
    transfer_len: usize,
    context: &TransferContext,
    enabled_irqs: InterruptControl,
) {
    let new_trig_level = core::cmp::min(super::FIFO_DEPTH, transfer_len - context.rx_progress);
    spi.write_rx_fifo_trigger(TriggerLevel::new(u5::new(new_trig_level as u8)));

    // If TX was already enabled and the transfer is finished, stop enabling it. Otherwise, we can
    // become stuck in an interrupt loop. In any other case, enable it. I am not fully sure
    // why this is necessary and why we can not stop interrupts as soon as we have the full
    // TX progress, but tests with ADCs have shown that not doing this causes timeouts.
    let enable_tx = !(enabled_irqs.tx() && context.tx_progress == transfer_len);

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

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TransferType {
    Read,
    Write,
    Transfer,
    TransferInPlace,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct TransferContext {
    transfer_type: Option<TransferType>,
    tx_progress: usize,
    rx_progress: usize,
    tx_slice: RawBufSlice,
    rx_slice: RawBufSliceMut,
    rx_overrun: bool,
}

#[allow(clippy::new_without_default)]
impl TransferContext {
    pub const fn new() -> Self {
        Self {
            transfer_type: None,
            tx_progress: 0,
            rx_progress: 0,
            tx_slice: RawBufSlice::new_nulled(),
            rx_slice: RawBufSliceMut::new_nulled(),
            rx_overrun: false,
        }
    }
}

pub struct SpiFuture<'spi> {
    bank: super::Bank,
    spi: &'spi mut super::Spi<u8>,
    finished_regularly: core::cell::Cell<bool>,
}

impl<'spi> SpiFuture<'spi> {
    fn new_for_read(spi: &'spi mut super::Spi<u8>, bank: super::Bank, words: &mut [u8]) -> Self {
        if words.is_empty() {
            panic!("words length unexpectedly 0");
        }
        Self::generic_init_transfer(spi, bank);

        let write_index = core::cmp::min(super::FIFO_DEPTH, words.len());
        // Send dummy bytes.
        (0..write_index).for_each(|_| {
            spi.regs.write_data(Data::new_with_raw_value(0));
        });

        Self::set_triggers(spi, write_index, words.len());

        critical_section::with(|cs| {
            let context_ref = TRANSFER_CONTEXTS[bank as usize].borrow(cs);
            let mut context = context_ref.borrow_mut();
            context.transfer_type = Some(TransferType::Read);
            unsafe {
                context.rx_slice.set(words);
            }
            context.tx_slice.set_null();
            context.tx_progress = write_index;
            context.rx_progress = 0;
            spi.regs.write_interrupt_clear(InterruptClear::ALL);
            spi.regs.write_interrupt_control(
                InterruptControl::ENABLE_ALL.with_tx(words.len() > FIFO_DEPTH),
            );
            spi.regs.modify_ctrl1(|v| v.with_mtxpause(false));
        });
        Self {
            bank,
            spi,
            finished_regularly: core::cell::Cell::new(false),
        }
    }

    fn new_for_write(spi: &'spi mut super::Spi<u8>, bank: super::Bank, words: &[u8]) -> Self {
        if words.is_empty() {
            panic!("words length unexpectedly 0");
        }
        let index = bank as usize;
        let write_index = Self::generic_init_transfer_write_transfer_in_place(spi, bank, words);
        critical_section::with(|cs| {
            let context_ref = TRANSFER_CONTEXTS[index].borrow(cs);
            let mut context = context_ref.borrow_mut();
            context.transfer_type = Some(TransferType::Write);
            unsafe {
                context.tx_slice.set(words);
            }
            context.rx_slice.set_null();
            context.tx_progress = write_index;
            context.rx_progress = 0;
            spi.regs.write_interrupt_clear(InterruptClear::ALL);
            spi.regs.write_interrupt_control(
                InterruptControl::ENABLE_ALL.with_tx(words.len() > FIFO_DEPTH),
            );
            spi.regs.modify_ctrl1(|v| v.with_mtxpause(false));
        });
        Self {
            bank,
            spi,
            finished_regularly: core::cell::Cell::new(false),
        }
    }

    fn new_for_transfer(
        spi: &'spi mut super::Spi<u8>,
        bank: super::Bank,
        read: &mut [u8],
        write: &[u8],
    ) -> Self {
        if read.is_empty() || write.is_empty() {
            panic!("read or write buffer unexpectedly empty");
        }
        let index = bank as usize;
        let full_write_len = core::cmp::max(read.len(), write.len());
        let fifo_prefill = core::cmp::min(
            core::cmp::min(super::FIFO_DEPTH, full_write_len),
            write.len(),
        );

        let write_idx = Self::generic_init_transfer_write_transfer_in_place(spi, bank, write);

        Self::generic_init_transfer(spi, bank);

        for write_index in 0..fifo_prefill {
            let value = write.get(write_index).copied().unwrap_or(0);
            spi.regs.write_data(Data::new_with_raw_value(value as u32));
        }

        Self::set_triggers(spi, fifo_prefill, full_write_len);

        critical_section::with(|cs| {
            let context_ref = TRANSFER_CONTEXTS[index].borrow(cs);
            let mut context = context_ref.borrow_mut();
            context.transfer_type = Some(TransferType::Transfer);
            unsafe {
                context.tx_slice.set(write);
                context.rx_slice.set(read);
            }
            context.tx_progress = write_idx;
            context.rx_progress = 0;
            spi.regs.write_interrupt_clear(InterruptClear::ALL);
            spi.regs.write_interrupt_control(
                InterruptControl::ENABLE_ALL.with_tx(full_write_len > FIFO_DEPTH),
            );
            spi.regs.modify_ctrl1(|v| v.with_mtxpause(false));
        });
        Self {
            bank,
            spi,
            finished_regularly: core::cell::Cell::new(false),
        }
    }

    fn new_for_transfer_in_place(
        spi: &'spi mut super::Spi<u8>,
        bank: super::Bank,
        words: &mut [u8],
    ) -> Self {
        if words.is_empty() {
            panic!("read and write buffer unexpectedly empty");
        }
        let write_idx = Self::generic_init_transfer_write_transfer_in_place(spi, bank, words);
        critical_section::with(|cs| {
            let context_ref = TRANSFER_CONTEXTS[bank as usize].borrow(cs);
            let mut context = context_ref.borrow_mut();
            context.transfer_type = Some(TransferType::TransferInPlace);
            unsafe {
                context.rx_slice.set(words);
            }
            context.tx_slice.set_null();
            context.tx_progress = write_idx;
            context.rx_progress = 0;
            spi.regs.write_interrupt_clear(InterruptClear::ALL);
            spi.regs.write_interrupt_control(
                InterruptControl::ENABLE_ALL.with_tx(words.len() > FIFO_DEPTH),
            );
            spi.regs.modify_ctrl1(|v| v.with_mtxpause(false));
        });
        Self {
            bank,
            spi,
            finished_regularly: core::cell::Cell::new(false),
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
                .write_data(Data::new_with_raw_value(write[idx] as u32));
        });

        Self::set_triggers(spi, write_idx, write.len());
        write_idx
    }

    fn set_triggers(spi: &mut super::Spi<u8>, write_idx: usize, write_len: usize) {
        // This should never fail because it is never larger than the FIFO depth.
        spi.regs
            .write_rx_fifo_trigger(TriggerLevel::new(u5::new(write_idx as u8)));
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

impl<'spi> Future for SpiFuture<'spi> {
    type Output = Result<(), RxOverrunError>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        WAKERS[self.bank as usize].register(cx.waker());
        if DONE[self.bank as usize].swap(false, core::sync::atomic::Ordering::Relaxed) {
            let rx_overrun = critical_section::with(|cs| {
                let mut ctx = TRANSFER_CONTEXTS[self.bank as usize]
                    .borrow(cs)
                    .borrow_mut();
                let overrun = ctx.rx_overrun;
                *ctx = TransferContext::default();
                overrun
            });
            self.finished_regularly.set(true);
            if rx_overrun {
                return core::task::Poll::Ready(Err(RxOverrunError));
            }
            return core::task::Poll::Ready(Ok(()));
        }
        core::task::Poll::Pending
    }
}

impl<'spi> Drop for SpiFuture<'spi> {
    fn drop(&mut self) {
        if !self.finished_regularly.get() {
            // It might be sufficient to disable and enable the SPI.. But this definitely
            // ensures the SPI is fully reset.
            self.spi.regs.write_interrupt_clear(InterruptClear::ALL);
            self.spi
                .regs
                .write_interrupt_control(InterruptControl::DISABLE_ALL);
            self.spi.regs.write_fifo_clear(FifoClear::ALL);
        }
    }
}

/// Asynchronous SPI driver.
///
/// This is the primary data structure used to perform non-blocking SPI operations.
/// It implements the [embedded_hal_async::spi::SpiBus] as well.
pub struct SpiAsync(pub super::Spi<u8>);

impl SpiAsync {
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
        // Disable blockmode for asynchronous mode.
        spi.regs
            .modify_ctrl1(|v| v.with_bm_stall(false).with_blockmode(false));
        Self(spi)
    }

    fn read(&mut self, words: &mut [u8]) -> Option<SpiFuture<'_>> {
        if words.is_empty() {
            return None;
        }
        let id = self.0.id;
        Some(SpiFuture::new_for_read(&mut self.0, id, words))
    }

    fn write(&mut self, words: &[u8]) -> Option<SpiFuture<'_>> {
        if words.is_empty() {
            return None;
        }
        let id = self.0.id;
        Some(SpiFuture::new_for_write(&mut self.0, id, words))
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Option<SpiFuture<'_>> {
        if read.is_empty() || write.is_empty() {
            return None;
        }
        let id = self.0.id;
        Some(SpiFuture::new_for_transfer(&mut self.0, id, read, write))
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Option<SpiFuture<'_>> {
        if words.is_empty() {
            return None;
        }
        let id = self.0.id;
        Some(SpiFuture::new_for_transfer_in_place(&mut self.0, id, words))
    }
}

impl embedded_hal_async::spi::ErrorType for SpiAsync {
    type Error = RxOverrunError;
}

impl embedded_hal_async::spi::SpiBus for SpiAsync {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        if words.is_empty() {
            return Ok(());
        }
        self.read(words).unwrap().await
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        if words.is_empty() {
            return Ok(());
        }
        self.write(words).unwrap().await
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        if read.is_empty() && write.is_empty() {
            return Ok(());
        }
        self.transfer(read, write).unwrap().await
    }

    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        if words.is_empty() {
            return Ok(());
        }
        self.transfer_in_place(words).unwrap().await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
