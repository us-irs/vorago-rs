use core::marker::PhantomData;

cfg_if::cfg_if! {
    if #[cfg(feature = "vor1x")] {
        /// UART A base address
        pub const BASE_ADDR_0: usize = 0x4004_0000;
        /// UART B base address
        pub const BASE_ADDR_1: usize = 0x4004_1000;
    } else if #[cfg(feature = "vor4x")] {
        /// UART 0 base address
        pub const BASE_ADDR_0: usize = 0x4002_4000;
        /// UART 1 base address
        pub const BASE_ADDR_1: usize = 0x4002_5000;
        /// UART 2 base address
        pub const BASE_ADDR_2: usize = 0x4001_7000;
    }
}

/// UART peripheral bank.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bank {
    /// UART0.
    Uart0 = 0,
    /// UART1.
    Uart1 = 1,
    /// UART2.
    #[cfg(feature = "vor4x")]
    Uart2 = 2,
}

impl Bank {
    /// Unsafely steal the GPIO peripheral block for the given port.
    ///
    /// # Safety
    ///
    /// Circumvents ownership and safety guarantees by the HAL.
    pub unsafe fn steal_regs(&self) -> MmioRegisters<'static> {
        Registers::new_mmio(*self)
    }

    /// The interrupt vector used for TX interrupts on this bank.
    #[cfg(feature = "vor4x")]
    pub const fn interrupt_id_tx(&self) -> va416xx::Interrupt {
        match self {
            Bank::Uart0 => va416xx::Interrupt::UART0_TX,
            Bank::Uart1 => va416xx::Interrupt::UART1_TX,
            Bank::Uart2 => va416xx::Interrupt::UART2_TX,
        }
    }

    /// The interrupt vector used for RX interrupts on this bank.
    #[cfg(feature = "vor4x")]
    pub const fn interrupt_id_rx(&self) -> va416xx::Interrupt {
        match self {
            Bank::Uart0 => va416xx::Interrupt::UART0_RX,
            Bank::Uart1 => va416xx::Interrupt::UART1_RX,
            Bank::Uart2 => va416xx::Interrupt::UART2_RX,
        }
    }
}

pub use types::*;

/// Register helper types.
pub mod types {
    use arbitrary_int::{u5, u6, u18};

    /// DATA register, used to read from and write to the FIFOs.
    #[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
    pub struct Data {
        /// Manually computed parity bit, only used if manual parity mode is enabled.
        #[bit(15, rw)]
        dparity: bool,
        /// The data word.
        #[bits(0..=7, rw)]
        data: u8,
    }

    /// ENABLE register.
    #[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
    pub struct Enable {
        /// Enable the transmitter.
        #[bit(1, rw)]
        tx: bool,
        /// Enable the receiver.
        #[bit(0, rw)]
        rx: bool,
    }

    /// Number of stop bits.
    #[bitbybit::bitenum(u1, exhaustive = true)]
    #[derive(Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stopbits {
        /// One stop bit.
        One = 0,
        /// Two stop bits.
        Two = 1,
    }

    /// UART word size.
    #[bitbybit::bitenum(u2, exhaustive = true)]
    #[derive(Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WordSize {
        /// 5 bit words.
        Five = 0b00,
        /// 6 bit words.
        Six = 0b01,
        /// 7 bit words.
        Seven = 0b10,
        /// 8 bit words.
        Eight = 0b11,
    }

    /// CONTROL register.
    #[bitbybit::bitfield(u32, default = 0x0, debug, defmt_fields(feature = "defmt"))]
    pub struct Control {
        /// Use 8x oversampling instead of the default 16x for the baud rate generator.
        #[bit(11, rw)]
        baud8: bool,
        /// Automatic RTS flow control.
        #[bit(10, rw)]
        auto_rts: bool,
        /// Default RTS level.
        #[bit(9, rw)]
        def_rts: bool,
        /// Automatic CTS flow control.
        #[bit(8, rw)]
        auto_cts: bool,
        /// Block the transmitter while in loopback mode.
        #[bit(7, rw)]
        loopback_block: bool,
        /// Enable loopback mode.
        #[bit(6, rw)]
        loopback: bool,
        /// Word size.
        #[bits(4..=5, rw)]
        wordsize: WordSize,
        /// Number of stop bits.
        #[bit(3, rw)]
        stopbits: Stopbits,
        /// Compute parity manually instead of automatically.
        #[bit(2, rw)]
        parity_manual: bool,
        /// Use even parity instead of odd parity.
        #[bit(1, rw)]
        parity_even: bool,
        /// Enable parity checking.
        #[bit(0, rw)]
        parity_enable: bool,
    }

    /// CLKSCALE register, controlling the baud rate generator.
    #[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
    pub struct ClockScale {
        /// Integer part of the clock divisor.
        #[bits(6..=23, rw)]
        int: u18,
        /// Fractional part of the clock divisor.
        #[bits(0..=5, rw)]
        frac: u6,
    }

    /// RX_STATUS register.
    #[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
    pub struct RxStatus {
        /// Level of the RTSN line.
        #[bit(15, r)]
        rx_rtsn: bool,
        /// Address bit of the 9-bit word received, if used.
        #[bit(9, r)]
        rx_addr9: bool,
        /// A break condition is currently active.
        #[bit(8, r)]
        busy_break: bool,
        /// A break condition was detected.
        #[bit(7, r)]
        break_error: bool,
        /// A parity error was detected.
        #[bit(6, r)]
        parity_error: bool,
        /// A framing error was detected.
        #[bit(5, r)]
        framing_error: bool,
        /// The RX FIFO overran.
        #[bit(4, r)]
        overrun_error: bool,
        /// The RX timeout condition occurred.
        #[bit(3, r)]
        timeout: bool,
        /// The receiver is currently busy.
        #[bit(2, r)]
        busy: bool,
        /// The RX FIFO is not full.
        #[bit(1, r)]
        not_full: bool,
        /// Data is available to read from the RX FIFO.
        #[bit(0, r)]
        data_available: bool,
    }

    /// TX_STATUS register.
    #[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
    pub struct TxStatus {
        /// Level of the CTSN line.
        #[bit(15, r)]
        tx_ctsn: bool,
        /// A write was lost because the TX FIFO was full.
        #[bit(3, r)]
        wr_lost: bool,
        /// The transmitter is currently busy.
        #[bit(2, r)]
        tx_busy: bool,
        /// A write to the TX FIFO is currently in progress.
        #[bit(1, r)]
        write_busy: bool,
        /// There is space in the FIFO to write data.
        #[bit(0, r)]
        ready: bool,
    }

    /// FIFO_CLR register.
    #[bitbybit::bitfield(u32, default = 0x0)]
    #[derive(Debug)]
    pub struct FifoClear {
        /// Clear the TX FIFO.
        #[bit(1, w)]
        tx: bool,
        /// Clear the RX FIFO.
        #[bit(0, w)]
        rx: bool,
    }

    /// Interrupt enable register.
    #[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
    pub struct InterruptControl {
        /// Generate an interrrupt when the RX FIFO is at least half-full (FIFO count >= trigger level)
        #[bit(0, rw)]
        rx: bool,
        /// Interrupts for status conditions (overrun, framing, parity and break)
        #[bit(1, rw)]
        rx_status: bool,
        /// Interrupt on timeout conditions.
        #[bit(2, rw)]
        rx_timeout: bool,

        /// Generates an interrupt when the TX FIFO is at least half-empty (FIFO count < trigger level)
        #[bit(4, rw)]
        tx_below_trigger: bool,
        /// Generates an interrupt on TX FIFO overflow.
        #[bit(5, rw)]
        tx_status: bool,
        /// Generates an interrupt when the transmit FIFO is empty and TXBUSY is 0.
        #[bit(6, rw)]
        tx_empty: bool,
        /// Generates an interrupt on a CTS line change.
        #[bit(7, rw)]
        tx_cts: bool,
    }

    /// Interrupt status register.
    #[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
    pub struct InterruptStatus {
        /// Generate an interrrupt when the RX FIFO is at least half-full (FIFO count >= trigger level)
        #[bit(0, r)]
        rx: bool,
        /// Interrupts for status conditions (overrun, framing, parity and break)
        #[bit(1, r)]
        rx_status: bool,
        /// Interrupt on timeout conditions.
        #[bit(2, r)]
        rx_timeout: bool,

        /// Generates an interrupt when the TX FIFO is at least half-empty (FIFO count < trigger level)
        #[bit(4, r)]
        tx_below_trigger: bool,
        /// Generates an interrupt on TX FIFO overflow.
        #[bit(5, r)]
        tx_status: bool,
        /// Generates an interrupt when the transmit FIFO is empty and TXBUSY is 0.
        #[bit(6, r)]
        tx_empty: bool,
        /// Generates an interrupt on a CTS line change.
        #[bit(7, r)]
        tx_cts: bool,
    }

    /// Interrupt clear register. As specified in the VA416x0 Programmers Guide, only the RX overflow
    /// bit can be cleared.
    #[bitbybit::bitfield(u32, default = 0x0)]
    #[derive(Debug)]
    pub struct InterruptClear {
        /// Clear the RX FIFO overrun flag.
        #[bit(1, w)]
        rx_overrun: bool,
        /// Not sure if this does anything, the programmer guides are not consistent on this..
        #[bit(5, w)]
        tx_overrun: bool,
    }

    /// FIFO trigger level register.
    #[bitbybit::bitfield(u32)]
    #[derive(Debug)]
    pub struct FifoTrigger {
        /// The configured trigger level.
        #[bits(0..=4, rw)]
        level: u5,
    }

    /// STATE register, exposing the raw FIFO fill levels.
    #[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
    pub struct State {
        /// Raw receiver state machine state.
        #[bits(0..=7, r)]
        rx_state: u8,
        /// Data count.
        #[bits(8..=12, r)]
        rx_fifo: u5,
        /// Raw transmitter state machine state.
        #[bits(16..=23, r)]
        tx_state: u8,
        /// Data count.
        #[bits(24..=28, r)]
        tx_fifo: u5,
    }
}

/// UART peripheral register block.
#[derive(derive_mmio::Mmio)]
#[mmio(no_ctors)]
#[repr(C)]
pub struct Registers {
    data: Data,
    enable: Enable,
    control: Control,
    clkscale: ClockScale,
    #[mmio(PureRead)]
    rx_status: RxStatus,
    #[mmio(PureRead)]
    tx_status: TxStatus,
    #[mmio(Write)]
    fifo_clr: FifoClear,
    #[mmio(Write)]
    txbreak: u32,
    addr9: u32,
    addr9mask: u32,
    interrupt_enable: InterruptControl,
    #[mmio(PureRead)]
    interrupt_raw: InterruptStatus,
    #[mmio(PureRead)]
    interrupt_status: InterruptStatus,
    #[mmio(Write)]
    irq_clr: InterruptClear,
    rx_fifo_trigger: FifoTrigger,
    tx_fifo_trigger: FifoTrigger,
    rx_fifo_rts_trigger: u32,
    #[mmio(PureRead)]
    state: State,
    _reserved: [u32; 0x3ED],
    /// Vorago 1x value: 0x0112_07E1. Vorago 4x value: 0x0212_07E9
    #[mmio(PureRead)]
    perid: u32,
}

static_assertions::const_assert_eq!(core::mem::size_of::<Registers>(), 0x1000);

impl Registers {
    fn new_mmio_at(base: usize) -> MmioRegisters<'static> {
        MmioRegisters {
            ptr: base as *mut _,
            phantom: PhantomData,
        }
    }

    /// Get an MMIO accessor for the register block of the given bank.
    pub fn new_mmio(bank: Bank) -> MmioRegisters<'static> {
        match bank {
            Bank::Uart0 => Self::new_mmio_at(BASE_ADDR_0),
            Bank::Uart1 => Self::new_mmio_at(BASE_ADDR_1),
            #[cfg(feature = "vor4x")]
            Bank::Uart2 => Self::new_mmio_at(BASE_ADDR_2),
        }
    }
}
