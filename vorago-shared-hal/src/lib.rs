//! Shared HAL code for Vorago VA108xx and VA416xx microcontrollers.
#![no_std]
#[cfg(feature = "vor4x")]
pub mod clock;
pub mod embassy;
pub mod gpio;
pub mod i2c;
pub mod ioconfig;
pub mod pins;
pub mod pwm;
pub mod spi;
pub mod sysconfig;
pub mod time;
pub mod timer;
pub mod uart;

pub use sysconfig::{
    assert_peripheral_reset, deassert_peripheral_reset, disable_peripheral_clock,
    enable_peripheral_clock, reset_peripheral_for_cycles,
};

#[cfg(not(feature = "_family-selected"))]
compile_error!("no Vorago CPU family was select. Choices: vor1x or vor4x");

pub use ioconfig::regs::FunctionSelect;
#[cfg(feature = "vor1x")]
use va108xx as pac;
#[cfg(feature = "vor4x")]
use va416xx as pac;

#[cfg(feature = "vor1x")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeripheralSelect {
    PortA = 0,
    PortB = 1,
    Spi0 = 4,
    Spi1 = 5,
    Spi2 = 6,
    Uart0 = 8,
    Uart1 = 9,
    I2c0 = 16,
    I2c1 = 17,
    Irqsel = 21,
    IoConfig = 22,
    Utility = 23,
    Gpio = 24,
}

#[cfg(feature = "vor4x")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeripheralSelect {
    Spi0 = 0,
    Spi1 = 1,
    Spi2 = 2,
    Spi3 = 3,
    Uart0 = 4,
    Uart1 = 5,
    Uart2 = 6,
    I2c0 = 7,
    I2c1 = 8,
    I2c2 = 9,
    Can0 = 10,
    Can1 = 11,
    Rng = 12,
    Adc = 13,
    Dac = 14,
    Dma = 15,
    Ebi = 16,
    Eth = 17,
    Spw = 18,
    Clkgen = 19,
    IrqRouter = 20,
    IoConfig = 21,
    Utility = 22,
    Watchdog = 23,
    PortA = 24,
    PortB = 25,
    PortC = 26,
    PortD = 27,
    PortE = 28,
    PortF = 29,
    PortG = 30,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "vor1x")] {
        /// Number of GPIO ports and IOCONFIG registers for PORT A
        pub const NUM_PORT_A: usize = 32;
        /// Number of GPIO ports and IOCONFIG registers for PORT B
        pub const NUM_PORT_B: usize = 24;
    } else if #[cfg(feature = "vor4x")] {
        /// Number of GPIO ports and IOCONFIG registers for PORT C to Port F
        pub const NUM_PORT_DEFAULT: usize = 16;
        /// Number of GPIO ports and IOCONFIG registers for PORT A
        pub const NUM_PORT_A: usize = NUM_PORT_DEFAULT;
        /// Number of GPIO ports and IOCONFIG registers for PORT B
        pub const NUM_PORT_B: usize = NUM_PORT_DEFAULT;
        /// Number of GPIO ports and IOCONFIG registers for PORT G
        pub const NUM_PORT_G: usize = 8;
    }
}

/// GPIO port enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port {
    A = 0,
    B = 1,
    #[cfg(feature = "vor4x")]
    C = 2,
    #[cfg(feature = "vor4x")]
    D = 3,
    #[cfg(feature = "vor4x")]
    E = 4,
    #[cfg(feature = "vor4x")]
    F = 5,
    #[cfg(feature = "vor4x")]
    G = 6,
}

impl Port {
    pub const fn max_offset(&self) -> usize {
        match self {
            Port::A => NUM_PORT_A,
            Port::B => NUM_PORT_B,
            #[cfg(feature = "vor4x")]
            Port::C | Port::D | Port::E | Port::F => NUM_PORT_DEFAULT,
            #[cfg(feature = "vor4x")]
            Port::G => NUM_PORT_G,
        }
    }

    /// Unsafely steal the GPIO peripheral block for the given port.
    ///
    /// # Safety
    ///
    /// Circumvents ownership and safety guarantees by the HAL.
    pub unsafe fn steal_gpio(&self) -> gpio::regs::MmioGpio<'static> {
        gpio::regs::Gpio::new_mmio(*self)
    }
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("invalid GPIO offset {offset} for port {port:?}")]
pub struct InvalidOffsetError {
    offset: usize,
    port: Port,
}

/// Generic interrupt config which can be used to specify whether the HAL driver will
/// use the IRQSEL register to route an interrupt, and whether the IRQ will be unmasked in the
/// Cortex-M0 NVIC. Both are generally necessary for IRQs to work, but the user might want to
/// perform those steps themselves.
#[cfg(feature = "vor1x")]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InterruptConfig {
    /// Interrupt target vector. Should always be set, might be required for disabling IRQs
    pub id: va108xx::Interrupt,
    /// Specfiy whether IRQ should be routed to an IRQ vector using the IRQSEL peripheral.
    pub route: bool,
    /// Specify whether the IRQ is unmasked in the Cortex-M NVIC. If an interrupt is used for
    /// multiple purposes, the user can enable the interrupts themselves.
    pub enable_in_nvic: bool,
}

#[cfg(feature = "vor1x")]
impl InterruptConfig {
    pub fn new(id: va108xx::Interrupt, route: bool, enable_in_nvic: bool) -> Self {
        InterruptConfig {
            id,
            route,
            enable_in_nvic,
        }
    }
}

/// Enable a specific interrupt using the NVIC peripheral.
///
/// # Safety
///
/// This function is `unsafe` because it can break mask-based critical sections.
#[inline]
pub unsafe fn enable_nvic_interrupt(irq: pac::Interrupt) {
    unsafe {
        cortex_m::peripheral::NVIC::unmask(irq);
    }
}

/// Disable a specific interrupt using the NVIC peripheral.
#[inline]
pub fn disable_nvic_interrupt(irq: pac::Interrupt) {
    cortex_m::peripheral::NVIC::mask(irq);
}

#[allow(dead_code)]
pub(crate) mod sealed {
    pub trait Sealed {}
}

pub(crate) mod shared {
    use arbitrary_int::u5;

    #[derive(Debug)]
    pub struct TriggerLevel(arbitrary_int::UInt<u32, 5>);

    impl TriggerLevel {
        pub const fn new(value: u5) -> Self {
            TriggerLevel(arbitrary_int::UInt::<u32, 5>::new(value.value() as u32))
        }

        pub const fn value(&self) -> u5 {
            u5::new(self.0.value() as u8)
        }
    }

    #[bitbybit::bitfield(u32, default = 0x0)]
    #[derive(Debug)]
    pub struct FifoClear {
        #[bit(1, w)]
        tx_fifo: bool,
        #[bit(0, w)]
        rx_fifo: bool,
    }
}
