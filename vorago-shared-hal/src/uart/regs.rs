use core::marker::PhantomData;

use arbitrary_int::{u5, u6, u18};

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bank {
    Uart0 = 0,
    Uart1 = 1,
    #[cfg(feature = "vor4x")]
    Uart2 = 2,
}

impl Bank {
    /// Unsafely steal the GPIO peripheral block for the given port.
    ///
    /// # Safety
    ///
    /// Circumvents ownership and safety guarantees by the HAL.
    pub unsafe fn steal_regs(&self) -> MmioUart<'static> {
        Uart::new_mmio(*self)
    }

    #[cfg(feature = "vor4x")]
    pub const fn interrupt_id_tx(&self) -> va416xx::Interrupt {
        match self {
            Bank::Uart0 => va416xx::Interrupt::UART0_TX,
            Bank::Uart1 => va416xx::Interrupt::UART1_TX,
            Bank::Uart2 => va416xx::Interrupt::UART2_TX,
        }
    }

    #[cfg(feature = "vor4x")]
    pub const fn interrupt_id_rx(&self) -> va416xx::Interrupt {
        match self {
            Bank::Uart0 => va416xx::Interrupt::UART0_RX,
            Bank::Uart1 => va416xx::Interrupt::UART1_RX,
            Bank::Uart2 => va416xx::Interrupt::UART2_RX,
        }
    }
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct Data {
    #[bit(15, rw)]
    dparity: bool,
    #[bits(0..=7, rw)]
    value: u8,
}

#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
pub struct Enable {
    #[bit(1, rw)]
    tx: bool,
    #[bit(0, rw)]
    rx: bool,
}

#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stopbits {
    One = 0,
    Two = 1,
}

#[bitbybit::bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WordSize {
    Five = 0b00,
    Six = 0b01,
    Seven = 0b10,
    Eight = 0b11,
}

#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_fields(feature = "defmt"))]
pub struct Control {
    #[bit(11, rw)]
    baud8: bool,
    #[bit(10, rw)]
    auto_rts: bool,
    #[bit(9, rw)]
    def_rts: bool,
    #[bit(8, rw)]
    auto_cts: bool,
    #[bit(7, rw)]
    loopback_block: bool,
    #[bit(6, rw)]
    loopback: bool,
    #[bits(4..=5, rw)]
    wordsize: WordSize,
    #[bit(3, rw)]
    stopbits: Stopbits,
    #[bit(2, rw)]
    parity_manual: bool,
    #[bit(1, rw)]
    parity_even: bool,
    #[bit(0, rw)]
    parity_enable: bool,
}

#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
pub struct ClockScale {
    #[bits(6..=23, rw)]
    int: u18,
    #[bits(0..=5, rw)]
    frac: u6,
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct RxStatus {
    #[bit(15, r)]
    rx_rtsn: bool,
    #[bit(9, r)]
    rx_addr9: bool,
    #[bit(8, r)]
    busy_break: bool,
    #[bit(7, r)]
    break_error: bool,
    #[bit(6, r)]
    parity_error: bool,
    #[bit(5, r)]
    framing_error: bool,
    #[bit(4, r)]
    overrun_error: bool,
    #[bit(3, r)]
    timeout: bool,
    #[bit(2, r)]
    busy: bool,
    #[bit(1, r)]
    not_full: bool,
    #[bit(0, r)]
    data_available: bool,
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct TxStatus {
    #[bit(15, r)]
    tx_ctsn: bool,
    #[bit(3, r)]
    wr_lost: bool,
    #[bit(2, r)]
    tx_busy: bool,
    #[bit(1, r)]
    write_busy: bool,
    /// There is space in the FIFO to write data.
    #[bit(0, r)]
    ready: bool,
}

#[bitbybit::bitfield(u32, default = 0x0)]
#[derive(Debug)]
pub struct FifoClear {
    #[bit(1, w)]
    tx: bool,
    #[bit(0, w)]
    rx: bool,
}

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
    tx: bool,
    /// Generates an interrupt on TX FIFO overflow.
    #[bit(5, rw)]
    tx_status: bool,
    /// Generates an interrupt when the transmit FIFO is empty and TXBUSY is 0.
    #[bit(6, rw)]
    tx_empty: bool,
    #[bit(7, rw)]
    tx_cts: bool,
}

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
    tx: bool,
    /// Generates an interrupt on TX FIFO overflow.
    #[bit(5, r)]
    tx_status: bool,
    /// Generates an interrupt when the transmit FIFO is empty and TXBUSY is 0.
    #[bit(6, r)]
    tx_empty: bool,
    #[bit(7, r)]
    tx_cts: bool,
}

/// As specified in the VA416x0 Programmers Guide, only the RX overflow bit can be cleared.
#[bitbybit::bitfield(u32, default = 0x0)]
#[derive(Debug)]
pub struct InterruptClear {
    #[bit(1, w)]
    rx_overrun: bool,
    /// Not sure if this does anything, the programmer guides are not consistent on this..
    #[bit(5, w)]
    tx_overrun: bool,
}

#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct FifoTrigger {
    #[bits(0..=4, rw)]
    level: u5,
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct State {
    #[bits(0..=7, r)]
    rx_state: u8,
    /// Data count.
    #[bits(8..=12, r)]
    rx_fifo: u5,
    #[bits(16..=23, r)]
    tx_state: u8,
    /// Data count.
    #[bits(24..=28, r)]
    tx_fifo: u5,
}

#[derive(derive_mmio::Mmio)]
#[mmio(no_ctors)]
#[repr(C)]
pub struct Uart {
    data: Data,
    enable: Enable,
    ctrl: Control,
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
    irq_enabled: InterruptControl,
    #[mmio(PureRead)]
    irq_raw: InterruptStatus,
    #[mmio(PureRead)]
    irq_status: InterruptStatus,
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

static_assertions::const_assert_eq!(core::mem::size_of::<Uart>(), 0x1000);

impl Uart {
    fn new_mmio_at(base: usize) -> MmioUart<'static> {
        MmioUart {
            ptr: base as *mut _,
            phantom: PhantomData,
        }
    }

    pub fn new_mmio(bank: Bank) -> MmioUart<'static> {
        match bank {
            Bank::Uart0 => Self::new_mmio_at(BASE_ADDR_0),
            Bank::Uart1 => Self::new_mmio_at(BASE_ADDR_1),
            #[cfg(feature = "vor4x")]
            Bank::Uart2 => Self::new_mmio_at(BASE_ADDR_2),
        }
    }
}
