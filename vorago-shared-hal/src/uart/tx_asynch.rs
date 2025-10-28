//! # Async UART transmission functionality.
//!
//! This module provides the [TxAsync] struct which implements the [embedded_io_async::Write] trait.
//! This trait allows for asynchronous sending of data streams. Please note that this module does
//! not specify/declare the interrupt handlers which must be provided for async support to work.
//! However, it the [on_interrupt_tx] interrupt handler.
//!
//! This handler should be called in ALL user interrupt handlers which handle UART TX interrupts
//! for a given UART bank.
use core::{cell::RefCell, future::Future};

use critical_section::Mutex;
use embassy_sync::waitqueue::AtomicWaker;
use embedded_io_async::Write;
use portable_atomic::AtomicBool;
use raw_slice::RawBufSlice;

use super::*;

static UART_TX_WAKERS: [AtomicWaker; 2] = [const { AtomicWaker::new() }; 2];
static TX_CONTEXTS: [Mutex<RefCell<TxContext>>; 2] =
    [const { Mutex::new(RefCell::new(TxContext::new())) }; 2];
// Completion flag. Kept outside of the context structure as an atomic to avoid
// critical section.
static TX_DONE: [AtomicBool; 2] = [const { AtomicBool::new(false) }; 2];

/// This is a generic interrupt handler to handle asynchronous UART TX operations for a given
/// UART bank.
///
/// The user has to call this once in the interrupt handler responsible for the TX interrupts on
/// the given UART bank.
pub fn on_interrupt_tx(bank: Bank) {
    let mut uart = unsafe { bank.steal_regs() };
    let idx = bank as usize;
    let irq_enabled = uart.read_irq_enabled();
    // IRQ is not related to TX.
    if !irq_enabled.tx() && !irq_enabled.tx_empty() {
        return;
    }

    let tx_status = uart.read_tx_status();
    let unexpected_overrun = tx_status.wr_lost();
    let mut context = critical_section::with(|cs| {
        let context_ref = TX_CONTEXTS[idx].borrow(cs);
        *context_ref.borrow()
    });
    context.tx_overrun = unexpected_overrun;
    // Safety: We documented that the user provided slice must outlive the future, so we convert
    // the raw pointer back to the slice here.
    let slice = unsafe { context.slice.get().unwrap() };
    if context.progress >= slice.len() && !tx_status.tx_busy() {
        uart.modify_irq_enabled(|mut value| {
            value.set_tx(false);
            value.set_tx_empty(false);
            value.set_tx_status(false);
            value
        });
        uart.modify_enable(|mut value| {
            value.set_tx(false);
            value
        });
        // Write back updated context structure.
        critical_section::with(|cs| {
            let context_ref = TX_CONTEXTS[idx].borrow(cs);
            *context_ref.borrow_mut() = context;
        });
        // Transfer is done.
        TX_DONE[idx].store(true, core::sync::atomic::Ordering::Relaxed);
        UART_TX_WAKERS[idx].wake();
        return;
    }
    while context.progress < slice.len() {
        if !uart.read_tx_status().ready() {
            break;
        }
        // Safety: TX structure is owned by the future which does not write into the the data
        // register, so we can assume we are the only one writing to the data register.
        uart.write_data(Data::new_with_raw_value(slice[context.progress] as u32));
        context.progress += 1;
    }

    // Write back updated context structure.
    critical_section::with(|cs| {
        let context_ref = TX_CONTEXTS[idx].borrow(cs);
        *context_ref.borrow_mut() = context;
    });
}

#[derive(Debug, Copy, Clone)]
pub struct TxContext {
    progress: usize,
    tx_overrun: bool,
    slice: RawBufSlice,
}

#[allow(clippy::new_without_default)]
impl TxContext {
    pub const fn new() -> Self {
        Self {
            progress: 0,
            tx_overrun: false,
            slice: RawBufSlice::new_nulled(),
        }
    }
}

pub struct TxFuture {
    id: Bank,
}

impl TxFuture {
    /// # Safety
    ///
    /// This function stores the raw pointer of the passed data slice. The user MUST ensure
    /// that the slice outlives the data structure.
    pub unsafe fn new(tx: &mut Tx, data: &[u8]) -> Self {
        TX_DONE[tx.id as usize].store(false, core::sync::atomic::Ordering::Relaxed);
        tx.disable_interrupts();
        tx.disable();
        tx.clear_fifo();

        let init_fill_count = core::cmp::min(data.len(), 16);
        // We fill the FIFO.
        for data in data.iter().take(init_fill_count) {
            tx.regs.write_data(Data::new_with_raw_value(*data as u32));
        }
        critical_section::with(|cs| {
            let context_ref = TX_CONTEXTS[tx.id as usize].borrow(cs);
            let mut context = context_ref.borrow_mut();
            unsafe { context.slice.set(data) };
            context.progress = init_fill_count;

            // Ensure those are enabled inside a critical section at the same time. Can lead to
            // weird glitches otherwise.
            tx.enable_interrupts(
                #[cfg(feature = "vor4x")]
                true,
            );
            tx.enable();
        });
        Self { id: tx.id }
    }
}

impl Future for TxFuture {
    type Output = Result<usize, TxOverrunError>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        UART_TX_WAKERS[self.id as usize].register(cx.waker());
        if TX_DONE[self.id as usize].swap(false, core::sync::atomic::Ordering::Relaxed) {
            let progress = critical_section::with(|cs| {
                TX_CONTEXTS[self.id as usize].borrow(cs).borrow().progress
            });
            return core::task::Poll::Ready(Ok(progress));
        }
        core::task::Poll::Pending
    }
}

impl Drop for TxFuture {
    fn drop(&mut self) {
        let mut reg_block = unsafe { self.id.steal_regs() };

        disable_tx_interrupts(&mut reg_block);
        disable_tx(&mut reg_block);
    }
}

pub struct TxAsync(Tx);

impl TxAsync {
    pub fn new(tx: Tx) -> Self {
        Self(tx)
    }

    pub fn release(self) -> Tx {
        self.0
    }
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("TX overrun error")]
pub struct TxOverrunError;

impl embedded_io_async::Error for TxOverrunError {
    fn kind(&self) -> embedded_io_async::ErrorKind {
        embedded_io_async::ErrorKind::Other
    }
}

impl embedded_io::ErrorType for TxAsync {
    type Error = TxOverrunError;
}

impl Write for TxAsync {
    /// Write a buffer asynchronously.
    ///
    /// This implementation is not side effect free, and a started future might have already
    /// written part of the passed buffer.
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let fut = unsafe { TxFuture::new(&mut self.0, buf) };
        fut.await
    }
}
