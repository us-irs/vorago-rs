//! # Async GPIO functionality for the Vorago GPIO peripherals.
//!
//! This module provides the [InputPinAsync] which implements
//! the [embedded_hal_async::digital::Wait] trait. These types allow for asynchronous waiting
//! on GPIO pins. Please note that this module does not specify/declare the interrupt handlers
//! which must be provided for async support to work. However, it provides the
//! [on_interrupt_for_async_gpio_for_port] generic interrupt handler. This should be called in all
//! IRQ functions which handle any GPIO interrupts with the corresponding [Port] argument.
use core::future::Future;

use embassy_sync::waitqueue::AtomicWaker;
use embedded_hal_async::digital::Wait;
use portable_atomic::AtomicBool;

#[cfg(feature = "vor4x")]
use crate::NUM_PORT_DEFAULT;
#[cfg(feature = "vor1x")]
use crate::{InterruptConfig, NUM_PORT_A, NUM_PORT_B};

#[cfg(feature = "vor4x")]
use super::ll::PortDoesNotSupportInterrupts;

#[cfg(feature = "vor1x")]
use va108xx as pac;

pub use super::ll::InterruptEdge;
use super::{
    Input, Port,
    ll::{DynPinId, LowLevelGpio},
};

cfg_if::cfg_if! {
    if #[cfg(feature = "vor1x")] {
        static WAKERS_FOR_PORT_A: [AtomicWaker; NUM_PORT_A] = [const { AtomicWaker::new() }; NUM_PORT_A];
        static WAKERS_FOR_PORT_B: [AtomicWaker; NUM_PORT_B] = [const { AtomicWaker::new() }; NUM_PORT_B];
        static EDGE_DETECTION_PORT_A: [AtomicBool; NUM_PORT_A] =
            [const { AtomicBool::new(false) }; NUM_PORT_A];
        static EDGE_DETECTION_PORT_B: [AtomicBool; NUM_PORT_B] =
            [const { AtomicBool::new(false) }; NUM_PORT_B];
    } else {
        static WAKERS_FOR_PORT_A: [AtomicWaker; NUM_PORT_DEFAULT] =
            [const { AtomicWaker::new() }; NUM_PORT_DEFAULT];
        static WAKERS_FOR_PORT_B: [AtomicWaker; NUM_PORT_DEFAULT] =
            [const { AtomicWaker::new() }; NUM_PORT_DEFAULT];
        static WAKERS_FOR_PORT_C: [AtomicWaker; NUM_PORT_DEFAULT] =
            [const { AtomicWaker::new() }; NUM_PORT_DEFAULT];
        static WAKERS_FOR_PORT_D: [AtomicWaker; NUM_PORT_DEFAULT] =
            [const { AtomicWaker::new() }; NUM_PORT_DEFAULT];
        static WAKERS_FOR_PORT_E: [AtomicWaker; NUM_PORT_DEFAULT] =
            [const { AtomicWaker::new() }; NUM_PORT_DEFAULT];
        static WAKERS_FOR_PORT_F: [AtomicWaker; NUM_PORT_DEFAULT] =
            [const { AtomicWaker::new() }; NUM_PORT_DEFAULT];

        static EDGE_DETECTION_PORT_A: [AtomicBool; NUM_PORT_DEFAULT] =
            [const { AtomicBool::new(false) }; NUM_PORT_DEFAULT];
        static EDGE_DETECTION_PORT_B: [AtomicBool; NUM_PORT_DEFAULT] =
            [const { AtomicBool::new(false) }; NUM_PORT_DEFAULT];
        static EDGE_DETECTION_PORT_C: [AtomicBool; NUM_PORT_DEFAULT] =
            [const { AtomicBool::new(false) }; NUM_PORT_DEFAULT];
        static EDGE_DETECTION_PORT_D: [AtomicBool; NUM_PORT_DEFAULT] =
            [const { AtomicBool::new(false) }; NUM_PORT_DEFAULT];
        static EDGE_DETECTION_PORT_E: [AtomicBool; NUM_PORT_DEFAULT] =
            [const { AtomicBool::new(false) }; NUM_PORT_DEFAULT];
        static EDGE_DETECTION_PORT_F: [AtomicBool; NUM_PORT_DEFAULT] =
            [const { AtomicBool::new(false) }; NUM_PORT_DEFAULT];
    }
}

#[inline]
fn pin_group_to_waker_and_edge_detection_group(
    port: Port,
) -> (&'static [AtomicWaker], &'static [AtomicBool]) {
    match port {
        Port::A => (WAKERS_FOR_PORT_A.as_ref(), EDGE_DETECTION_PORT_A.as_ref()),
        Port::B => (WAKERS_FOR_PORT_B.as_ref(), EDGE_DETECTION_PORT_B.as_ref()),
        #[cfg(feature = "vor4x")]
        Port::C => (WAKERS_FOR_PORT_C.as_ref(), EDGE_DETECTION_PORT_C.as_ref()),
        #[cfg(feature = "vor4x")]
        Port::D => (WAKERS_FOR_PORT_D.as_ref(), EDGE_DETECTION_PORT_D.as_ref()),
        #[cfg(feature = "vor4x")]
        Port::E => (WAKERS_FOR_PORT_E.as_ref(), EDGE_DETECTION_PORT_E.as_ref()),
        #[cfg(feature = "vor4x")]
        Port::F => (WAKERS_FOR_PORT_F.as_ref(), EDGE_DETECTION_PORT_F.as_ref()),
        #[cfg(feature = "vor4x")]
        Port::G => unreachable!(),
    }
}

/// Generic interrupt handler for GPIO interrupts on a specific port to support async functionalities
///
/// This function should be called in all interrupt handlers which handle any GPIO interrupts
/// matching the [Port] argument.
/// The handler will wake the corresponding wakers for the pins that triggered an interrupts
/// as well as update the static edge detection structures. This allows the pin future to complete
/// complete async operations.
#[cfg(feature = "vor1x")]
pub fn on_interrupt_for_async_gpio_for_port(port: Port) {
    on_interrupt_for_async_gpio_for_port_generic(port);
}
#[cfg(feature = "vor4x")]
pub fn on_interrupt_for_async_gpio_for_port(
    port: Port,
) -> Result<(), PortDoesNotSupportInterrupts> {
    if port == Port::G {
        return Err(PortDoesNotSupportInterrupts);
    }
    on_interrupt_for_async_gpio_for_port_generic(port);
    Ok(())
}

fn on_interrupt_for_async_gpio_for_port_generic(port: Port) {
    let gpio = unsafe { port.steal_gpio() };

    let irq_enb = gpio.read_irq_enable();
    let edge_status = gpio.read_edge_status();
    let (wakers, edge_detection) = pin_group_to_waker_and_edge_detection_group(port);

    on_interrupt_for_port(irq_enb, edge_status, wakers, edge_detection);
}

#[inline]
fn on_interrupt_for_port(
    mut irq_enb: u32,
    edge_status: u32,
    wakers: &'static [AtomicWaker],
    edge_detection: &'static [AtomicBool],
) {
    while irq_enb != 0 {
        let bit_pos = irq_enb.trailing_zeros() as usize;
        let bit_mask = 1 << bit_pos;

        wakers[bit_pos].wake();

        if edge_status & bit_mask != 0 {
            edge_detection[bit_pos].store(true, core::sync::atomic::Ordering::Relaxed);

            // Clear the processed bit
            irq_enb &= !bit_mask;
        }
    }
}

/// Input pin future which implements the [Future] trait.
///
/// Generally, you want to use the [InputPinAsync] types instead of this
/// which also implements the [embedded_hal_async::digital::Wait] trait. However, access to this
/// struture is granted  to allow writing custom async structures.
pub struct InputPinFuture {
    id: DynPinId,
    waker_group: &'static [AtomicWaker],
    edge_detection_group: &'static [AtomicBool],
}

impl InputPinFuture {
    #[cfg(feature = "vor1x")]
    pub fn new_with_input_pin(pin: &mut Input, irq: pac::Interrupt, edge: InterruptEdge) -> Self {
        let (waker_group, edge_detection_group) =
            pin_group_to_waker_and_edge_detection_group(pin.id().port());
        edge_detection_group[pin.id().offset()].store(false, core::sync::atomic::Ordering::Relaxed);
        pin.configure_edge_interrupt(edge);
        #[cfg(feature = "vor1x")]
        pin.enable_interrupt(InterruptConfig::new(irq, true, true));
        Self {
            id: pin.id(),
            waker_group,
            edge_detection_group,
        }
    }
    #[cfg(feature = "vor4x")]
    pub fn new_with_input_pin(
        pin: &mut Input,
        edge: InterruptEdge,
    ) -> Result<Self, PortDoesNotSupportInterrupts> {
        let (waker_group, edge_detection_group) =
            pin_group_to_waker_and_edge_detection_group(pin.id().port());
        pin.configure_edge_interrupt(edge);
        pin.enable_interrupt(true)?;
        Ok(Self {
            id: pin.id(),
            waker_group,
            edge_detection_group,
        })
    }
}

impl Drop for InputPinFuture {
    fn drop(&mut self) {
        let mut ll = LowLevelGpio::new(self.id);
        #[cfg(feature = "vor1x")]
        ll.disable_interrupt(false);
        #[cfg(feature = "vor4x")]
        ll.disable_interrupt();
    }
}

impl Future for InputPinFuture {
    type Output = ();
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let idx = self.id.offset();
        self.waker_group[idx].register(cx.waker());
        if self.edge_detection_group[idx].swap(false, core::sync::atomic::Ordering::Relaxed) {
            return core::task::Poll::Ready(());
        }
        core::task::Poll::Pending
    }
}

pub struct InputPinAsync {
    pin: Input,
    #[cfg(feature = "vor1x")]
    irq: va108xx::Interrupt,
}

impl InputPinAsync {
    /// Create a new asynchronous input pin from an [Input] pin. The interrupt ID to be used must be
    /// passed as well and is used to route and enable the interrupt.
    ///
    /// Please note that the interrupt handler itself must be provided by the user and the
    /// generic [on_interrupt_for_async_gpio_for_port] function must be called inside that function
    /// for the asynchronous functionality to work.
    #[cfg(feature = "vor1x")]
    pub fn new(pin: Input, irq: va108xx::Interrupt) -> Self {
        Self { pin, irq }
    }
    #[cfg(feature = "vor4x")]
    pub fn new(pin: Input) -> Result<Self, PortDoesNotSupportInterrupts> {
        if pin.id().port() == Port::G {
            return Err(PortDoesNotSupportInterrupts);
        }
        Ok(Self { pin })
    }

    /// Asynchronously wait until the pin is high.
    ///
    /// This returns immediately if the pin is already high.
    pub async fn wait_for_high(&mut self) {
        // Unwrap okay, checked pin in constructor.
        #[cfg(feature = "vor1x")]
        let fut =
            InputPinFuture::new_with_input_pin(&mut self.pin, self.irq, InterruptEdge::LowToHigh);
        #[cfg(feature = "vor4x")]
        let fut =
            InputPinFuture::new_with_input_pin(&mut self.pin, InterruptEdge::LowToHigh).unwrap();
        if self.pin.is_high() {
            return;
        }
        fut.await;
    }

    /// Asynchronously wait until the pin is low.
    ///
    /// This returns immediately if the pin is already high.
    pub async fn wait_for_low(&mut self) {
        // Unwrap okay, checked pin in constructor.
        #[cfg(feature = "vor1x")]
        let fut =
            InputPinFuture::new_with_input_pin(&mut self.pin, self.irq, InterruptEdge::HighToLow);
        #[cfg(feature = "vor4x")]
        let fut =
            InputPinFuture::new_with_input_pin(&mut self.pin, InterruptEdge::HighToLow).unwrap();
        if self.pin.is_low() {
            return;
        }
        fut.await;
    }

    /// Asynchronously wait until the pin sees a falling edge.
    pub async fn wait_for_falling_edge(&mut self) {
        // Unwrap okay, checked pin in constructor.
        #[cfg(feature = "vor1x")]
        InputPinFuture::new_with_input_pin(&mut self.pin, self.irq, InterruptEdge::HighToLow).await;
        #[cfg(feature = "vor4x")]
        InputPinFuture::new_with_input_pin(&mut self.pin, InterruptEdge::HighToLow)
            .unwrap()
            .await;
    }

    /// Asynchronously wait until the pin sees a rising edge.
    pub async fn wait_for_rising_edge(&mut self) {
        // Unwrap okay, checked pin in constructor.
        #[cfg(feature = "vor1x")]
        InputPinFuture::new_with_input_pin(&mut self.pin, self.irq, InterruptEdge::LowToHigh).await;
    }

    /// Asynchronously wait until the pin sees any edge (either rising or falling).
    pub async fn wait_for_any_edge(&mut self) {
        // Unwrap okay, checked pin in constructor.
        #[cfg(feature = "vor1x")]
        InputPinFuture::new_with_input_pin(&mut self.pin, self.irq, InterruptEdge::BothEdges).await;
        #[cfg(feature = "vor4x")]
        InputPinFuture::new_with_input_pin(&mut self.pin, InterruptEdge::BothEdges)
            .unwrap()
            .await;
    }

    pub fn release(self) -> Input {
        self.pin
    }
}

impl embedded_hal::digital::ErrorType for InputPinAsync {
    type Error = core::convert::Infallible;
}

impl Wait for InputPinAsync {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        self.wait_for_high().await;
        Ok(())
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        self.wait_for_low().await;
        Ok(())
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_rising_edge().await;
        Ok(())
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_falling_edge().await;
        Ok(())
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_any_edge().await;
        Ok(())
    }
}
