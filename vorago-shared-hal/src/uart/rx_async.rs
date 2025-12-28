//! # Async UART reception functionality.
//!
//! This module provides the [RxAsync] and [RxAsyncOverwriting] struct which both implement the
//! [embedded_io_async::Read] trait.
//! This trait allows for asynchronous reception of data streams. Please note that this module does
//! not specify/declare the interrupt handlers which must be provided for async support to work.
//! However, it provides two interrupt handlers:
//!
//! - [on_interrupt_rx]
//! - [on_interrupt_rx_overwriting]
//!
//! The first two are used for the [RxAsync] struct, while the latter two are used with the
//! [RxAsyncOverwriting] struct. The later two will overwrite old values in the used ring buffer.
//!
//! Error handling is performed in the user interrupt handler by checking the [AsyncUartErrors]
//! structure returned by the interrupt handlers.
use core::{cell::RefCell, convert::Infallible, future::Future, sync::atomic::Ordering};

use arbitrary_int::prelude::*;
use critical_section::Mutex;
use embassy_sync::waitqueue::AtomicWaker;
use embedded_io::ErrorType;
use portable_atomic::AtomicBool;

use super::{
    Bank, Rx, UartErrors,
    regs::{InterruptClear, MmioUart},
};

static UART_RX_WAKERS: [AtomicWaker; 2] = [const { AtomicWaker::new() }; 2];
static RX_READ_ACTIVE: [AtomicBool; 2] = [const { AtomicBool::new(false) }; 2];
static RX_HAS_DATA: [AtomicBool; 2] = [const { AtomicBool::new(false) }; 2];

struct RxFuture {
    id: Bank,
}

impl RxFuture {
    pub fn new(rx: &mut Rx) -> Self {
        RX_READ_ACTIVE[rx.id as usize].store(true, Ordering::Relaxed);
        Self { id: rx.id }
    }
}

impl Future for RxFuture {
    type Output = Result<(), Infallible>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        UART_RX_WAKERS[self.id as usize].register(cx.waker());
        if RX_HAS_DATA[self.id as usize].load(Ordering::Relaxed) {
            return core::task::Poll::Ready(Ok(()));
        }
        core::task::Poll::Pending
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AsyncUartErrors {
    /// Queue has overflowed, data might have been lost.
    pub queue_overflow: bool,
    /// UART errors.
    pub uart_errors: UartErrors,
}

fn on_interrupt_handle_rx_errors(uart: &mut MmioUart<'static>) -> Option<UartErrors> {
    let rx_status = uart.read_rx_status();
    if rx_status.overrun_error() || rx_status.framing_error() || rx_status.parity_error() {
        let mut errors_val = UartErrors::default();

        if rx_status.overrun_error() {
            errors_val.overflow = true;
        }
        if rx_status.framing_error() {
            errors_val.framing = true;
        }
        if rx_status.parity_error() {
            errors_val.parity = true;
        }
        return Some(errors_val);
    }
    None
}

fn on_interrupt_rx_common_post_processing(
    id: Bank,
    rx_enabled: bool,
    read_some_data: bool,
) -> Option<UartErrors> {
    let idx = id as usize;
    if read_some_data {
        RX_HAS_DATA[idx].store(true, Ordering::Relaxed);
        if RX_READ_ACTIVE[idx].load(Ordering::Relaxed) {
            UART_RX_WAKERS[idx].wake();
        }
    }

    let mut errors = None;
    let mut uart_regs = unsafe { id.steal_regs() };
    // Check for RX errors
    if rx_enabled {
        errors = on_interrupt_handle_rx_errors(&mut uart_regs);
    }

    // Clear the interrupt status bits
    uart_regs.write_irq_clr(
        InterruptClear::builder()
            .with_rx_overrun(true)
            .with_tx_overrun(false)
            .build(),
    );
    errors
}

/// Interrupt handler with overwriting behaviour when the ring buffer is full.
///
/// Should be called in the user interrupt handler to enable
/// asynchronous reception. This variant will overwrite old data in the ring buffer in case
/// the ring buffer is full.
pub fn on_interrupt_rx_overwriting(
    bank: Bank,
    prod: &mut heapless::spsc::Producer<u8>,
    shared_consumer: &Mutex<RefCell<Option<heapless::spsc::Consumer<'static, u8>>>>,
) -> Result<(), AsyncUartErrors> {
    on_interrupt_rx_async_heapless_queue_overwriting(bank, prod, shared_consumer)
}

pub fn on_interrupt_rx_async_heapless_queue_overwriting(
    bank: Bank,
    prod: &mut heapless::spsc::Producer<u8>,
    shared_consumer: &Mutex<RefCell<Option<heapless::spsc::Consumer<'static, u8>>>>,
) -> Result<(), AsyncUartErrors> {
    let uart_regs = unsafe { bank.steal_regs() };
    let irq_status = uart_regs.read_irq_status();
    let irq_enabled = uart_regs.read_irq_enabled();
    let rx_enabled = irq_enabled.rx();
    let mut read_some_data = false;
    let mut queue_overflow = false;

    // Half-Full interrupt. We have a guaranteed amount of data we can read.
    if irq_status.rx() {
        let available_bytes = uart_regs.read_rx_fifo_trigger().level().as_usize();

        // If this interrupt bit is set, the trigger level is available at the very least.
        // Read everything as fast as possible
        for _ in 0..available_bytes {
            let byte = uart_regs.read_data().value();
            if !prod.ready() {
                queue_overflow = true;
                critical_section::with(|cs| {
                    let mut cons_ref = shared_consumer.borrow(cs).borrow_mut();
                    cons_ref.as_mut().unwrap().dequeue();
                });
            }
            prod.enqueue(byte).ok();
        }
        read_some_data = true;
    }

    // Timeout, empty the FIFO completely.
    if irq_status.rx_timeout() {
        while uart_regs.read_rx_status().data_available() {
            // While there is data in the FIFO, write it into the reception buffer
            let byte = uart_regs.read_data().value();
            if !prod.ready() {
                queue_overflow = true;
                critical_section::with(|cs| {
                    let mut cons_ref = shared_consumer.borrow(cs).borrow_mut();
                    cons_ref.as_mut().unwrap().dequeue();
                });
            }
            prod.enqueue(byte).ok();
        }
        read_some_data = true;
    }

    let uart_errors = on_interrupt_rx_common_post_processing(bank, rx_enabled, read_some_data);
    if uart_errors.is_some() || queue_overflow {
        return Err(AsyncUartErrors {
            queue_overflow,
            uart_errors: uart_errors.unwrap_or_default(),
        });
    }
    Ok(())
}

/// Interrupt handler for asynchronous RX operations.
///
/// Should be called in the user interrupt handler to enable asynchronous reception.
pub fn on_interrupt_rx(
    bank: Bank,
    prod: &mut heapless::spsc::Producer<'_, u8>,
) -> Result<(), AsyncUartErrors> {
    on_interrupt_rx_async_heapless_queue(bank, prod)
}

pub fn on_interrupt_rx_async_heapless_queue(
    bank: Bank,
    prod: &mut heapless::spsc::Producer<'_, u8>,
) -> Result<(), AsyncUartErrors> {
    let uart_regs = unsafe { bank.steal_regs() };
    let irq_status = uart_regs.read_irq_status();
    let irq_enabled = uart_regs.read_irq_enabled();
    let rx_enabled = irq_enabled.rx();
    let mut read_some_data = false;
    let mut queue_overflow = false;

    // Half-Full interrupt. We have a guaranteed amount of data we can read.
    if irq_status.rx() {
        let available_bytes = uart_regs.read_rx_fifo_trigger().level().as_usize();

        // If this interrupt bit is set, the trigger level is available at the very least.
        // Read everything as fast as possible
        for _ in 0..available_bytes {
            let byte = uart_regs.read_data().value();
            if !prod.ready() {
                queue_overflow = true;
            }
            prod.enqueue(byte).ok();
        }
        read_some_data = true;
    }

    // Timeout, empty the FIFO completely.
    if irq_status.rx_timeout() {
        while uart_regs.read_rx_status().data_available() {
            // While there is data in the FIFO, write it into the reception buffer
            let byte = uart_regs.read_data().value();
            if !prod.ready() {
                queue_overflow = true;
            }
            prod.enqueue(byte).ok();
        }
        read_some_data = true;
    }

    let uart_errors = on_interrupt_rx_common_post_processing(bank, rx_enabled, read_some_data);
    if uart_errors.is_some() || queue_overflow {
        return Err(AsyncUartErrors {
            queue_overflow,
            uart_errors: uart_errors.unwrap_or_default(),
        });
    }
    Ok(())
}

struct ActiveReadGuard(usize);

impl Drop for ActiveReadGuard {
    fn drop(&mut self) {
        RX_READ_ACTIVE[self.0].store(false, Ordering::Relaxed);
    }
}

struct RxAsyncInner {
    rx: Rx,
    pub queue: heapless::spsc::Consumer<'static, u8>,
}

/// Core data structure to allow asynchronous UART reception.
///
/// If the ring buffer becomes full, data will be lost.
pub struct RxAsync(Option<RxAsyncInner>);

impl ErrorType for RxAsync {
    /// Error reporting is done using the result of the interrupt functions.
    type Error = Infallible;
}

fn stop_async_rx(rx: &mut Rx) {
    rx.disable_interrupts();
    rx.disable();
    rx.clear_fifo();
}

impl RxAsync {
    /// Create a new asynchronous receiver.
    ///
    /// The passed [heapless::spsc::Consumer] will be used to asynchronously receive data which
    /// is filled by the interrupt handler [on_interrupt_rx].
    pub fn new(mut rx: Rx, queue: heapless::spsc::Consumer<'static, u8>) -> Self {
        rx.disable_interrupts();
        rx.disable();
        rx.clear_fifo();
        // Enable those together.
        critical_section::with(|_| {
            #[cfg(feature = "vor1x")]
            rx.enable_interrupts(true);
            #[cfg(feature = "vor4x")]
            rx.enable_interrupts(true, true);
            rx.enable();
        });
        Self(Some(RxAsyncInner { rx, queue }))
    }

    pub fn stop(&mut self) {
        stop_async_rx(&mut self.0.as_mut().unwrap().rx);
    }

    pub fn release(mut self) -> (Rx, heapless::spsc::Consumer<'static, u8>) {
        self.stop();
        let inner = self.0.take().unwrap();
        (inner.rx, inner.queue)
    }
}

impl Drop for RxAsync {
    fn drop(&mut self) {
        self.stop();
    }
}

impl embedded_io_async::Read for RxAsync {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let inner = self.0.as_ref().unwrap();
        // Need to wait for the IRQ to read data and set this flag. If the queue is not
        // empty, we can read data immediately.
        if inner.queue.is_empty() {
            RX_HAS_DATA[inner.rx.id as usize].store(false, Ordering::Relaxed);
        }
        let _guard = ActiveReadGuard(inner.rx.id as usize);
        let mut handle_data_in_queue = |consumer: &mut heapless::spsc::Consumer<'static, u8>| {
            let data_to_read = consumer.len().min(buf.len());
            for byte in buf.iter_mut().take(data_to_read) {
                // We own the consumer and we checked that the amount of data is guaranteed to be available.
                *byte = unsafe { consumer.dequeue_unchecked() };
            }
            data_to_read
        };
        let mut_ref = self.0.as_mut().unwrap();
        let fut = RxFuture::new(&mut mut_ref.rx);
        // Data is available, so read that data immediately.
        let read_data = handle_data_in_queue(&mut mut_ref.queue);
        if read_data > 0 {
            return Ok(read_data);
        }
        // Await data.
        let _ = fut.await;
        Ok(handle_data_in_queue(&mut mut_ref.queue))
    }
}

struct RxAsyncOverwritingInner {
    rx: Rx,
    pub shared_consumer: &'static Mutex<RefCell<Option<heapless::spsc::Consumer<'static, u8>>>>,
}

/// Core data structure to allow asynchronous UART reception.
///
/// If the ring buffer becomes full, the oldest data will be overwritten when using the
/// [on_interrupt_rx_overwriting] interrupt handlers.
pub struct RxAsyncOverwriting(Option<RxAsyncOverwritingInner>);

impl ErrorType for RxAsyncOverwriting {
    /// Error reporting is done using the result of the interrupt functions.
    type Error = Infallible;
}

impl RxAsyncOverwriting {
    /// Create a new asynchronous receiver.
    ///
    /// The passed shared [heapless::spsc::Consumer] will be used to asynchronously receive data
    /// which is filled by the interrupt handler. The shared property allows using it in the
    /// interrupt handler to overwrite old data.
    pub fn new(
        mut rx: Rx,
        shared_consumer: &'static Mutex<RefCell<Option<heapless::spsc::Consumer<'static, u8>>>>,
    ) -> Self {
        rx.disable_interrupts();
        rx.disable();
        rx.clear_fifo();
        // Enable those together.
        critical_section::with(|_| {
            #[cfg(feature = "vor4x")]
            rx.enable_interrupts(true, true);
            #[cfg(feature = "vor1x")]
            rx.enable_interrupts(true);
            rx.enable();
        });
        Self(Some(RxAsyncOverwritingInner {
            rx,
            shared_consumer,
        }))
    }

    pub fn stop(&mut self) {
        stop_async_rx(&mut self.0.as_mut().unwrap().rx);
    }

    pub fn release(mut self) -> Rx {
        self.stop();
        let inner = self.0.take().unwrap();
        inner.rx
    }
}

impl Drop for RxAsyncOverwriting {
    fn drop(&mut self) {
        self.stop();
    }
}

impl embedded_io_async::Read for RxAsyncOverwriting {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let inner = self.0.as_ref().unwrap();
        let id = inner.rx.id as usize;
        // Need to wait for the IRQ to read data and set this flag. If the queue is not
        // empty, we can read data immediately.

        critical_section::with(|cs| {
            let queue = inner.shared_consumer.borrow(cs);
            if queue.borrow().as_ref().unwrap().is_empty() {
                RX_HAS_DATA[id].store(false, Ordering::Relaxed);
            }
        });
        let _guard = ActiveReadGuard(id);
        let mut handle_data_in_queue = |inner: &mut RxAsyncOverwritingInner| {
            critical_section::with(|cs| {
                let mut consumer_ref = inner.shared_consumer.borrow(cs).borrow_mut();
                let consumer = consumer_ref.as_mut().unwrap();
                let data_to_read = consumer.len().min(buf.len());
                for byte in buf.iter_mut().take(data_to_read) {
                    // We own the consumer and we checked that the amount of data is guaranteed to be available.
                    *byte = unsafe { consumer.dequeue_unchecked() };
                }
                data_to_read
            })
        };
        let fut = RxFuture::new(&mut self.0.as_mut().unwrap().rx);
        // Data is available, so read that data immediately.
        let read_data = handle_data_in_queue(self.0.as_mut().unwrap());
        if read_data > 0 {
            return Ok(read_data);
        }
        // Await data.
        let _ = fut.await;
        let read_data = handle_data_in_queue(self.0.as_mut().unwrap());
        Ok(read_data)
    }
}
