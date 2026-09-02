use core::cell::Cell;

use crate::{
    i2c::{
        FIFO_DEPTH, I2cAddress,
        regs::{self, Command, Data, InterruptClear},
    },
    shared::asynch::TransferState,
};
use arbitrary_int::u11;
use portable_atomic::{AtomicU8, Ordering};

#[cfg(feature = "vor1x")]
use crate::InterruptConfig;

/// Number of I2C peripherals.
#[cfg(feature = "vor1x")]
pub const NUM_I2C: usize = 2;
/// Number of I2C peripherals.
#[cfg(feature = "vor4x")]
pub const NUM_I2C: usize = 3;

static TRANSFER_CONTEXTS: [TransferContext; NUM_I2C] = [const { TransferContext::new() }; NUM_I2C];

/// Kind of transfer an async [Transfer] is performing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TransferType {
    /// A read transfer.
    Read = 0,
    /// A write transfer.
    Write = 1,
    /// A simultaneous read and write transfer, still in the write phase.
    WriteRead = 2,
    /// A simultaneous read and write transfer, in the read phase.
    ///
    /// `waiting` is not clearable and stays set for the remainder of the transfer once
    /// observed, so it cannot be used to tell whether the read phase's `Start`/`Stop` was
    /// already issued. This variant is the gate's own record of that instead: the interrupt
    /// handler transitions [TransferType::WriteRead] to this exactly once, the first time
    /// `waiting` is observed, so the read phase is only ever kicked off a single time.
    WriteReadReceiving = 3,
}

impl TransferType {
    /// Stored in [TransferContext::transfer_type] while no transfer is active.
    const NONE: u8 = 0xff;

    const fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            0 => Some(Self::Read),
            1 => Some(Self::Write),
            2 => Some(Self::WriteRead),
            3 => Some(Self::WriteReadReceiving),
            _ => None,
        }
    }
}

/// Error condition observed by the interrupt handler, stored in [TransferContext::error].
///
/// Mirrors [super::Error], minus the payload on [super::Error::ClockTimeout]: `num_enum`'s
/// conversion derives only support field-less, C-like enums, and the timeout limit is cheap to
/// re-read from the register at the point [TransferContext::take_error] reconstructs the full
/// error, so there is no need to carry it through the atomic.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
#[repr(u8)]
enum TransferErrorKind {
    /// No error occurred. The value [TransferContext::error] is reset to.
    None = 0,
    /// Arbitration was lost.
    ArbitrationLost = 1,
    /// Address not acknowledged.
    NackAddr = 2,
    /// Data not acknowledged in write operation.
    NackData = 3,
    /// The I2C clock was seen low for longer than the configured timeout.
    ClockTimeout = 4,
    /// The RX or TX FIFO overflowed.
    Overflow = 5,
}

/// Transfer context structure.
///
/// `transfer_type` doubles as the "transfer active" flag: it is always published last
/// (`Release`) after the buffers and progress counters in [TransferState], and read first
/// (`Acquire`) before them. A reader which observes an active transfer type is therefore
/// guaranteed to see the matching buffers and counters, rather than stale ones from a previous
/// transfer.
struct TransferContext {
    transfer_type: AtomicU8,
    state: TransferState,
    /// Set by the interrupt handler on an error condition, consumed by `take_error`.
    error: AtomicU8,
}

impl TransferContext {
    const fn new() -> Self {
        Self {
            transfer_type: AtomicU8::new(TransferType::NONE),
            state: TransferState::new(),
            error: AtomicU8::new(TransferErrorKind::None as u8),
        }
    }

    /// Records an error condition, to be observed by `take_error`.
    #[inline]
    fn set_error(&self, kind: TransferErrorKind) {
        self.error.store(kind.into(), Ordering::Relaxed);
    }

    /// Takes (and clears) the recorded error condition, if any.
    ///
    /// For [TransferErrorKind::ClockTimeout], the caller must still read the timeout limit
    /// itself to build a full [super::Error::ClockTimeout] value.
    #[inline]
    fn take_error(&self) -> Option<TransferErrorKind> {
        let raw = self
            .error
            .swap(TransferErrorKind::None as u8, Ordering::Relaxed);
        match TransferErrorKind::try_from(raw) {
            Ok(TransferErrorKind::None) | Err(_) => None,
            Ok(kind) => Some(kind),
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

    /// Closes the gate and restores the initial state, so the slot can be reused.
    #[inline]
    fn reset(&self) {
        // Must come first in program order: a live interrupt (e.g. during Drop's
        // cancellation path) must never see the fields below cleared while still
        // observing an armed transfer.
        self.disarm();
        self.state.reset();
        self.error
            .store(TransferErrorKind::None as u8, Ordering::Relaxed);
    }
}

/// Async I2C driver, built on top of the blocking [I2cMaster](super::I2cMaster).
pub struct I2c(super::I2cMaster);

impl I2c {
    /// Construct an asynchronous I2C driver for the given I2C peripheral.
    ///
    /// # Safety
    ///
    /// The user MUST ensure that the `Drop` method of all futures generated with this driver
    /// is called on transfer cancellation. By default, this does not require any special
    /// handling. This case was considered exotic enough to justify not making the function
    /// `unsafe`.
    pub fn new(
        mut i2c: super::I2cMaster,
        #[cfg(feature = "vor1x")] opt_irq_cfg: Option<InterruptConfig>,
    ) -> Self {
        i2c.regs
            .write_interrupt_enable(regs::InterruptControl::ZERO);
        i2c.regs.write_interrupt_clear(InterruptClear::ALL);
        #[cfg(feature = "vor1x")]
        if let Some(irq_cfg) = opt_irq_cfg {
            if irq_cfg.route {
                crate::enable_peripheral_clock(crate::PeripheralSelect::Irqsel);
                unsafe { va108xx::Irqsel::steal() }
                    .i2c_ms(i2c.id as usize)
                    .write(|w| unsafe { w.bits(irq_cfg.id as u32) });
            }
            if irq_cfg.enable_in_nvic {
                // Safety: User has specifically configured this.
                unsafe { crate::enable_nvic_interrupt(irq_cfg.id) };
            }
        }
        // Unlike vor1x, vor4x has a fixed interrupt vector per bank instead of an IRQSEL mux,
        // so there is no routing decision to make: always enable it.
        #[cfg(feature = "vor4x")]
        unsafe {
            crate::enable_nvic_interrupt(i2c.id.interrupt_id_master());
        }
        Self(i2c)
    }

    /// Interrupt handler for the given I2C bank.
    ///
    /// Call this from the bank's interrupt vector. Does nothing if no async transfer is active.
    ///
    /// Returns the live status observed on this call. The driver only treats clock timeouts,
    /// arbitration loss, NACKs and FIFO overflows as transfer errors. Other bits, like
    /// `stalled`, are not currently surfaced as an [super::Error] variant: read them from the
    /// returned value if you need to observe them.
    pub fn on_interrupt(bank_id: super::Bank) -> regs::Status {
        let mut regs = unsafe { bank_id.steal_regs() };
        let context = &TRANSFER_CONTEXTS[bank_id as usize];

        // Use live status register.
        let interrupt_status = regs.read_interrupt_status();
        // Clear all interrupts.
        regs.write_interrupt_clear(InterruptClear::ALL);
        let status = regs.read_status();

        let Some(transfer_type) = context.active_transfer_type() else {
            // Disable interrupts if there is no active transfer to avoid an interrupt loop.
            regs.write_interrupt_enable(regs::InterruptControl::new_with_raw_value(0));
            return status;
        };

        let mut common_error_handling = |error: TransferErrorKind| {
            context.set_error(error);
            regs.write_interrupt_enable(regs::InterruptControl::ZERO);
            // Ends the transaction on the bus. Necessary because a `WriteRead`'s write phase
            // uses a plain `Start`, which does not self-terminate the way `StartWithStop` does.
            regs.write_command(
                Command::builder()
                    .with_start(false)
                    .with_stop(false)
                    .with_cancel(true)
                    .build(),
            );
            context.state.signal_done();
        };
        if interrupt_status.clock_timeout() {
            common_error_handling(TransferErrorKind::ClockTimeout);
            return status;
        }
        if status.arb_lost() {
            common_error_handling(TransferErrorKind::ArbitrationLost);
            return status;
        }
        if status.nack_addr() {
            common_error_handling(TransferErrorKind::NackAddr);
            return status;
        }
        // `nack_data` only indicates a real failure while transmitting: a slave NACKing a byte
        // we wrote. On a receive, the *master* NACKs the last byte itself to end the transfer,
        // which is normal termination, not a slave failure, but sets the same bit. The blocking
        // driver makes the same distinction: `write_blocking_generic` checks this, `read_blocking`
        // does not.
        let nack_data_is_error = match transfer_type {
            TransferType::Read => false,
            TransferType::Write => true,
            // If `waiting` is observed on this same read, the write phase already succeeded (a
            // real write NACK would have prevented it from ever firing) — even if this is the
            // very first interrupt for the whole transfer, so `transfer_type` has not been
            // updated to `WriteReadReceiving` yet. The write and read phases can complete close
            // enough together that both show up in the same interrupt entry.
            TransferType::WriteRead => !status.waiting(),
            TransferType::WriteReadReceiving => false,
        };
        if nack_data_is_error && status.nack_data() {
            common_error_handling(TransferErrorKind::NackData);
            return status;
        }
        if interrupt_status.rx_overflow() || interrupt_status.tx_overflow() {
            common_error_handling(TransferErrorKind::Overflow);
            return status;
        }

        match transfer_type {
            TransferType::Read => {
                Self::on_interrupt_read(&mut regs, context);
            }
            TransferType::Write => {
                Self::on_interrupt_write(&mut regs, context);
            }
            TransferType::WriteRead => {
                Self::on_interrupt_write_read(&mut regs, context, status);
            }
            TransferType::WriteReadReceiving => {
                Self::on_interrupt_write_read_receiving(&mut regs, context);
            }
        }
        status
    }

    fn on_interrupt_read(regs: &mut regs::MmioRegisters<'static>, context: &TransferContext) {
        let mut progress = context.state.rx_progress.load(Ordering::Relaxed);
        let slice = unsafe { context.state.rx_slice() };
        while regs.read_status().rx_not_empty() && progress < slice.len() {
            slice[progress] = regs.read_data().data();
            progress += 1;
        }
        context.state.rx_progress.store(progress, Ordering::Relaxed);
        // Checked after draining, not before: otherwise a byte that arrived and was
        // drained on this same interrupt would need a whole extra interrupt just to
        // notice `progress` is now complete.
        if progress >= slice.len() && regs.read_status().idle() {
            context.state.signal_done();
            regs.write_interrupt_enable(regs::InterruptControl::ZERO);
        }
    }

    fn on_interrupt_write(regs: &mut regs::MmioRegisters<'static>, context: &TransferContext) {
        let mut progress = context.state.tx_progress.load(Ordering::Relaxed);
        let slice = unsafe { context.state.tx_slice() };
        if progress >= slice.len() {
            context.state.signal_done();
            regs.write_interrupt_enable(regs::InterruptControl::ZERO);
            return;
        }
        while regs.read_status().tx_not_full() && progress < slice.len() {
            regs.write_data(Data::new(slice[progress]));
            progress += 1;
        }
        context.state.tx_progress.store(progress, Ordering::Relaxed);
    }

    fn on_interrupt_write_read(
        regs: &mut regs::MmioRegisters<'static>,
        context: &TransferContext,
        status: regs::Status,
    ) {
        // Still in the write phase.
        let mut tx_progress = context.state.tx_progress.load(Ordering::Relaxed);
        let tx_slice = unsafe { context.state.tx_slice() };
        while regs.read_status().tx_not_full() && tx_progress < tx_slice.len() {
            regs.write_data(Data::new(tx_slice[tx_progress]));
            tx_progress += 1;
        }
        context
            .state
            .tx_progress
            .store(tx_progress, Ordering::Relaxed);

        if status.waiting() {
            // Write phase finished, so we need to set up the reception transfer.
            // `waiting` is not clearable and stays set for the rest of the transfer, so
            // transitioning to `WriteReadReceiving` here is what stops this from
            // re-issuing the read phase's `Start`/`Stop` on every later interrupt.
            let rx_slice = unsafe { context.state.rx_slice() };
            regs.write_words(regs::Words::new(u11::new(rx_slice.len() as u16)));
            regs.modify_address(|val| val.with_direction(regs::Direction::Receive));
            regs.write_command(
                Command::builder()
                    .with_start(true)
                    .with_stop(true)
                    .with_cancel(false)
                    .build(),
            );
            context.arm(TransferType::WriteReadReceiving);
        }
    }

    fn on_interrupt_write_read_receiving(
        regs: &mut regs::MmioRegisters<'static>,
        context: &TransferContext,
    ) {
        let mut progress = context.state.rx_progress.load(Ordering::Relaxed);
        let slice = unsafe { context.state.rx_slice() };
        while regs.read_status().rx_not_empty() && progress < slice.len() {
            slice[progress] = regs.read_data().data();
            progress += 1;
        }
        context.state.rx_progress.store(progress, Ordering::Relaxed);
        // Checked after draining, not before: otherwise a byte that arrived and was
        // drained on this same interrupt would need a whole extra interrupt just to
        // notice `progress` is now complete.
        if progress >= slice.len() && regs.read_status().idle() {
            context.state.signal_done();
            regs.write_interrupt_enable(regs::InterruptControl::ZERO);
        }
    }

    /// Start an async read transaction, returning a future which completes once `buf` was
    /// filled.
    pub fn read(&mut self, address: u8, buf: &mut [u8]) -> Result<Transfer<'_>, super::Error> {
        let len = buf.len();
        if len > 0x7fe {
            return Err(super::Error::DataTooLarge);
        }
        let bank = self.0.id;
        self.0.clear_rx_fifo();
        // Load number of words
        self.0
            .regs
            .write_words(regs::Words::new(u11::new(len as u16)));

        let context = &TRANSFER_CONTEXTS[bank as usize];
        // Safety contract is documented in top-level driver: Users are not alllowed to forget
        // I2C transfers.
        unsafe {
            context.state.set_rx_slice(buf);
        }
        context.arm(TransferType::Read);

        self.0.regs.write_interrupt_clear(InterruptClear::ALL);
        // Only issue the address and command after arming the gate but before enabling
        // interrupts: enabling `idle`/`waiting` beforehand would let a spurious interrupt for
        // the still-genuinely-idle bus fall into the armed match arm below, find nothing to do,
        // and never disable interrupts again, since nothing about the real transaction has
        // started yet to ever make the completion condition true.
        self.0
            .write_address(I2cAddress::Regular(address), regs::Direction::Send);
        self.0.write_command(super::I2cCommand::StartWithStop);

        self.0.regs.write_interrupt_enable(
            regs::InterruptControl::builder()
                // Error conditions.
                .with_clock_timeout(true)
                .with_rx_overflow(true)
                .with_tx_overflow(false)
                .with_arb_lost(true)
                .with_nack_addr(true)
                .with_nack_data(true)
                // FIFO drain condition.
                .with_rx_ready(len > FIFO_DEPTH)
                .with_tx_ready(false)
                // Done status.
                .with_idle(true)
                // Explicitely set to false, only required for write-read transactions.
                .with_waiting(false)
                // Users might be interested in getting informed about stall conditions.
                .with_stalled(true)
                // Unused.
                .with_i2c_idle(false)
                .with_tx_empty(false)
                .with_rx_full(false)
                .build(),
        );

        Ok(Transfer {
            driver: self,
            finished_regularly: core::cell::Cell::new(false),
        })
    }

    /// Start an async write transaction, returning a future which completes once `data` was
    /// sent.
    pub fn write(&mut self, address: u8, data: &[u8]) -> Result<Transfer<'_>, super::Error> {
        let len = data.len();
        if len > 0x7fe {
            return Err(super::Error::DataTooLarge);
        }
        let bank = self.0.id;
        self.0.clear_tx_fifo();
        // Load number of words
        self.0
            .regs
            .write_words(regs::Words::new(u11::new(len as u16)));
        let current_index = core::cmp::min(FIFO_DEPTH, len);
        let mut bytes = data.iter();
        // load the FIFO
        for _ in 0..current_index {
            self.0.write_fifo_unchecked(*bytes.next().unwrap());
        }

        let context = &TRANSFER_CONTEXTS[bank as usize];
        // Safety contract is documented in top-level driver: Users are not alllowed to forget
        // I2C transfers.
        unsafe {
            context.state.set_tx_slice(data);
        }
        context
            .state
            .tx_progress
            .store(current_index, Ordering::Relaxed);
        context.arm(TransferType::Write);

        self.0.regs.write_interrupt_clear(InterruptClear::ALL);
        // See the comment in `read` on why the address/command must be issued before
        // interrupts are enabled.
        self.0
            .write_address(I2cAddress::Regular(address), regs::Direction::Send);
        self.0.write_command(super::I2cCommand::StartWithStop);

        self.0.regs.write_interrupt_enable(
            regs::InterruptControl::builder()
                // Error conditions.
                .with_clock_timeout(true)
                .with_tx_overflow(true)
                .with_rx_overflow(false)
                .with_arb_lost(true)
                .with_nack_addr(true)
                .with_nack_data(true)
                // FIFO re-fill condition.
                .with_tx_ready(len > FIFO_DEPTH)
                .with_rx_ready(false)
                // Done status.
                .with_idle(true)
                // Explicitely set to false, only required for write-read transactions.
                .with_waiting(false)
                // Users might be interested in getting informed about stall conditions.
                .with_stalled(true)
                // Unused.
                .with_i2c_idle(false)
                .with_tx_empty(false)
                .with_rx_full(false)
                .build(),
        );

        Ok(Transfer {
            driver: self,
            finished_regularly: core::cell::Cell::new(false),
        })
    }

    /// Start an async write-then-read transaction, returning a future which completes once
    /// `write` was sent and `read` was filled.
    pub fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<Transfer<'_>, super::Error> {
        if write.len() > 0x7fe || read.len() > 0x7fe {
            return Err(super::Error::DataTooLarge);
        }
        let bank = self.0.id;
        self.0.clear_rx_fifo();
        self.0.clear_tx_fifo();
        let write_len = write.len();
        // Load number of words
        self.0
            .regs
            .write_words(regs::Words::new(u11::new(write_len as u16)));
        let mut bytes = write.iter();
        let current_index = core::cmp::min(FIFO_DEPTH, write.len());
        // load the FIFO
        for _ in 0..current_index {
            self.0.write_fifo_unchecked(*bytes.next().unwrap());
        }

        let context = &TRANSFER_CONTEXTS[bank as usize];
        // Safety contract is documented in top-level driver: Users are not alllowed to forget
        // I2C transfers.
        unsafe {
            context.state.set_tx_slice(write);
            context.state.set_rx_slice(read);
        }
        context
            .state
            .tx_progress
            .store(current_index, Ordering::Relaxed);
        context.arm(TransferType::WriteRead);

        self.0.regs.write_interrupt_clear(InterruptClear::ALL);
        // See the comment in `read` on why the address/command must be issued before
        // interrupts are enabled.
        self.0
            .write_address(I2cAddress::Regular(address), regs::Direction::Send);
        self.0.write_command(super::I2cCommand::Start);

        self.0.regs.write_interrupt_enable(
            regs::InterruptControl::builder()
                // Error conditions.
                .with_clock_timeout(true)
                .with_rx_overflow(true)
                .with_tx_overflow(true)
                .with_arb_lost(true)
                .with_nack_addr(true)
                .with_nack_data(true)
                // FIFO drain condition.
                .with_rx_ready(read.len() > FIFO_DEPTH)
                .with_tx_ready(write_len > FIFO_DEPTH)
                // Done status. Not needed here: `waiting` already signals the write phase's
                // completion, and the read phase's completion comes from the master's own
                // terminating NACK on the last byte (`nack_data`), so `idle` never has to be
                // enabled as an interrupt source for this transfer. The handler still checks
                // the live `status.idle()` bit itself in the completion condition.
                .with_idle(false)
                // Will be set when the write part is finished.
                .with_waiting(true)
                // Users might be interested in getting informed about stall conditions.
                .with_stalled(true)
                // Unused.
                .with_i2c_idle(false)
                .with_tx_empty(false)
                .with_rx_full(false)
                .build(),
        );

        Ok(Transfer {
            driver: self,
            finished_regularly: core::cell::Cell::new(false),
        })
    }
}

impl embedded_hal_async::i2c::I2c for I2c {
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        for operation in operations {
            match operation {
                embedded_hal::i2c::Operation::Read(buf) => {
                    self.read(address, buf)?.await?;
                }
                embedded_hal::i2c::Operation::Write(buf) => {
                    self.write(address, buf)?.await?;
                }
            }
        }
        Ok(())
    }

    #[inline]
    async fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.write_read(address, write, read)?.await?;
        Ok(())
    }
}

impl embedded_hal_async::i2c::ErrorType for I2c {
    type Error = super::Error;
}

/// Live I2C transfer returned by the async transfer methods on [I2c].
///
/// Implements [Future] and can be polled/awaited to completion.
pub struct Transfer<'a> {
    driver: &'a mut I2c,
    finished_regularly: Cell<bool>,
}

impl core::future::Future for Transfer<'_> {
    type Output = Result<(), super::Error>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let context = &TRANSFER_CONTEXTS[self.driver.0.id() as usize];
        if context.state.poll_done(cx.waker()) {
            self.finished_regularly.set(true);
            // Read the error before resetting: `reset` clears it too, so it must happen last.
            let error = context.take_error();
            context.reset();
            if let Some(error) = error {
                let transfer_error = match error {
                    TransferErrorKind::ArbitrationLost => super::Error::ArbitrationLost,
                    TransferErrorKind::NackAddr => super::Error::NackAddr,
                    TransferErrorKind::NackData => super::Error::NackData,
                    TransferErrorKind::ClockTimeout => super::Error::ClockTimeout(
                        self.driver.0.regs.read_clk_timeout_limit().value(),
                    ),
                    TransferErrorKind::Overflow => super::Error::Overflow,
                    _ => return core::task::Poll::Ready(Ok(())),
                };
                return core::task::Poll::Ready(Err(transfer_error));
            }
            return core::task::Poll::Ready(Ok(()));
        }
        core::task::Poll::Pending
    }
}

impl Drop for Transfer<'_> {
    fn drop(&mut self) {
        if !self.finished_regularly.get() {
            self.driver.0.disable_interrupts();
            let context = &TRANSFER_CONTEXTS[self.driver.0.id() as usize];
            context.reset();
            self.driver
                .0
                .regs
                .write_interrupt_clear(InterruptClear::ALL);
            self.driver.0.cancel_transfer();
            self.driver.0.clear_tx_fifo();
            self.driver.0.clear_rx_fifo();
        }
    }
}
