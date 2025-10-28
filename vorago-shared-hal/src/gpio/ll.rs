pub use embedded_hal::digital::PinState;

use crate::ioconfig::FilterClockSelect;
use crate::ioconfig::FilterType;
#[cfg(feature = "vor1x")]
use crate::{PeripheralSelect, sysconfig::enable_peripheral_clock};

pub use crate::InvalidOffsetError;
pub use crate::Port;
pub use crate::ioconfig::regs::Pull;
use crate::ioconfig::regs::{FunctionSelect, IoConfig, MmioIoConfig};
use crate::pins::PinId;

use super::Pin;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InterruptEdge {
    HighToLow,
    LowToHigh,
    BothEdges,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InterruptLevel {
    Low = 0,
    High = 1,
}

/// Pin identifier for all physical pins exposed by Vorago MCUs.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DynPinId {
    port: Port,
    /// Offset within the port.
    offset: u8,
}

#[derive(Debug, thiserror::Error)]
#[cfg(feature = "vor4x")]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("port G does not support interrupts")]
pub struct PortDoesNotSupportInterrupts;

impl DynPinId {
    /// Unchecked constructor which panics on invalid offsets.
    pub const fn new_unchecked(port: Port, offset: usize) -> Self {
        if offset >= port.max_offset() {
            panic!("Pin ID construction: offset is out of range");
        }
        DynPinId {
            port,
            offset: offset as u8,
        }
    }

    pub const fn new(port: Port, offset: usize) -> Result<Self, InvalidOffsetError> {
        if offset >= port.max_offset() {
            return Err(InvalidOffsetError { offset, port });
        }
        Ok(DynPinId {
            port,
            offset: offset as u8,
        })
    }

    pub const fn port(&self) -> Port {
        self.port
    }

    pub const fn offset(&self) -> usize {
        self.offset as usize
    }

    /// This function panics if the port is [Port::G].
    #[cfg(feature = "vor4x")]
    pub fn irq(&self) -> Result<va416xx::Interrupt, PortDoesNotSupportInterrupts> {
        if self.port() == Port::G {
            return Err(PortDoesNotSupportInterrupts);
        }
        Ok(self.irq_unchecked())
    }

    /// This function panics if the port is [Port::G].
    #[cfg(feature = "vor4x")]
    pub const fn irq_unchecked(&self) -> va416xx::Interrupt {
        match self.port() {
            Port::A => match self.offset() {
                0 => va416xx::Interrupt::PORTA0,
                1 => va416xx::Interrupt::PORTA1,
                2 => va416xx::Interrupt::PORTA2,
                3 => va416xx::Interrupt::PORTA3,
                4 => va416xx::Interrupt::PORTA4,
                5 => va416xx::Interrupt::PORTA5,
                6 => va416xx::Interrupt::PORTA6,
                7 => va416xx::Interrupt::PORTA7,
                8 => va416xx::Interrupt::PORTA8,
                9 => va416xx::Interrupt::PORTA9,
                10 => va416xx::Interrupt::PORTA10,
                11 => va416xx::Interrupt::PORTA11,
                12 => va416xx::Interrupt::PORTA12,
                13 => va416xx::Interrupt::PORTA13,
                14 => va416xx::Interrupt::PORTA14,
                15 => va416xx::Interrupt::PORTA15,
                _ => unreachable!(),
            },
            Port::B => match self.offset() {
                0 => va416xx::Interrupt::PORTB0,
                1 => va416xx::Interrupt::PORTB1,
                2 => va416xx::Interrupt::PORTB2,
                3 => va416xx::Interrupt::PORTB3,
                4 => va416xx::Interrupt::PORTB4,
                5 => va416xx::Interrupt::PORTB5,
                6 => va416xx::Interrupt::PORTB6,
                7 => va416xx::Interrupt::PORTB7,
                8 => va416xx::Interrupt::PORTB8,
                9 => va416xx::Interrupt::PORTB9,
                10 => va416xx::Interrupt::PORTB10,
                11 => va416xx::Interrupt::PORTB11,
                12 => va416xx::Interrupt::PORTB12,
                13 => va416xx::Interrupt::PORTB13,
                14 => va416xx::Interrupt::PORTB14,
                15 => va416xx::Interrupt::PORTB15,
                _ => unreachable!(),
            },
            Port::C => match self.offset() {
                0 => va416xx::Interrupt::PORTC0,
                1 => va416xx::Interrupt::PORTC1,
                2 => va416xx::Interrupt::PORTC2,
                3 => va416xx::Interrupt::PORTC3,
                4 => va416xx::Interrupt::PORTC4,
                5 => va416xx::Interrupt::PORTC5,
                6 => va416xx::Interrupt::PORTC6,
                7 => va416xx::Interrupt::PORTC7,
                8 => va416xx::Interrupt::PORTC8,
                9 => va416xx::Interrupt::PORTC9,
                10 => va416xx::Interrupt::PORTC10,
                11 => va416xx::Interrupt::PORTC11,
                12 => va416xx::Interrupt::PORTC12,
                13 => va416xx::Interrupt::PORTC13,
                14 => va416xx::Interrupt::PORTC14,
                15 => va416xx::Interrupt::PORTC15,
                _ => unreachable!(),
            },
            Port::D => match self.offset() {
                0 => va416xx::Interrupt::PORTD0,
                1 => va416xx::Interrupt::PORTD1,
                2 => va416xx::Interrupt::PORTD2,
                3 => va416xx::Interrupt::PORTD3,
                4 => va416xx::Interrupt::PORTD4,
                5 => va416xx::Interrupt::PORTD5,
                6 => va416xx::Interrupt::PORTD6,
                7 => va416xx::Interrupt::PORTD7,
                8 => va416xx::Interrupt::PORTD8,
                9 => va416xx::Interrupt::PORTD9,
                10 => va416xx::Interrupt::PORTD10,
                11 => va416xx::Interrupt::PORTD11,
                12 => va416xx::Interrupt::PORTD12,
                13 => va416xx::Interrupt::PORTD13,
                14 => va416xx::Interrupt::PORTD14,
                15 => va416xx::Interrupt::PORTD15,
                _ => unreachable!(),
            },
            Port::E => match self.offset() {
                0 => va416xx::Interrupt::PORTE0,
                1 => va416xx::Interrupt::PORTE1,
                2 => va416xx::Interrupt::PORTE2,
                3 => va416xx::Interrupt::PORTE3,
                4 => va416xx::Interrupt::PORTE4,
                5 => va416xx::Interrupt::PORTE5,
                6 => va416xx::Interrupt::PORTE6,
                7 => va416xx::Interrupt::PORTE7,
                8 => va416xx::Interrupt::PORTE8,
                9 => va416xx::Interrupt::PORTE9,
                10 => va416xx::Interrupt::PORTE10,
                11 => va416xx::Interrupt::PORTE11,
                12 => va416xx::Interrupt::PORTE12,
                13 => va416xx::Interrupt::PORTE13,
                14 => va416xx::Interrupt::PORTE14,
                15 => va416xx::Interrupt::PORTE15,
                _ => unreachable!(),
            },
            Port::F => match self.offset() {
                0 => va416xx::Interrupt::PORTF0,
                1 => va416xx::Interrupt::PORTF1,
                2 => va416xx::Interrupt::PORTF2,
                3 => va416xx::Interrupt::PORTF3,
                4 => va416xx::Interrupt::PORTF4,
                5 => va416xx::Interrupt::PORTF5,
                6 => va416xx::Interrupt::PORTF6,
                7 => va416xx::Interrupt::PORTF7,
                8 => va416xx::Interrupt::PORTF8,
                9 => va416xx::Interrupt::PORTF9,
                10 => va416xx::Interrupt::PORTF10,
                11 => va416xx::Interrupt::PORTF11,
                12 => va416xx::Interrupt::PORTF12,
                13 => va416xx::Interrupt::PORTF13,
                14 => va416xx::Interrupt::PORTF14,
                15 => va416xx::Interrupt::PORTF15,
                _ => unreachable!(),
            },
            Port::G => panic!("port G does not have interrupts"),
        }
    }
}

/// Low-level driver structure for GPIO pins.
pub struct LowLevelGpio {
    gpio: super::regs::MmioGpio<'static>,
    ioconfig: MmioIoConfig<'static>,
    id: DynPinId,
}

impl core::fmt::Debug for LowLevelGpio {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LowLevelGpio")
            .field("gpio", &self.gpio.port())
            .field("id", &self.id)
            .finish()
    }
}

impl LowLevelGpio {
    /// Create a new low-level GPIO pin instance from a given [Pin].
    ///
    /// Can be used for performing resource management of the [Pin]s.
    pub fn new_with_pin<I: PinId>(_pin: Pin<I>) -> Self {
        Self::new(I::ID)
    }

    /// Create a new low-level GPIO pin instance using only the [PinId].
    pub fn new(id: DynPinId) -> Self {
        LowLevelGpio {
            gpio: super::regs::Gpio::new_mmio(id.port),
            ioconfig: IoConfig::new_mmio(),
            id,
        }
    }

    #[inline]
    pub fn id(&self) -> DynPinId {
        self.id
    }

    #[inline]
    pub fn port(&self) -> Port {
        self.id.port()
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.id.offset()
    }

    pub fn configure_as_input_floating(&mut self) {
        self.ioconfig.modify_pin_config(self.id, |mut config| {
            config.set_funsel(FunctionSelect::Sel0);
            config.set_io_disable(false);
            config.set_invert_input(false);
            config.set_open_drain(false);
            config.set_pull_enable(false);
            config.set_pull_when_output_active(false);
            config.set_invert_output(false);
            config.set_input_enable_when_output(false);
            config
        });
        self.gpio.modify_dir(|mut dir| {
            dir &= !(1 << self.id.offset());
            dir
        });
    }

    pub fn configure_as_input_with_pull(&mut self, pull: Pull) {
        self.ioconfig.modify_pin_config(self.id, |mut config| {
            config.set_funsel(FunctionSelect::Sel0);
            config.set_io_disable(false);
            config.set_invert_input(false);
            config.set_open_drain(false);
            config.set_pull_enable(true);
            config.set_pull_dir(pull);
            config.set_pull_when_output_active(false);
            config.set_invert_output(false);
            config.set_input_enable_when_output(false);
            config
        });
        self.gpio.modify_dir(|mut dir| {
            dir &= !(1 << self.id.offset());
            dir
        });
    }

    pub fn configure_as_output_push_pull(&mut self, init_level: PinState) {
        self.ioconfig.modify_pin_config(self.id, |mut config| {
            config.set_funsel(FunctionSelect::Sel0);
            config.set_io_disable(false);
            config.set_invert_input(false);
            config.set_open_drain(false);
            config.set_pull_enable(false);
            config.set_pull_when_output_active(false);
            config.set_invert_output(false);
            config.set_input_enable_when_output(true);
            config
        });
        match init_level {
            PinState::Low => self.gpio.write_clr_out(self.mask_32()),
            PinState::High => self.gpio.write_set_out(self.mask_32()),
        }
        self.gpio.modify_dir(|mut dir| {
            dir |= 1 << self.id.offset();
            dir
        });
    }

    pub fn configure_as_output_open_drain(&mut self, init_level: PinState) {
        self.ioconfig.modify_pin_config(self.id, |mut config| {
            config.set_funsel(FunctionSelect::Sel0);
            config.set_io_disable(false);
            config.set_invert_input(false);
            config.set_open_drain(true);
            config.set_pull_enable(true);
            config.set_pull_dir(Pull::Up);
            config.set_pull_when_output_active(false);
            config.set_invert_output(false);
            config.set_input_enable_when_output(true);
            config
        });
        let mask32 = self.mask_32();
        match init_level {
            PinState::Low => self.gpio.write_clr_out(mask32),
            PinState::High => self.gpio.write_set_out(mask32),
        }
        self.gpio.modify_dir(|mut dir| {
            dir |= mask32;
            dir
        });
    }

    pub fn configure_as_peripheral_pin(&mut self, fun_sel: FunctionSelect, pull: Option<Pull>) {
        self.ioconfig.modify_pin_config(self.id, |mut config| {
            config.set_funsel(fun_sel);
            config.set_io_disable(false);
            config.set_invert_input(false);
            config.set_open_drain(false);
            config.set_pull_enable(pull.is_some());
            config.set_pull_dir(pull.unwrap_or(Pull::Up));
            config.set_invert_output(false);
            config
        });
    }

    #[inline]
    pub fn is_high(&self) -> bool {
        (self.gpio.read_data_in() >> self.offset()) & 1 == 1
    }

    #[inline]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    #[inline]
    pub fn set_high(&mut self) {
        self.gpio.write_set_out(self.mask_32());
    }

    #[inline]
    pub fn set_low(&mut self) {
        self.gpio.write_clr_out(self.mask_32());
    }

    #[inline]
    pub fn is_set_high(&self) -> bool {
        (self.gpio.read_data_out() >> self.offset()) & 1 == 1
    }

    #[inline]
    pub fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }

    #[inline]
    pub fn toggle(&mut self) {
        self.gpio.write_tog_out(self.mask_32());
    }

    #[cfg(feature = "vor1x")]
    pub fn enable_interrupt(&mut self, irq_cfg: crate::InterruptConfig) {
        if irq_cfg.route {
            self.configure_irqsel(irq_cfg.id);
        }
        if irq_cfg.enable_in_nvic {
            unsafe { crate::enable_nvic_interrupt(irq_cfg.id) };
        }
        self.gpio.modify_irq_enable(|mut value| {
            value |= 1 << self.id.offset;
            value
        });
    }

    #[cfg(feature = "vor4x")]
    pub fn enable_interrupt(
        &mut self,
        enable_in_nvic: bool,
    ) -> Result<(), PortDoesNotSupportInterrupts> {
        if enable_in_nvic {
            unsafe { crate::enable_nvic_interrupt(self.id().irq_unchecked()) };
        }
        self.gpio.modify_irq_enable(|mut value| {
            value |= 1 << self.id.offset;
            value
        });
        Ok(())
    }

    #[cfg(feature = "vor1x")]
    pub fn disable_interrupt(&mut self, reset_irqsel: bool) {
        if reset_irqsel {
            self.reset_irqsel();
        }
        // We only manipulate our own bit.
        self.gpio.modify_irq_enable(|mut value| {
            value &= !(1 << self.id.offset);
            value
        });
    }

    #[cfg(feature = "vor4x")]
    pub fn disable_interrupt(&mut self) {
        self.gpio.modify_irq_enable(|mut value| {
            value &= !(1 << self.id.offset);
            value
        });
    }

    /// Only useful for interrupt pins. Configure whether to use edges or level as interrupt soure
    /// When using edge mode, it is possible to generate interrupts on both edges as well
    #[inline]
    pub fn configure_edge_interrupt(&mut self, edge_type: InterruptEdge) {
        let mask32 = self.mask_32();
        self.gpio.modify_irq_sen(|mut value| {
            value &= !mask32;
            value
        });
        match edge_type {
            InterruptEdge::HighToLow => {
                self.gpio.modify_irq_evt(|mut value| {
                    value &= !mask32;
                    value
                });
            }
            InterruptEdge::LowToHigh => {
                self.gpio.modify_irq_evt(|mut value| {
                    value |= mask32;
                    value
                });
            }
            InterruptEdge::BothEdges => {
                self.gpio.modify_irq_edge(|mut value| {
                    value |= mask32;
                    value
                });
            }
        }
    }

    /// Configure which edge or level type triggers an interrupt
    #[inline]
    pub fn configure_level_interrupt(&mut self, level: InterruptLevel) {
        let mask32 = self.mask_32();
        self.gpio.modify_irq_sen(|mut value| {
            value |= mask32;
            value
        });
        if level == InterruptLevel::Low {
            self.gpio.modify_irq_evt(|mut value| {
                value &= !mask32;
                value
            });
        } else {
            self.gpio.modify_irq_evt(|mut value| {
                value |= mask32;
                value
            });
        }
    }

    /// Only useful for input pins
    #[inline]
    pub fn configure_filter_type(&mut self, filter: FilterType, clksel: FilterClockSelect) {
        self.ioconfig.modify_pin_config(self.id, |mut config| {
            config.set_filter_type(filter);
            config.set_filter_clk_sel(clksel);
            config
        });
    }

    /// Only useful for output pins.
    #[inline]
    pub fn configure_pulse_mode(&mut self, enable: bool, default_state: PinState) {
        self.gpio.modify_pulse(|mut value| {
            if enable {
                value |= 1 << self.id.offset;
            } else {
                value &= !(1 << self.id.offset);
            }
            value
        });
        self.gpio.modify_pulsebase(|mut value| {
            if default_state == PinState::High {
                value |= 1 << self.id.offset;
            } else {
                value &= !(1 << self.id.offset);
            }
            value
        });
    }

    /// Only useful for output pins
    #[inline]
    pub fn configure_delay(&mut self, delay_1: bool, delay_2: bool) {
        self.gpio.modify_delay1(|mut value| {
            if delay_1 {
                value |= 1 << self.id.offset;
            } else {
                value &= !(1 << self.id.offset);
            }
            value
        });
        self.gpio.modify_delay2(|mut value| {
            if delay_2 {
                value |= 1 << self.id.offset;
            } else {
                value &= !(1 << self.id.offset);
            }
            value
        });
    }

    #[cfg(feature = "vor1x")]
    /// Configure the IRQSEL peripheral for this particular pin with the given interrupt ID.
    pub fn configure_irqsel(&mut self, id: va108xx::Interrupt) {
        let irqsel = unsafe { va108xx::Irqsel::steal() };
        enable_peripheral_clock(PeripheralSelect::Irqsel);
        match self.id().port() {
            // Set the correct interrupt number in the IRQSEL register
            super::Port::A => {
                irqsel
                    .porta(self.id().offset())
                    .write(|w| unsafe { w.bits(id as u32) });
            }
            super::Port::B => {
                irqsel
                    .portb(self.id().offset())
                    .write(|w| unsafe { w.bits(id as u32) });
            }
        }
    }

    #[cfg(feature = "vor1x")]
    /// Reset the IRQSEL peripheral value for this particular pin.
    pub fn reset_irqsel(&mut self) {
        let irqsel = unsafe { va108xx::Irqsel::steal() };
        enable_peripheral_clock(PeripheralSelect::Irqsel);
        match self.id().port() {
            // Set the correct interrupt number in the IRQSEL register
            super::Port::A => {
                irqsel
                    .porta(self.id().offset())
                    .write(|w| unsafe { w.bits(u32::MAX) });
            }
            super::Port::B => {
                irqsel
                    .portb(self.id().offset())
                    .write(|w| unsafe { w.bits(u32::MAX) });
            }
        }
    }

    #[inline(always)]
    pub const fn mask_32(&self) -> u32 {
        1 << self.id.offset()
    }
}
