//! GPIO support module.
use core::convert::Infallible;

pub use crate::ioconfig::{FilterClockSelect, FilterType, regs::FunctionSelect};
pub use crate::pins::{Pin, PinId};
pub use embedded_hal::digital::PinState;
pub use ll::{DynPinId, InterruptEdge, InterruptLevel, Port, Pull};

pub mod asynch;
pub mod ll;
pub mod regs;

/// Push-Pull output pin.
#[derive(Debug)]
pub struct Output(ll::LowLevelGpio);

impl Output {
    pub fn new<I: PinId>(_pin: Pin<I>, init_level: PinState) -> Self {
        let mut ll = ll::LowLevelGpio::new(I::ID);
        ll.configure_as_output_push_pull(init_level);
        Output(ll)
    }

    #[inline]
    pub fn port(&self) -> Port {
        self.0.port()
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.0.offset()
    }

    #[inline]
    pub fn set_high(&mut self) {
        self.0.set_high();
    }

    #[inline]
    pub fn set_low(&mut self) {
        self.0.set_low();
    }

    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.0.is_set_high()
    }

    #[inline]
    pub fn is_set_low(&self) -> bool {
        self.0.is_set_low()
    }

    /// Toggle pin output with dedicated HW feature.
    #[inline]
    pub fn toggle(&mut self) {
        self.0.toggle();
    }

    #[inline]
    pub fn configure_pulse_mode(&mut self, enable: bool, default_state: PinState) {
        self.0.configure_pulse_mode(enable, default_state);
    }

    #[inline]
    pub fn configure_delay(&mut self, delay_1: bool, delay_2: bool) {
        self.0.configure_delay(delay_1, delay_2);
    }
}

impl embedded_hal::digital::ErrorType for Output {
    type Error = Infallible;
}

impl embedded_hal::digital::OutputPin for Output {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.0.set_low();
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.0.set_high();
        Ok(())
    }
}

impl embedded_hal::digital::StatefulOutputPin for Output {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.0.is_set_high())
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.0.is_set_low())
    }

    /// Toggle pin output with dedicated HW feature.
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.0.toggle();
        Ok(())
    }
}

/// Input pin.
///
/// Can be created as a floating input pin or as an input pin with pull-up or pull-down.
#[derive(Debug)]
pub struct Input(ll::LowLevelGpio);

impl Input {
    pub fn new_floating<I: PinId>(_pin: Pin<I>) -> Self {
        let mut ll = ll::LowLevelGpio::new(I::ID);
        ll.configure_as_input_floating();
        Input(ll)
    }

    pub fn new_with_pull<I: PinId>(_pin: Pin<I>, pull: Pull) -> Self {
        let mut ll = ll::LowLevelGpio::new(I::ID);
        ll.configure_as_input_with_pull(pull);
        Input(ll)
    }

    #[inline]
    pub fn id(&self) -> DynPinId {
        self.0.id()
    }

    #[cfg(feature = "vor1x")]
    #[inline]
    pub fn enable_interrupt(&mut self, irq_cfg: crate::InterruptConfig) {
        self.0.enable_interrupt(irq_cfg);
    }

    #[cfg(feature = "vor4x")]
    #[inline]
    pub fn enable_interrupt(
        &mut self,
        enable_in_nvic: bool,
    ) -> Result<(), ll::PortDoesNotSupportInterrupts> {
        self.0.enable_interrupt(enable_in_nvic)
    }

    #[inline]
    pub fn configure_edge_interrupt(&mut self, edge: InterruptEdge) {
        self.0.configure_edge_interrupt(edge);
    }

    #[inline]
    pub fn configure_level_interrupt(&mut self, edge: InterruptLevel) {
        self.0.configure_level_interrupt(edge);
    }

    #[inline]
    pub fn configure_delay(&mut self, delay_1: bool, delay_2: bool) {
        self.0.configure_delay(delay_1, delay_2);
    }

    #[inline]
    pub fn configure_filter_type(&mut self, filter: FilterType, clksel: FilterClockSelect) {
        self.0.configure_filter_type(filter, clksel);
    }

    #[inline]
    pub fn is_low(&self) -> bool {
        self.0.is_low()
    }

    #[inline]
    pub fn is_high(&self) -> bool {
        self.0.is_high()
    }
}

impl embedded_hal::digital::ErrorType for Input {
    type Error = Infallible;
}

impl embedded_hal::digital::InputPin for Input {
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.0.is_low())
    }

    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.0.is_high())
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PinMode {
    InputFloating,
    InputWithPull(Pull),
    OutputPushPull,
    OutputOpenDrain,
}

impl PinMode {
    pub fn is_input(&self) -> bool {
        matches!(self, PinMode::InputFloating | PinMode::InputWithPull(_))
    }

    pub fn is_output(&self) -> bool {
        !self.is_input()
    }
}

/// Flex pin abstraction which can be dynamically re-configured.
///
/// The following functions can be configured at run-time:
///
///  - Input Floating
///  - Input with Pull-Up
///  - Output Push-Pull
///  - Output Open-Drain.
///
/// Flex pins are always floating input pins after construction.
#[derive(Debug)]
pub struct Flex {
    ll: ll::LowLevelGpio,
    mode: PinMode,
}

impl Flex {
    pub fn new<I: PinId>(_pin: Pin<I>) -> Self {
        let mut ll = ll::LowLevelGpio::new(I::ID);
        ll.configure_as_input_floating();
        Flex {
            ll,
            mode: PinMode::InputFloating,
        }
    }

    #[inline]
    pub fn port(&self) -> Port {
        self.ll.port()
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.ll.offset()
    }

    /// Reads the input state of the pin, regardless of configured mode.
    #[inline]
    pub fn is_low(&self) -> bool {
        self.ll.is_low()
    }

    /// Reads the input state of the pin, regardless of configured mode.
    #[inline]
    pub fn is_high(&self) -> bool {
        self.ll.is_high()
    }

    /// If the pin is configured as an input pin, this function does nothing.
    #[inline]
    pub fn set_low(&mut self) {
        if !self.mode.is_input() {
            return;
        }
        self.ll.set_low();
    }

    /// If the pin is configured as an input pin, this function does nothing.
    #[inline]
    pub fn set_high(&mut self) {
        if !self.mode.is_input() {
            return;
        }
        self.ll.set_high();
    }
}

impl embedded_hal::digital::ErrorType for Flex {
    type Error = Infallible;
}

impl embedded_hal::digital::InputPin for Flex {
    /// Reads the input state of the pin, regardless of configured mode.
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.ll.is_low())
    }

    /// Reads the input state of the pin, regardless of configured mode.
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.ll.is_high())
    }
}

impl embedded_hal::digital::OutputPin for Flex {
    /// If the pin is configured as an input pin, this function does nothing.
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }

    /// If the pin is configured as an input pin, this function does nothing.
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }
}

impl embedded_hal::digital::StatefulOutputPin for Flex {
    /// If the pin is not configured as a stateful output pin like Output Push-Pull, the result
    /// of this function is undefined.
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.ll.is_set_high())
    }

    /// If the pin is not configured as a stateful output pin like Output Push-Pull, the result
    /// of this function is undefined.
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.ll.is_set_low())
    }

    /// Toggle pin output.
    ///
    /// If the pin is not configured as a stateful output pin like Output Push-Pull, the result
    /// of this function is undefined.
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.ll.toggle();
        Ok(())
    }
}

/// IO peripheral pin structure.
///
/// Can be used to configure pins as IO peripheral pins.
pub struct IoPeriphPin {
    ll: ll::LowLevelGpio,
    fun_sel: FunctionSelect,
}

impl IoPeriphPin {
    pub fn new_with_pin<I: PinId>(
        _pin: Pin<I>,
        fun_sel: FunctionSelect,
        pull: Option<Pull>,
    ) -> Self {
        let mut ll = ll::LowLevelGpio::new(I::ID);
        ll.configure_as_peripheral_pin(fun_sel, pull);
        IoPeriphPin { ll, fun_sel }
    }

    pub fn new(pin_id: DynPinId, fun_sel: FunctionSelect, pull: Option<Pull>) -> Self {
        let mut ll = ll::LowLevelGpio::new(pin_id);
        ll.configure_as_peripheral_pin(fun_sel, pull);
        IoPeriphPin { ll, fun_sel }
    }

    pub fn port(&self) -> Port {
        self.ll.port()
    }

    pub fn offset(&self) -> usize {
        self.ll.offset()
    }

    pub fn fun_sel(&self) -> FunctionSelect {
        self.fun_sel
    }
}
