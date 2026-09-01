//! # API for the UART peripheral
//!
//! The core of this API are the [Uart], [Rx] and [Tx] structures.
//! The RX structure also has a dedicated [RxWithInterrupt] variant which allows reading the receiver
//! using interrupts.
//!
//! The [asynch] module provides an asynchronous, non-blocking TX driver. There is no equivalent
//! async RX driver: instead, drain [RxWithInterrupt::on_interrupt] into a queue of your choice
//! from your own interrupt handler (e.g. an `embassy_sync::pipe::Pipe`, which already gives you
//! an async `read`), and await that queue from your task. This is a handful of lines and gives
//! you full control over buffering; see [asynch] and the `async-uart-rx` examples for the exact
//! pattern.
//!
//! ## Examples
//!
//! - [UART simple example](https://egit.irs.uni-stuttgart.de/rust/vorago-rs/src/branch/main/va108xx/examples/simple/examples/uart.rs)
//! - [UART with IRQ and RTIC](https://egit.irs.uni-stuttgart.de/rust/vorago-rs/src/branch/main/va108xx/examples/rtic/src/bin/uart-echo-rtic.rs)
//! - [Flashloader exposing a CCSDS interface via UART](https://egit.irs.uni-stuttgart.de/rust/vorago-rs/src/branch/main/va108xx/flashloader)
use core::convert::Infallible;
/// Register definitions for the UART peripheral.
pub mod regs;
#[cfg(feature = "vor1x")]
use crate::InterruptConfig;
use crate::{
    FunctionSelect,
    gpio::{DynPinId, IoPeriphPin},
    pins::AnyPin,
    sealed::Sealed,
};
use arbitrary_int::{prelude::*, u6, u18};
use regs::{ClockScale, Control, Data, Enable, FifoClear, InterruptClear, MmioUart};

use crate::{PeripheralSelect, enable_nvic_interrupt, enable_peripheral_clock, time::Hertz};
use embedded_hal_nb::serial::Read;
pub use regs::{Bank, Stopbits, WordSize};

#[cfg(feature = "vor1x")]
mod pins_vor1x;
#[cfg(feature = "vor4x")]
mod pins_vor4x;

#[cfg(feature = "vor4x")]
use crate::clock::Clocks;
#[cfg(feature = "vor1x")]
use va108xx as pac;
#[cfg(feature = "vor4x")]
use va416xx as pac;

pub mod asynch;

/// FIFO depth of the UART for both the RX and TX FIFO.
pub const FIFO_DEPTH: usize = 16;

//==================================================================================================
// Type-Level support
//==================================================================================================

/// Marker trait for pins usable as the TX pin of UART0.
pub trait TxPin0: AnyPin {
    /// UART bank this pin belongs to.
    const BANK: Bank = Bank::Uart0;
    /// Alternate function to select to route this pin to the UART peripheral.
    const FUNC_SEL: FunctionSelect;
}
/// Marker trait for pins usable as the RX pin of UART0.
pub trait RxPin0: AnyPin {
    /// UART bank this pin belongs to.
    const BANK: Bank = Bank::Uart0;
    /// Alternate function to select to route this pin to the UART peripheral.
    const FUNC_SEL: FunctionSelect;
}

/// Marker trait for pins usable as the TX pin of UART1.
pub trait TxPin1: AnyPin {
    /// UART bank this pin belongs to.
    const BANK: Bank = Bank::Uart1;
    /// Alternate function to select to route this pin to the UART peripheral.
    const FUNC_SEL: FunctionSelect;
}
/// Marker trait for pins usable as the RX pin of UART1.
pub trait RxPin1: AnyPin {
    /// UART bank this pin belongs to.
    const BANK: Bank = Bank::Uart1;
    /// Alternate function to select to route this pin to the UART peripheral.
    const FUNC_SEL: FunctionSelect;
}

/// Marker trait for pins usable as the TX pin of UART2.
#[cfg(feature = "vor4x")]
pub trait TxPin2: AnyPin {
    /// UART bank this pin belongs to.
    const BANK: Bank = Bank::Uart2;
    /// Alternate function to select to route this pin to the UART peripheral.
    const FUNC_SEL: FunctionSelect;
}
/// Marker trait for pins usable as the RX pin of UART2.
#[cfg(feature = "vor4x")]
pub trait RxPin2: AnyPin {
    /// UART bank this pin belongs to.
    const BANK: Bank = Bank::Uart2;
    /// Alternate function to select to route this pin to the UART peripheral.
    const FUNC_SEL: FunctionSelect;
}

//==================================================================================================
// Regular Definitions
//==================================================================================================

/// No interrupt ID was configured for the given UART instance.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("no interrupt ID was set")]
pub struct NoInterruptIdWasSet;

/// A transfer is already pending and cannot be started again.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("transer is pending")]
pub struct TransferPendingError;

/// UART interrupt events, see [Uart::listen] and [Uart::unlisten].
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Event {
    /// Receiver FIFO interrupt enable. Generates interrupt
    /// when FIFO is at least half full. Half full is defined as FIFO
    /// count >= RXFIFOIRQTRG
    RxFifoHalfFull,
    /// Framing error, Overrun error, Parity Error and Break error
    RxError,
    /// Event for timeout condition: Data in the FIFO and no receiver
    /// FIFO activity for 4 character times
    RxTimeout,

    /// Transmitter FIFO interrupt enable. Generates interrupt
    /// when FIFO is at least half full. Half full is defined as FIFO
    /// count >= TXFIFOIRQTRG
    TxFifoHalfFull,
    /// FIFO overflow error
    TxError,
    /// Generate interrupt when transmit FIFO is empty and TXBUSY is 0
    TxEmpty,
    /// Interrupt when CTSn changes value
    TxCts,
}

/// Baud clock mode, see [ClockConfig::calculate].
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BaudMode {
    /// Default 16x baud clock.
    #[default]
    _16 = 0,
    /// Slower 8x baud clock.
    _8 = 1,
}

impl BaudMode {
    /// Baud clock multiplier for this mode.
    pub const fn multiplier(&self) -> u32 {
        match self {
            BaudMode::_16 => 16,
            BaudMode::_8 => 8,
        }
    }
}

/// UART parity configuration.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Parity {
    /// No parity bit.
    None,
    /// Odd parity.
    Odd,
    /// Even parity.
    Even,
}

/// UART baud rate clock configuration.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClockConfig {
    /// Integer divisor.
    pub div: u18,
    /// Fractional divide value in 1/64 units.
    pub frac: u6,
    /// Baud clock mode.
    pub baud_mode: BaudMode,
}

impl ClockConfig {
    /// Calculate the clock configuration for the given reference clock and target baudrate.
    pub const fn calculate(ref_clk: Hertz, baudrate: Hertz, baud_mode: BaudMode) -> Self {
        // This is the calculation: (64.0 * (x - integer_part as f32) + 0.5) as u32 without floating
        // point calculations.
        let multiplier = baud_mode.multiplier();
        let frac = ((ref_clk.to_raw() % (baudrate.to_raw() * multiplier)) * 64
            + (baudrate.to_raw() * (multiplier / 2)))
            / (baudrate.to_raw() * multiplier);
        // Calculations here are derived from chapter 4.8.5 (p.79) of the datasheet.
        let integer_div = ref_clk.to_raw() / (baudrate.to_raw() * multiplier);
        Self {
            frac: u6::new(frac as u8),
            div: u18::new(integer_div),
            baud_mode,
        }
    }

    /// Calculate the clock configuration for the given UART bank's reference clock and target
    /// baudrate.
    #[cfg(feature = "vor4x")]
    pub fn calculate_with_clocks(
        uart_id: Bank,
        clks: &Clocks,
        baudrate: Hertz,
        baud_mode: BaudMode,
    ) -> Self {
        let clk = if uart_id == Bank::Uart2 {
            clks.apb1()
        } else {
            clks.apb2()
        };
        Self::calculate(clk, baudrate, baud_mode)
    }
}

/// Configuration for [Uart::new_for_uart0] and related constructors.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// Clock configuration, determining the baudrate.
    pub clock_config: ClockConfig,
    /// Parity configuration.
    pub parity: Parity,
    /// Number of stop bits.
    pub stopbits: Stopbits,
    /// Word size.
    pub wordsize: WordSize,
    /// Enable the transmitter.
    pub enable_tx: bool,
    /// Enable the receiver.
    pub enable_rx: bool,
}

impl Config {
    /// Create a new configuration with the given clock configuration and the remaining fields
    /// set to their default values.
    pub fn new_with_clock_config(clock_config: ClockConfig) -> Self {
        Config {
            clock_config,
            parity: Parity::None,
            stopbits: Stopbits::One,
            wordsize: WordSize::Eight,
            enable_tx: true,
            enable_rx: true,
        }
    }

    /// Set the clock configuration.
    pub fn with_clock_config(mut self, clock_config: ClockConfig) -> Self {
        self.clock_config = clock_config;
        self
    }

    /// Disable parity.
    pub fn with_parity_none(mut self) -> Self {
        self.parity = Parity::None;
        self
    }

    /// Use even parity.
    pub fn with_parity_even(mut self) -> Self {
        self.parity = Parity::Even;
        self
    }

    /// Use odd parity.
    pub fn with_parity_odd(mut self) -> Self {
        self.parity = Parity::Odd;
        self
    }

    /// Set the number of stop bits.
    pub fn with_stopbits(mut self, stopbits: Stopbits) -> Self {
        self.stopbits = stopbits;
        self
    }

    /// Set the word size.
    pub fn with_wordsize(mut self, wordsize: WordSize) -> Self {
        self.wordsize = wordsize;
        self
    }
}

//==================================================================================================
// IRQ Definitions
//==================================================================================================

/// State tracked across interrupts for
/// [RxWithInterrupt::on_interrupt_max_size_or_timeout_based].
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InterruptContextTimeoutOrMaxSize {
    rx_idx: usize,
    mode: InterruptReceptionMode,
    /// Maximum length of the packet to receive.
    pub max_len: usize,
}

impl InterruptContextTimeoutOrMaxSize {
    /// Create a new context for a packet with the given maximum length.
    pub fn new(max_len: usize) -> Self {
        InterruptContextTimeoutOrMaxSize {
            rx_idx: 0,
            max_len,
            mode: InterruptReceptionMode::Idle,
        }
    }
}

impl InterruptContextTimeoutOrMaxSize {
    /// Reset the context to start receiving a new packet.
    pub fn reset(&mut self) {
        self.rx_idx = 0;
        self.mode = InterruptReceptionMode::Idle;
    }
}

/// This struct is used to return the default IRQ handler result to the user
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InterruptResult {
    /// Number of bytes read.
    pub bytes_read: usize,
    /// Receiver errors encountered, if any.
    pub errors: Option<UartErrors>,
}

/// This struct is used to return the default IRQ handler result to the user
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InterruptResultMaxSizeOrTimeout {
    complete: bool,
    timeout: bool,
    /// Receiver errors encountered, if any.
    pub errors: Option<UartErrors>,
    /// Number of bytes read so far.
    pub bytes_read: usize,
}

impl InterruptResultMaxSizeOrTimeout {
    /// Create a new, empty result.
    pub fn new() -> Self {
        InterruptResultMaxSizeOrTimeout {
            complete: false,
            timeout: false,
            errors: None,
            bytes_read: 0,
        }
    }
}
impl InterruptResultMaxSizeOrTimeout {
    /// Whether any receiver errors were encountered.
    #[inline]
    pub fn has_errors(&self) -> bool {
        self.errors.is_some()
    }

    /// Whether an overflow error was encountered.
    #[inline]
    pub fn overflow_error(&self) -> bool {
        self.errors.is_some_and(|e| e.overflow)
    }

    /// Whether a framing error was encountered.
    #[inline]
    pub fn framing_error(&self) -> bool {
        self.errors.is_some_and(|e| e.framing)
    }

    /// Whether a parity error was encountered.
    #[inline]
    pub fn parity_error(&self) -> bool {
        self.errors.is_some_and(|e| e.parity)
    }

    /// Whether the transfer completed due to a hardware timeout.
    #[inline]
    pub fn timeout(&self) -> bool {
        self.timeout
    }

    /// Whether the transfer completed.
    #[inline]
    pub fn complete(&self) -> bool {
        self.complete
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum InterruptReceptionMode {
    Idle,
    Pending,
}

/// Receiver error flags.
#[derive(Default, Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UartErrors {
    overflow: bool,
    framing: bool,
    parity: bool,
    other: bool,
}

impl UartErrors {
    /// FIFO overflow error.
    #[inline(always)]
    pub fn overflow(&self) -> bool {
        self.overflow
    }

    /// Framing error.
    #[inline(always)]
    pub fn framing(&self) -> bool {
        self.framing
    }

    /// Parity error.
    #[inline(always)]
    pub fn parity(&self) -> bool {
        self.parity
    }

    /// Any other error.
    #[inline(always)]
    pub fn other(&self) -> bool {
        self.other
    }
}

impl UartErrors {
    /// Whether any error is set.
    #[inline(always)]
    pub fn error(&self) -> bool {
        self.overflow || self.framing || self.parity || self.other
    }
}

/// The provided buffer is shorter than the maximum expected packet length.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BufferTooShortError {
    found: usize,
    expected: usize,
}

//==================================================================================================
// UART peripheral wrapper
//==================================================================================================

/// Common trait implemented by the PAC peripheral access structure for UART0.
pub trait Uart0Instance: Sealed {
    /// UART bank of the peripheral.
    const ID: Bank = Bank::Uart0;
    /// Peripheral selector used for clock and reset control.
    const PERIPH_SEL: PeripheralSelect;
}

/// Common trait implemented by the PAC peripheral access structure for UART1.
pub trait Uart1Instance: Sealed {
    /// UART bank of the peripheral.
    const ID: Bank = Bank::Uart1;
    /// Peripheral selector used for clock and reset control.
    const PERIPH_SEL: PeripheralSelect;
}

/// Common trait implemented by the PAC peripheral access structure for UART2.
#[cfg(feature = "vor4x")]
pub trait Uart2Instance: Sealed {
    /// UART bank of the peripheral.
    const ID: Bank = Bank::Uart2;
    /// Peripheral selector used for clock and reset control.
    const PERIPH_SEL: PeripheralSelect;
}

/// UART0 peripheral instance.
#[cfg(feature = "vor1x")]
pub type Uart0 = pac::Uarta;
/// UART0 peripheral instance.
#[cfg(feature = "vor4x")]
pub type Uart0 = pac::Uart0;

impl Uart0Instance for Uart0 {
    const ID: Bank = Bank::Uart0;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Uart0;
}
impl Sealed for Uart0 {}

/// UART1 peripheral instance.
#[cfg(feature = "vor1x")]
pub type Uart1 = pac::Uartb;
/// UART1 peripheral instance.
#[cfg(feature = "vor4x")]
pub type Uart1 = pac::Uart1;

impl Uart1Instance for Uart1 {
    const ID: Bank = Bank::Uart1;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Uart1;
}
impl Sealed for Uart1 {}

#[cfg(feature = "vor4x")]
impl Uart2Instance for pac::Uart2 {
    const ID: Bank = Bank::Uart2;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Uart2;
}
#[cfg(feature = "vor4x")]
impl Sealed for pac::Uart2 {}

//==================================================================================================
// UART implementation
//==================================================================================================

/// UART driver structure.
pub struct Uart {
    tx: Tx,
    rx: Rx,
}

impl Uart {
    cfg_if::cfg_if! {
        if #[cfg(feature = "vor1x")] {
            /// Calls [Self::new_for_uart0] with the interrupt configuration to some valid value.
            pub fn new_with_interrupt_uart0<Uart: Uart0Instance, Tx: TxPin0, Rx: RxPin0>(
                uart: Uart,
                tx_pin: Tx,
                rx_pin: Rx,
                config: Config,
                irq_cfg: InterruptConfig,
            ) -> Self {
                Self::new_for_uart0(uart, tx_pin, rx_pin, config, Some(irq_cfg))
            }

            /// Calls [Self::new_for_uart1] with the interrupt configuration to some valid value.
            pub fn new_with_interrupt_uart1<Uart: Uart1Instance, Tx: TxPin1, Rx: RxPin1>(
                uart: Uart,
                tx_pin: Tx,
                rx_pin: Rx,
                config: Config,
                irq_cfg: InterruptConfig,
            ) -> Self {
                Self::new_for_uart1(uart, tx_pin, rx_pin, config, Some(irq_cfg))
            }

            /// Calls [Self::new_for_uart0] with the interrupt configuration to [None].
            pub fn new_without_interrupt_uart0<Uart: Uart0Instance, Tx: TxPin0, Rx: RxPin0>(
                uart: Uart,
                tx_pin: Tx,
                rx_pin: Rx,
                config: Config,
            ) -> Self {
                Self::new_for_uart0(uart, tx_pin, rx_pin, config, None)
            }

            /// Calls [Self::new_for_uart1] with the interrupt configuration to [None].
            pub fn new_without_interrupt_uart1<Uart: Uart1Instance, Tx: TxPin1, Rx: RxPin1>(
                uart: Uart,
                tx_pin: Tx,
                rx_pin: Rx,
                config: Config,
            ) -> Self {
                Self::new_for_uart1(uart, tx_pin, rx_pin, config, None)
            }

            /// Create a new UART peripheral driver with an interrupt configuration.
            ///
            /// # Arguments
            ///
            /// - `syscfg`: The system configuration register block
            /// - `sys_clk`: The system clock frequency
            /// - `uart`: The concrete UART peripheral instance.
            /// - `pins`: UART TX and RX pin tuple.
            /// - `config`: UART specific configuration parameters like baudrate.
            /// - `irq_cfg`: Optional interrupt configuration. This should be a valid value if the plan
            ///   is to use TX or RX functionality relying on interrupts. If only the blocking API without
            ///   any interrupt support is used, this can be [None].
            pub fn new_for_uart0<Uart: Uart0Instance, Tx: TxPin0, Rx: RxPin0>(
                _uart: Uart,
                _tx_pin: Tx,
                _rx_pin: Rx,
                config: Config,
                opt_irq_cfg: Option<InterruptConfig>,
            ) -> Self {
                Self::new_internal(
                    Uart::PERIPH_SEL,
                    Uart::ID,
                    Tx::ID,
                    Tx::FUNC_SEL,
                    Rx::ID,
                    Rx::FUNC_SEL,
                    config,
                    opt_irq_cfg
                )
            }

            /// Create a new UART peripheral driver with an interrupt configuration.
            ///
            /// # Arguments
            ///
            /// - `syscfg`: The system configuration register block
            /// - `sys_clk`: The system clock frequency
            /// - `uart`: The concrete UART peripheral instance.
            /// - `pins`: UART TX and RX pin tuple.
            /// - `config`: UART specific configuration parameters like baudrate.
            /// - `irq_cfg`: Optional interrupt configuration. This should be a valid value if the plan
            ///   is to use TX or RX functionality relying on interrupts. If only the blocking API without
            ///   any interrupt support is used, this can be [None].
            pub fn new_for_uart1<Uart: Uart1Instance, Tx: TxPin1, Rx: RxPin1>(
                _uart: Uart,
                _tx_pin: Tx,
                _rx_pin: Rx,
                config: Config,
                opt_irq_cfg: Option<InterruptConfig>,
            ) -> Self {
                Self::new_internal(
                    Uart::PERIPH_SEL,
                    Uart::ID,
                    Tx::ID,
                    Tx::FUNC_SEL,
                    Rx::ID,
                    Rx::FUNC_SEL,
                    config,
                    opt_irq_cfg
                )
            }
        } else if #[cfg(feature = "vor4x")] {
            /// Create a new UART peripheral driver for UART 0.
            ///
            /// # Arguments
            ///
            /// - `clks`: Frozen system clock configuration.
            /// - `uart`: The concrete UART peripheral instance.
            /// - `pins`: UART TX and RX pin tuple.
            /// - `config`: UART specific configuration parameters like baudrate.
            pub fn new_for_uart0<Uart: Uart0Instance, Tx: TxPin0, Rx: RxPin0>(
                _uart: Uart,
                _tx_pin: Tx,
                _rx_pin: Rx,
                config: Config,
            ) -> Self {
                Self::new_internal(
                    Uart::PERIPH_SEL,
                    Uart::ID,
                    Tx::ID,
                    Tx::FUNC_SEL,
                    Rx::ID,
                    Rx::FUNC_SEL,
                    config
                )
            }

            /// Create a new UART peripheral driver for UART 1.
            ///
            /// # Arguments
            ///
            /// - `clks`: Frozen system clock configuration.
            /// - `uart`: The concrete UART peripheral instance.
            /// - `pins`: UART TX and RX pin tuple.
            /// - `config`: UART specific configuration parameters like baudrate.
            pub fn new_for_uart1<Uart: Uart1Instance, Tx: TxPin1, Rx: RxPin1>(
                _uart: Uart,
                _tx_pin: Tx,
                _rx_pin: Rx,
                config: Config,
            ) -> Self {
                Self::new_internal(
                    Uart::PERIPH_SEL,
                    Uart::ID,
                    Tx::ID,
                    Tx::FUNC_SEL,
                    Rx::ID,
                    Rx::FUNC_SEL,
                    config
                )
            }

            /// Create a new UART peripheral driver for UART 2.
            ///
            /// # Arguments
            ///
            /// - `clks`: Frozen system clock configuration.
            /// - `uart`: The concrete UART peripheral instance.
            /// - `pins`: UART TX and RX pin tuple.
            /// - `config`: UART specific configuration parameters like baudrate.
            pub fn new_for_uart2<Uart: Uart2Instance, Tx: TxPin2, Rx: RxPin2>(
                _uart: Uart,
                _tx_pin: Tx,
                _rx_pin: Rx,
                config: Config,
            ) -> Self {
                Self::new_internal(
                    Uart::PERIPH_SEL,
                    Uart::ID,
                    Tx::ID,
                    Tx::FUNC_SEL,
                    Rx::ID,
                    Rx::FUNC_SEL,
                    config
                )
            }

            /// Create a new UART peripheral driver given a reference clock with UART 0.
            ///
            /// # Arguments
            ///
            /// - `ref_clk`: APB1 clock for UART2, APB2 clock otherwise.
            /// - `uart`: The concrete UART peripheral instance.
            /// - `pins`: UART TX and RX pin tuple.
            /// - `config`: UART specific configuration parameters like baudrate.
            pub fn new_with_ref_clk_uart0<Uart: Uart0Instance, Tx: TxPin0, Rx: RxPin0>(
                _uart: Uart,
                _tx_pin: Tx,
                _rx_pin: Rx,
                config: Config,
            ) -> Self {
                Self::new_internal(
                    Uart::PERIPH_SEL,
                    Uart::ID,
                    Tx::ID,
                    Tx::FUNC_SEL,
                    Rx::ID,
                    Rx::FUNC_SEL,
                    config
                )
            }

            /// Create a new UART peripheral driver given a reference clock with UART 1.
            ///
            /// # Arguments
            ///
            /// - `ref_clk`: APB1 clock for UART2, APB2 clock otherwise.
            /// - `uart`: The concrete UART peripheral instance.
            /// - `pins`: UART TX and RX pin tuple.
            /// - `config`: UART specific configuration parameters like baudrate.
            pub fn new_with_ref_clk_uart1<Uart: Uart1Instance, Tx: TxPin1, Rx: RxPin1>(
                _uart: Uart,
                _tx_pin: Tx,
                _rx_pin: Rx,
                config: Config,
            ) -> Self {
                Self::new_internal(
                    Uart::PERIPH_SEL,
                    Uart::ID,
                    Tx::ID,
                    Tx::FUNC_SEL,
                    Rx::ID,
                    Rx::FUNC_SEL,
                    config
                )
            }

            /// Create a new UART peripheral driver given a reference clock with UART 2.
            ///
            /// # Arguments
            ///
            /// - `ref_clk`: APB1 clock for UART2, APB2 clock otherwise.
            /// - `uart`: The concrete UART peripheral instance.
            /// - `pins`: UART TX and RX pin tuple.
            /// - `config`: UART specific configuration parameters like baudrate.
            pub fn new_with_ref_clk_uart2<Uart: Uart2Instance, Tx: TxPin2, Rx: RxPin2>(
                _uart: Uart,
                _tx_pin: Tx,
                _rx_pin: Rx,
                config: Config,
            ) -> Self {
                Self::new_internal(
                    Uart::PERIPH_SEL,
                    Uart::ID,
                    Tx::ID,
                    Tx::FUNC_SEL,
                    Rx::ID,
                    Rx::FUNC_SEL,
                    config
                )
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn new_internal(
        periph_sel: PeripheralSelect,
        uart_bank: Bank,
        tx_pin_id: DynPinId,
        tx_func_sel: FunctionSelect,
        rx_pin_id: DynPinId,
        rx_func_sel: FunctionSelect,
        config: Config,
        #[cfg(feature = "vor1x")] opt_irq_cfg: Option<InterruptConfig>,
    ) -> Self {
        IoPeriphPin::new(tx_pin_id, tx_func_sel, None);
        IoPeriphPin::new(rx_pin_id, rx_func_sel, None);
        enable_peripheral_clock(periph_sel);

        let mut reg_block = regs::Uart::new_mmio(uart_bank);
        reg_block.write_clkscale(
            ClockScale::builder()
                .with_int(config.clock_config.div)
                .with_frac(config.clock_config.frac)
                .build(),
        );

        let (paren, pareven) = match config.parity {
            Parity::None => (false, false),
            Parity::Odd => (true, false),
            Parity::Even => (true, true),
        };
        reg_block.write_control(
            Control::builder()
                .with_baud8(config.clock_config.baud_mode == BaudMode::_8)
                .with_auto_rts(false)
                .with_def_rts(false)
                .with_auto_cts(false)
                .with_loopback_block(false)
                .with_loopback(false)
                .with_wordsize(config.wordsize)
                .with_stopbits(config.stopbits)
                .with_parity_manual(false)
                .with_parity_even(pareven)
                .with_parity_enable(paren)
                .build(),
        );
        // Clear the FIFO
        reg_block.write_fifo_clr(FifoClear::builder().with_tx(true).with_rx(true).build());
        reg_block.write_enable(
            Enable::builder()
                .with_tx(config.enable_tx)
                .with_rx(config.enable_rx)
                .build(),
        );

        #[cfg(feature = "vor1x")]
        if let Some(irq_cfg) = opt_irq_cfg {
            if irq_cfg.route {
                enable_peripheral_clock(PeripheralSelect::Irqsel);
                unsafe { va108xx::Irqsel::steal() }
                    .uart(uart_bank as usize)
                    .write(|w| unsafe { w.bits(irq_cfg.id as u32) });
            }
            if irq_cfg.enable_in_nvic {
                // Safety: User has specifically configured this.
                unsafe { enable_nvic_interrupt(irq_cfg.id) };
            }
        }

        Uart {
            tx: Tx::new(uart_bank),
            rx: Rx::new(uart_bank),
        }
    }

    /// Read the peripheral ID register.
    #[inline]
    pub fn peripheral_id(&self) -> u32 {
        self.tx.perid()
    }

    /// Enable the receiver.
    #[inline]
    pub fn enable_rx(&mut self) {
        self.rx.enable();
    }

    /// Disable the receiver.
    #[inline]
    pub fn disable_rx(&mut self) {
        self.rx.disable();
    }

    /// Enable the transmitter.
    #[inline]
    pub fn enable_tx(&mut self) {
        self.tx.enable();
    }

    /// Disable the transmitter.
    #[inline]
    pub fn disable_tx(&mut self) {
        self.tx.disable();
    }

    /// This also clears status conditons for the RX FIFO.
    #[inline]
    pub fn clear_rx_fifo(&mut self) {
        self.rx.clear_fifo();
    }

    /// This also clears status conditons for the TX FIFO.
    #[inline]
    pub fn clear_tx_fifo(&mut self) {
        self.tx.clear_fifo();
    }

    /// Enable the interrupt for the given event.
    pub fn listen(&mut self, event: Event) {
        self.tx.regs.modify_interrupt_enable(|mut value| {
            match event {
                Event::RxError => value.set_rx_status(true),
                Event::RxFifoHalfFull => value.set_rx(true),
                Event::RxTimeout => value.set_rx_timeout(true),
                Event::TxEmpty => value.set_tx_empty(true),
                Event::TxError => value.set_tx_status(true),
                Event::TxFifoHalfFull => value.set_tx_below_trigger(true),
                Event::TxCts => value.set_tx_cts(true),
            }
            value
        });
    }

    /// Disable the interrupt for the given event.
    pub fn unlisten(&mut self, event: Event) {
        self.tx.regs.modify_interrupt_enable(|mut value| {
            match event {
                Event::RxError => value.set_rx_status(false),
                Event::RxFifoHalfFull => value.set_rx(false),
                Event::RxTimeout => value.set_rx_timeout(false),
                Event::TxEmpty => value.set_tx_empty(false),
                Event::TxError => value.set_tx_status(false),
                Event::TxFifoHalfFull => value.set_tx_below_trigger(false),
                Event::TxCts => value.set_tx_cts(false),
            }
            value
        });
    }

    /// Poll receiver errors.
    pub fn poll_rx_errors(&self) -> Option<UartErrors> {
        self.rx.poll_errors()
    }

    /// Split the driver into its transmitter and receiver halves.
    pub fn split(self) -> (Tx, Rx) {
        (self.tx, self.rx)
    }
}

impl embedded_io::ErrorType for Uart {
    type Error = Infallible;
}

impl embedded_hal_nb::serial::ErrorType for Uart {
    type Error = Infallible;
}

impl embedded_hal_nb::serial::Read<u8> for Uart {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.rx.read()
    }
}

impl embedded_hal_nb::serial::Write<u8> for Uart {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.tx.write(word).map_err(|e| {
            if let nb::Error::Other(_) = e {
                unreachable!()
            }
            nb::Error::WouldBlock
        })
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.tx.flush().map_err(|e| {
            if let nb::Error::Other(_) = e {
                unreachable!()
            }
            nb::Error::WouldBlock
        })
    }
}

/// Enable the receiver on the given register block.
#[inline(always)]
pub fn enable_rx(uart: &mut MmioUart<'static>) {
    uart.modify_enable(|mut value| {
        value.set_rx(true);
        value
    });
}

/// Disable the receiver on the given register block.
#[inline(always)]
pub fn disable_rx(uart: &mut MmioUart<'static>) {
    uart.modify_enable(|mut value| {
        value.set_rx(false);
        value
    });
}

/// Enable the RX interrupts on the given register block.
#[inline(always)]
pub fn enable_rx_interrupts(uart: &mut MmioUart<'static>, timeout: bool) {
    uart.modify_interrupt_enable(|mut value| {
        value.set_rx_status(true);
        value.set_rx(true);
        if timeout {
            value.set_rx_timeout(true);
        }
        value
    });
}

/// Disable the RX interrupts on the given register block.
#[inline(always)]
pub fn disable_rx_interrupts(uart: &mut MmioUart<'static>) {
    uart.modify_interrupt_enable(|mut value| {
        value.set_rx_status(false);
        value.set_rx(false);
        value.set_rx_timeout(false);
        value
    });
}

/// Serial receiver.
///
/// Can be created by using the [Uart::split] API.
pub struct Rx {
    id: Bank,
    regs: regs::MmioUart<'static>,
}

impl core::fmt::Debug for Rx {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Rx")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

impl Rx {
    /// Retrieve a TX pin without expecting an explicit UART structure
    ///
    /// # Safety
    ///
    /// Circumvents the HAL safety guarantees.
    #[inline(always)]
    pub unsafe fn steal(id: Bank) -> Self {
        Self::new(id)
    }

    #[inline(always)]
    fn new(id: Bank) -> Self {
        Self {
            id,
            regs: regs::Uart::new_mmio(id),
        }
    }

    /// Poll and clear receiver errors.
    pub fn poll_errors(&self) -> Option<UartErrors> {
        let mut errors = UartErrors::default();

        let status = self.regs.read_rx_status();
        if status.overrun_error() {
            errors.overflow = true;
        } else if status.framing_error() {
            errors.framing = true;
        } else if status.parity_error() {
            errors.parity = true;
        } else {
            return None;
        };
        Some(errors)
    }

    /// Read the peripheral ID register.
    #[inline]
    pub fn perid(&self) -> u32 {
        self.regs.read_perid()
    }

    /// Clear the RX FIFO.
    #[inline]
    pub fn clear_fifo(&mut self) {
        self.regs
            .write_fifo_clr(FifoClear::builder().with_tx(false).with_rx(true).build());
    }

    /// Disable the RX interrupts.
    #[inline]
    pub fn disable_interrupts(&mut self) {
        disable_rx_interrupts(&mut self.regs);
    }

    /// Enable the RX interrupts.
    #[inline]
    pub fn enable_interrupts(
        &mut self,
        #[cfg(feature = "vor4x")] enable_in_nvic: bool,
        timeout: bool,
    ) {
        #[cfg(feature = "vor4x")]
        if enable_in_nvic {
            unsafe {
                enable_nvic_interrupt(self.id.interrupt_id_rx());
            }
        }
        enable_rx_interrupts(&mut self.regs, timeout);
    }

    /// Enable the receiver.
    #[inline]
    pub fn enable(&mut self) {
        enable_rx(&mut self.regs);
    }

    /// Disable the receiver.
    #[inline]
    pub fn disable(&mut self) {
        disable_rx(&mut self.regs);
    }

    /// Low level function to read a word from the UART FIFO.
    ///
    /// Uses the [nb] API to allow usage in blocking and non-blocking contexts.
    ///
    /// Please note that you might have to mask the returned value with 0xff to retrieve the actual
    /// value if you use the manual parity mode. See chapter 4.6.2 for more information.
    #[inline(always)]
    pub fn read_fifo(&mut self) -> nb::Result<u32, Infallible> {
        if !self.regs.read_rx_status().data_available() {
            return Err(nb::Error::WouldBlock);
        }
        Ok(self.read_fifo_unchecked())
    }

    /// Low level function to read a word from from the UART FIFO.
    ///
    /// This does not necesarily mean there is a word in the FIFO available.
    /// Use the [Self::read_fifo] function to read a word from the FIFO reliably using the [nb]
    /// API.
    ///
    /// Please note that you might have to mask the returned value with 0xff to retrieve the actual
    /// value if you use the manual parity mode. See chapter 4.6.2 for more information.
    #[inline(always)]
    pub fn read_fifo_unchecked(&mut self) -> u32 {
        self.regs.read_data().raw_value()
    }

    /// Convert this driver into an interrupt-driven receiver.
    #[inline]
    pub fn into_rx_with_interrupt(self) -> RxWithInterrupt {
        RxWithInterrupt::new(self)
    }

    /// Convert this driver into an interrupt-driven receiver.
    #[deprecated(since = "0.3.0", note = "Use into_rx_with_interrupt instead")]
    #[inline]
    pub fn into_rx_with_irq(self) -> RxWithInterrupt {
        RxWithInterrupt::new(self)
    }
}

impl embedded_io::ErrorType for Rx {
    type Error = Infallible;
}

impl embedded_hal_nb::serial::ErrorType for Rx {
    type Error = Infallible;
}

impl embedded_hal_nb::serial::Read<u8> for Rx {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.read_fifo().map(|val| (val & 0xff) as u8).map_err(|e| {
            if let nb::Error::Other(_) = e {
                unreachable!()
            }
            nb::Error::WouldBlock
        })
    }
}

impl embedded_io::Read for Rx {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        let mut read = 0;
        loop {
            if self.regs.read_rx_status().data_available() {
                break;
            }
        }
        for byte in buf.iter_mut() {
            match <Self as embedded_hal_nb::serial::Read<u8>>::read(self) {
                Ok(w) => {
                    *byte = w;
                    read += 1;
                }
                Err(nb::Error::WouldBlock) => break,
            }
        }

        Ok(read)
    }
}

/// Enable the transmitter on the given register block.
#[inline(always)]
pub fn enable_tx(uart: &mut MmioUart<'static>) {
    uart.modify_enable(|mut value| {
        value.set_tx(true);
        value
    });
}

/// Disable the transmitter on the given register block.
#[inline(always)]
pub fn disable_tx(uart: &mut MmioUart<'static>) {
    uart.modify_enable(|mut value| {
        value.set_tx(false);
        value
    });
}

/// Enable the TX interrupts on the given register block.
#[inline(always)]
pub fn enable_tx_interrupts(tx_below_trigger: bool, uart: &mut MmioUart<'static>) {
    uart.modify_interrupt_enable(|mut value| {
        value.set_tx_below_trigger(tx_below_trigger);
        value.set_tx_empty(true);
        value.set_tx_status(true);
        value
    });
}

/// Disable the TX interrupts on the given register block.
#[inline(always)]
pub fn disable_tx_interrupts(uart: &mut MmioUart<'static>) {
    uart.modify_interrupt_enable(|mut value| {
        value.set_tx_below_trigger(false);
        value.set_tx_empty(false);
        value.set_tx_status(false);
        value
    });
}

/// Serial transmitter
///
/// Can be created by using the [Uart::split] API.
pub struct Tx {
    id: Bank,
    regs: regs::MmioUart<'static>,
}

impl core::fmt::Debug for Tx {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Tx")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

impl Tx {
    /// Retrieve a TX pin without expecting an explicit UART structure
    ///
    /// # Safety
    ///
    /// Circumvents the HAL safety guarantees.
    #[inline(always)]
    pub unsafe fn steal(id: Bank) -> Self {
        Self::new(id)
    }

    #[inline(always)]
    fn new(id: Bank) -> Self {
        Self {
            id,
            regs: regs::Uart::new_mmio(id),
        }
    }

    /// Read the peripheral ID register.
    #[inline]
    pub fn perid(&self) -> u32 {
        self.regs.read_perid()
    }

    /// Clear the TX FIFO.
    #[inline]
    pub fn clear_fifo(&mut self) {
        self.regs
            .write_fifo_clr(FifoClear::builder().with_tx(true).with_rx(false).build());
    }

    /// Enable the transmitter.
    #[inline]
    pub fn enable(&mut self) {
        self.regs.modify_enable(|mut value| {
            value.set_tx(true);
            value
        });
    }

    /// Disable the transmitter.
    #[inline]
    pub fn disable(&mut self) {
        self.regs.modify_enable(|mut value| {
            value.set_tx(false);
            value
        });
    }

    /// Enables the IRQ_TX, IRQ_TX_STATUS and IRQ_TX_EMPTY interrupts.
    ///
    /// - The IRQ_TX interrupt is generated when the TX FIFO is at least half empty and the
    ///   `tx_below_trigger` parameter is set to `true`. This should be set to true if the total
    ///   amount of data to be transmitted is larger than the FIFO size.
    /// - The IRQ_TX_STATUS interrupt is generated when write data is lost due to a FIFO overflow
    /// - The IRQ_TX_EMPTY interrupt is generated when the TX FIFO is empty and the TXBUSY signal
    ///   is 0
    #[inline]
    pub fn enable_interrupts(
        &mut self,
        tx_below_trigger: bool,
        #[cfg(feature = "vor4x")] enable_in_nvic: bool,
    ) {
        #[cfg(feature = "vor4x")]
        if enable_in_nvic {
            unsafe { enable_nvic_interrupt(self.id.interrupt_id_tx()) };
        }
        // Safety: We own the UART structure
        enable_tx_interrupts(tx_below_trigger, &mut self.regs);
    }

    /// Disables the IRQ_TX, IRQ_TX_STATUS and IRQ_TX_EMPTY interrupts.
    ///
    /// [Self::enable_interrupts] documents the interrupts.
    #[inline]
    pub fn disable_interrupts(&mut self) {
        // Safety: We own the UART structure
        disable_tx_interrupts(&mut self.regs);
    }

    /// Low level function to write a word to the UART FIFO.
    ///
    /// Uses the [nb] API to allow usage in blocking and non-blocking contexts.
    ///
    /// Please note that you might have to mask the returned value with 0xff to retrieve the actual
    /// value if you use the manual parity mode. See chapter 11.4.1 for more information.
    #[inline(always)]
    pub fn write_fifo(&mut self, data: u32) -> nb::Result<(), Infallible> {
        if !self.regs.read_tx_status().ready() {
            return Err(nb::Error::WouldBlock);
        }
        self.write_fifo_unchecked(data);
        Ok(())
    }

    /// Low level function to write a word to the UART FIFO.
    ///
    /// This does not necesarily mean that the FIFO can process another word because it might be
    /// full.
    /// Use the [Self::write_fifo] function to write a word to the FIFO reliably using the [nb]
    /// API.
    #[inline(always)]
    pub fn write_fifo_unchecked(&mut self, data: u32) {
        self.regs.write_data(Data::new_with_raw_value(data));
    }

    /// Create an asynchronous UART driver.
    ///
    /// See [asynch::Tx::new] for details.
    pub fn into_async(self) -> asynch::Tx {
        asynch::Tx::new(self)
    }
}

impl embedded_io::ErrorType for Tx {
    type Error = Infallible;
}

impl embedded_hal_nb::serial::ErrorType for Tx {
    type Error = Infallible;
}

impl embedded_hal_nb::serial::Write<u8> for Tx {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.write_fifo(word as u32)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // SAFETY: Only TX related registers are used.
        if self.regs.read_tx_status().write_busy() {
            return Err(nb::Error::WouldBlock);
        }
        Ok(())
    }
}

impl embedded_io::Write for Tx {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            if self.regs.read_tx_status().ready() {
                break;
            }
        }
        let mut written = 0;
        for byte in buf.iter() {
            match <Self as embedded_hal_nb::serial::Write<u8>>::write(self, *byte) {
                Ok(_) => written += 1,
                Err(nb::Error::WouldBlock) => return Ok(written),
            }
        }

        Ok(written)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        nb::block!(<Self as embedded_hal_nb::serial::Write<u8>>::flush(self))
    }
}

impl core::fmt::Write for Tx {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        <Self as embedded_io::Write>::write_all(self, s.as_bytes()).map_err(|_| core::fmt::Error)
    }
}

/// Serial receiver, using interrupts to offload reading to the hardware.
///
/// You can use [Rx::into_rx_with_irq] to convert a normal [Rx] structure into this structure.
/// This structure provides two distinct ways to read the UART RX using interrupts. It should
/// be noted that the interrupt service routine (ISR) still has to be provided by the user. However,
/// this structure provides API calls which can be used inside the ISRs to simplify the reading
/// of the UART.
///
///  1. The first way simply empties the FIFO on an interrupt into a user provided buffer. You
///     can simply use [Self::start] to prepare the peripheral and then call [Self::on_interrupt]
///     (bare interrupt handler with no owned instance, e.g. embassy's `#[interrupt] fn`) or
///     [Self::on_interrupt_owned] (owned instance, e.g. an RTIC `local` resource) in the
///     interrupt service routine.
///  2. The second way reads packets bounded by a maximum size or a baudtick based timeout. You
///     can use [Self::read_fixed_len_or_timeout_based_using_irq] to prepare the peripheral and
///     then call the [Self::on_interrupt_max_size_or_timeout_based] in the interrupt service
///     routine. You have to call [Self::read_fixed_len_or_timeout_based_using_irq] in the ISR to
///     start reading the next packet.
///
/// For continuous reception from an async task, call [Self::on_interrupt] from the ISR and
/// forward the drained bytes into a queue of your choice (e.g. an `embassy_sync::pipe::Pipe`),
/// then await that queue from the task. See the `async-uart-rx` examples.
pub struct RxWithInterrupt(Rx);

impl RxWithInterrupt {
    /// Wrap an [Rx] driver to enable interrupt-driven reception.
    #[inline]
    pub fn new(rx: Rx) -> Self {
        Self(rx)
    }

    /// Steal the RX peripheral with interrupt support for the given UART bank.
    ///
    /// Can be useful to retrieve an instance in an interrupt if this instance is not used in the
    /// main thread after initialization time.
    ///
    /// # Safety
    ///
    /// Circumvents the HAL ownership and safety guarantees.
    pub unsafe fn steal(id: Bank) -> Self {
        Self(unsafe { Rx::steal(id) })
    }

    /// This function should be called once at initialization time if the regular
    /// [Self::on_interrupt] is used to read the UART receiver to enable and start the receiver.
    pub fn start(&mut self) {
        #[cfg(feature = "vor4x")]
        self.enable_interrupts(true, true);
        #[cfg(feature = "vor1x")]
        self.enable_interrupts(true);
        self.0.enable();
    }

    /// The wrapped [Rx] driver.
    #[inline(always)]
    pub fn rx(&self) -> &Rx {
        &self.0
    }

    /// Token identifying the UART peripheral driven by this instance.
    ///
    /// Pass this to [Self::on_interrupt] to service the peripheral's RX interrupts from a bare
    /// interrupt handler that has no access to an owned instance (e.g. a plain
    /// `#[interrupt] fn ...()`, as opposed to an RTIC task with a `local` resource). Since it is
    /// `Copy`, stash it in a `Mutex<Cell<_>>`/`OnceCell` and hand it to the interrupt handler.
    #[inline]
    pub fn bank_id(&self) -> Bank {
        self.0.id
    }

    /// This function is used together with the [Self::on_interrupt_max_size_or_timeout_based]
    /// function to read packets with a maximum size or variable sized packets by using the
    /// receive timeout of the hardware.
    ///
    /// This function should be called once at initialization to initiate the context state
    /// and to [Self::start] the receiver. After that, it should be called after each
    /// completed [Self::on_interrupt_max_size_or_timeout_based] call to restart the reception
    /// of a packet.
    pub fn read_fixed_len_or_timeout_based_using_irq(
        &mut self,
        context: &mut InterruptContextTimeoutOrMaxSize,
    ) -> Result<(), TransferPendingError> {
        if context.mode != InterruptReceptionMode::Idle {
            return Err(TransferPendingError);
        }
        context.mode = InterruptReceptionMode::Pending;
        context.rx_idx = 0;
        self.start();
        Ok(())
    }

    #[inline]
    fn enable_interrupts(&mut self, #[cfg(feature = "vor4x")] enable_in_nvic: bool, timeout: bool) {
        #[cfg(feature = "vor4x")]
        self.0.enable_interrupts(enable_in_nvic, timeout);
        #[cfg(feature = "vor1x")]
        self.0.enable_interrupts(timeout);
    }

    #[inline]
    fn disable_interrupts(&mut self) {
        self.0.disable_interrupts();
    }

    /// Cancel the current reception, disabling interrupts and clearing the RX FIFO.
    pub fn cancel_transfer(&mut self) {
        self.disable_interrupts();
        self.0.clear_fifo();
    }

    /// Drains the RX FIFO into `buf`, using the token returned by [Self::bank_id] rather than an
    /// owned instance, so it can be called from a bare interrupt handler that was never handed
    /// this driver. See [Self::on_interrupt_owned] for the semantics; this performs the same
    /// operation on a freshly [Self::steal]ed instance.
    pub fn on_interrupt(bank_id: Bank, buf: &mut [u8; 16]) -> InterruptResult {
        // Safety: Only touches this bank's registers for the duration of this call, same as any
        // other interrupt handler in this crate.
        unsafe { Self::steal(bank_id) }.on_interrupt_owned(buf)
    }

    /// This function should be called in the user provided UART interrupt handler.
    ///
    /// It simply empties any bytes in the FIFO into the user provided buffer and returns the
    /// result of the operation.
    ///
    /// This function will not disable the RX interrupts, so you don't need to call any other
    /// API after calling this function to continue emptying the FIFO. RX errors are handled
    /// as partial errors and are returned as part of the [InterruptResult].
    ///
    /// Prefer [Self::on_interrupt] if you do not already have an owned instance available in
    /// your interrupt handler (e.g. an RTIC `local` resource); that takes a `Bank` token instead.
    pub fn on_interrupt_owned(&mut self, buf: &mut [u8; 16]) -> InterruptResult {
        let mut result = InterruptResult::default();

        let irq_status = self.0.regs.read_interrupt_status();
        let irq_enabled = self.0.regs.read_interrupt_enable();
        let rx_enabled = irq_enabled.rx();

        // Half-Full interrupt. We have a guaranteed amount of data we can read.
        if irq_status.rx() {
            let available_bytes = self.0.regs.read_rx_fifo_trigger().level().as_usize();

            // If this interrupt bit is set, the trigger level is available at the very least.
            // Read everything as fast as possible
            for _ in 0..available_bytes {
                buf[result.bytes_read] = (self.0.read_fifo_unchecked() & 0xff) as u8;
                result.bytes_read += 1;
            }
        }

        // Timeout, empty the FIFO completely.
        if irq_status.rx_timeout() {
            // While there is data in the FIFO, write it into the reception buffer
            while let Ok(byte) = self.0.read_fifo() {
                buf[result.bytes_read] = byte as u8;
                result.bytes_read += 1;
            }
        }

        // RX transfer not complete, check for RX errors
        if rx_enabled {
            self.check_for_errors(&mut result.errors);
        }

        // Clear the interrupt status bits
        self.0.regs.write_irq_clr(
            InterruptClear::builder()
                .with_rx_overrun(true)
                .with_tx_overrun(false)
                .build(),
        );
        result
    }

    /// This function should be called in the user provided UART interrupt handler.
    ///
    /// This function is used to read packets which either have a maximum size or variable sized
    /// packet which are bounded by sufficient delays between them, triggering a hardware timeout.
    ///
    /// If either the maximum number of packets have been read or a timeout occured, the transfer
    /// will be deemed completed. The state information of the transfer is tracked in the
    /// [InterruptContextTimeoutOrMaxSize] structure.
    ///
    /// If passed buffer is equal to or larger than the specified maximum length, an
    /// [BufferTooShortError] will be returned. Other RX errors are treated as partial errors
    /// and returned inside the [InterruptResultMaxSizeOrTimeout] structure.
    pub fn on_interrupt_max_size_or_timeout_based(
        &mut self,
        context: &mut InterruptContextTimeoutOrMaxSize,
        buf: &mut [u8],
    ) -> Result<InterruptResultMaxSizeOrTimeout, BufferTooShortError> {
        if buf.len() < context.max_len {
            return Err(BufferTooShortError {
                found: buf.len(),
                expected: context.max_len,
            });
        }
        let mut result = InterruptResultMaxSizeOrTimeout::default();

        let irq_status = self.0.regs.read_interrupt_status();
        let rx_enabled = self.0.regs.read_enable().rx();

        // Half-Full interrupt. We have a guaranteed amount of data we can read.
        if irq_status.rx() {
            // Determine the number of bytes to read, ensuring we leave 1 byte in the FIFO.
            // We use this trick/hack because the timeout feature of the peripheral relies on data
            // being in the RX FIFO. If data continues arriving, another half-full IRQ will fire.
            // If not, the last byte(s) is/are emptied by the timeout interrupt.
            let available_bytes = self.0.regs.read_rx_fifo_trigger().level().as_usize();

            let bytes_to_read = core::cmp::min(
                available_bytes.saturating_sub(1),
                context.max_len - context.rx_idx,
            );

            // If this interrupt bit is set, the trigger level is available at the very least.
            // Read everything as fast as possible
            for _ in 0..bytes_to_read {
                buf[context.rx_idx] = (self.0.read_fifo_unchecked() & 0xff) as u8;
                context.rx_idx += 1;
            }

            // On high-baudrates, data might be available immediately, and we possible have to
            // read continuosly? Then again, the CPU should always be faster than that. I'd rather
            // rely on the hardware firing another IRQ. I have not tried baudrates higher than
            // 115200 so far.
        }
        // Timeout, empty the FIFO completely.
        if irq_status.rx_timeout() {
            // While there is data in the FIFO, write it into the reception buffer
            loop {
                if context.rx_idx == context.max_len {
                    break;
                }
                // While there is data in the FIFO, write it into the reception buffer
                match self.0.read() {
                    Ok(byte) => {
                        buf[context.rx_idx] = byte;
                        context.rx_idx += 1;
                    }
                    Err(_) => break,
                }
            }
            self.irq_completion_handler_max_size_timeout(&mut result, context);
            return Ok(result);
        }

        // RX transfer not complete, check for RX errors
        if (context.rx_idx < context.max_len) && rx_enabled {
            self.check_for_errors(&mut result.errors);
        }

        // Clear the interrupt status bits
        self.0.regs.write_irq_clr(
            InterruptClear::builder()
                .with_rx_overrun(true)
                .with_tx_overrun(false)
                .build(),
        );
        Ok(result)
    }

    fn check_for_errors(&self, errors: &mut Option<UartErrors>) {
        let rx_status = self.0.regs.read_rx_status();

        if rx_status.overrun_error() || rx_status.framing_error() || rx_status.parity_error() {
            let err = errors.get_or_insert(UartErrors::default());

            if rx_status.overrun_error() {
                err.overflow = true;
            }
            if rx_status.framing_error() {
                err.framing = true;
            }
            if rx_status.parity_error() {
                err.parity = true;
            }
        }
    }

    fn irq_completion_handler_max_size_timeout(
        &mut self,
        res: &mut InterruptResultMaxSizeOrTimeout,
        context: &mut InterruptContextTimeoutOrMaxSize,
    ) {
        self.disable_interrupts();
        self.0.disable();
        res.bytes_read = context.rx_idx;
        res.complete = true;
        context.mode = InterruptReceptionMode::Idle;
        context.rx_idx = 0;
    }

    /// # Safety
    ///
    /// This API allows creating multiple UART instances when releasing the TX structure as well.
    /// The user must ensure that these instances are not used to create multiple overlapping
    /// UART drivers.
    pub unsafe fn release(mut self) -> Rx {
        self.disable_interrupts();
        self.0
    }
}
