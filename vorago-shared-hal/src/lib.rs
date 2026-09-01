//! Shared HAL code for Vorago VA108xx and VA416xx microcontrollers.
#![no_std]
#![deny(missing_docs)]
/// Clock configuration for Vorago 4x devices.
#[cfg(feature = "vor4x")]
pub mod clock;
/// Embassy time driver integration.
pub mod embassy;
pub mod gpio;
/// I2C peripheral driver.
pub mod i2c;
/// Pin function select (IOCONFIG) peripheral driver.
pub mod ioconfig;
/// GPIO pin type definitions for each port.
pub mod pins;
/// PWM peripheral driver.
pub mod pwm;
/// SPI peripheral driver.
pub mod spi;
/// Peripheral clock gating and reset control.
pub mod sysconfig;
pub mod time;
/// Timer peripheral driver.
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

/// Peripherals which can be individually clock-gated and reset, for use with
/// [enable_peripheral_clock] and related functions.
#[cfg(feature = "vor1x")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeripheralSelect {
    /// GPIO port A.
    PortA = 0,
    /// GPIO port B.
    PortB = 1,
    /// SPI0.
    Spi0 = 4,
    /// SPI1.
    Spi1 = 5,
    /// SPI2.
    Spi2 = 6,
    /// UART0.
    Uart0 = 8,
    /// UART1.
    Uart1 = 9,
    /// I2C0.
    I2c0 = 16,
    /// I2C1.
    I2c1 = 17,
    /// IRQSEL, the interrupt routing peripheral.
    Irqsel = 21,
    /// IOCONFIG, the pin function select peripheral.
    IoConfig = 22,
    /// The utility peripheral.
    Utility = 23,
    /// The GPIO peripheral itself, as opposed to an individual port.
    Gpio = 24,
}

/// Peripherals which can be individually clock-gated and reset, for use with
/// [enable_peripheral_clock] and related functions.
#[cfg(feature = "vor4x")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeripheralSelect {
    /// SPI0.
    Spi0 = 0,
    /// SPI1.
    Spi1 = 1,
    /// SPI2.
    Spi2 = 2,
    /// SPI3.
    Spi3 = 3,
    /// UART0.
    Uart0 = 4,
    /// UART1.
    Uart1 = 5,
    /// UART2.
    Uart2 = 6,
    /// I2C0.
    I2c0 = 7,
    /// I2C1.
    I2c1 = 8,
    /// I2C2.
    I2c2 = 9,
    /// CAN0.
    Can0 = 10,
    /// CAN1.
    Can1 = 11,
    /// The random number generator peripheral.
    Rng = 12,
    /// The ADC peripheral.
    Adc = 13,
    /// The DAC peripheral.
    Dac = 14,
    /// The DMA peripheral.
    Dma = 15,
    /// The external bus interface peripheral.
    Ebi = 16,
    /// The Ethernet peripheral.
    Eth = 17,
    /// The SpaceWire peripheral.
    Spw = 18,
    /// The clock generation peripheral.
    Clkgen = 19,
    /// The interrupt routing peripheral.
    IrqRouter = 20,
    /// IOCONFIG, the pin function select peripheral.
    IoConfig = 21,
    /// The utility peripheral.
    Utility = 22,
    /// The watchdog peripheral.
    Watchdog = 23,
    /// GPIO port A.
    PortA = 24,
    /// GPIO port B.
    PortB = 25,
    /// GPIO port C.
    PortC = 26,
    /// GPIO port D.
    PortD = 27,
    /// GPIO port E.
    PortE = 28,
    /// GPIO port F.
    PortF = 29,
    /// GPIO port G.
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
    /// Port A.
    A = 0,
    /// Port B.
    B = 1,
    /// Port C.
    #[cfg(feature = "vor4x")]
    C = 2,
    /// Port D.
    #[cfg(feature = "vor4x")]
    D = 3,
    /// Port E.
    #[cfg(feature = "vor4x")]
    E = 4,
    /// Port F.
    #[cfg(feature = "vor4x")]
    F = 5,
    /// Port G.
    #[cfg(feature = "vor4x")]
    G = 6,
}

impl Port {
    /// Number of valid pin offsets for this port.
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
    pub unsafe fn steal_regs(&self) -> gpio::regs::MmioGpio<'static> {
        gpio::regs::Gpio::new_mmio(*self)
    }
}

/// The given pin offset is out of range for the given [Port].
#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("invalid GPIO offset {offset} for port {port:?}")]
pub struct InvalidOffsetError {
    /// The offset which was out of range.
    offset: usize,
    /// The port the offset was checked against.
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
    /// Create a new interrupt configuration.
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

    /// FIFO trigger level, shared by the SPI and UART drivers.
    #[derive(Debug)]
    pub struct TriggerLevel(arbitrary_int::UInt<u32, 5>);

    impl TriggerLevel {
        /// Create a new trigger level.
        pub const fn new(value: u5) -> Self {
            TriggerLevel(arbitrary_int::UInt::<u32, 5>::new(value.value() as u32))
        }

        /// The raw trigger level value.
        pub const fn value(&self) -> u5 {
            u5::new(self.0.value() as u8)
        }
    }

    /// FIFO clear command, shared by the SPI and UART drivers.
    #[bitbybit::bitfield(u32, default = 0x0)]
    #[derive(Debug)]
    pub struct FifoClear {
        /// Clear the TX FIFO.
        #[bit(1, w)]
        tx_fifo: bool,
        /// Clear the RX FIFO.
        #[bit(0, w)]
        rx_fifo: bool,
    }

    impl FifoClear {
        /// Clears both the TX and RX FIFO.
        pub const ALL: Self = Self::builder()
            .with_tx_fifo(true)
            .with_rx_fifo(true)
            .build();
    }
}
