use core::marker::PhantomData;

pub use crate::shared::{FifoClear, TriggerLevel};

cfg_if::cfg_if! {
    if #[cfg(feature = "vor1x")] {
        /// SPI A base address
        pub const BASE_ADDR_0: usize = 0x4005_0000;
        /// SPI B base address
        pub const BASE_ADDR_1: usize = 0x4005_1000;
        /// SPI C base address
        pub const BASE_ADDR_2: usize = 0x4005_2000;
    } else if #[cfg(feature = "vor4x")] {
        /// SPI 0 base address
        pub const BASE_ADDR_0: usize = 0x4001_5000;
        /// SPI 1 base address
        pub const BASE_ADDR_1: usize = 0x4001_5400;
        /// SPI 2 base address
        pub const BASE_ADDR_2: usize = 0x4001_5800;
        /// SPI 3 base address
        pub const BASE_ADDR_3: usize = 0x4001_5C00;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bank {
    Spi0,
    Spi1,
    Spi2,
    #[cfg(feature = "vor4x")]
    Spi3,
}

impl Bank {
    /// Unsafely steal the SPI peripheral block for the given port.
    ///
    /// # Safety
    ///
    /// Circumvents ownership and safety guarantees by the HAL.
    pub unsafe fn steal_regs(&self) -> MmioSpi<'static> {
        Spi::new_mmio(*self)
    }
}

#[bitbybit::bitenum(u4)]
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WordSize {
    OneBit = 0x00,
    FourBits = 0x03,
    EightBits = 0x07,
    SixteenBits = 0x0f,
}

#[derive(Debug, PartialEq, Eq)]
#[bitbybit::bitenum(u3, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HwChipSelectId {
    Id0 = 0,
    Id1 = 1,
    Id2 = 2,
    Id3 = 3,
    Id4 = 4,
    Id5 = 5,
    Id6 = 6,
    Id7 = 7,
}

#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_fields(feature = "defmt"))]
pub struct Control0 {
    #[bits(8..=15, rw)]
    scrdv: u8,
    #[bit(7, rw)]
    sph: bool,
    #[bit(6, rw)]
    spo: bool,
    #[bits(0..=3, rw)]
    word_size: Option<WordSize>,
}

#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
pub struct Control1 {
    #[bit(11, rw)]
    mtxpause: bool,
    #[bit(10, rw)]
    mdlycap: bool,
    #[bit(9, rw)]
    bm_stall: bool,
    #[bit(8, rw)]
    bm_start: bool,
    #[bit(7, rw)]
    blockmode: bool,
    #[bits(4..=6, rw)]
    ss: HwChipSelectId,
    #[bit(3, rw)]
    sod: bool,
    #[bit(2, rw)]
    slave_mode: bool,
    #[bit(1, rw)]
    enable: bool,
    #[bit(0, rw)]
    lbm: bool,
}

#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct Data {
    /// Only used for BLOCKMODE. For received data, this bit indicated that the data was the first
    /// word after the chip select went active. For transmitted data, setting this bit to 1
    /// will end an SPI frame (deassert CS) after the specified data word.
    #[bit(31, rw)]
    bm_start_stop: bool,
    /// Only used for BLOCKMODE. Setting this bit to 1 along with the BMSTOP bit will end an SPI
    /// frame without any additional data to be transmitted. If BMSTOP is not set, this bit is
    /// ignored.
    #[bit(30, rw)]
    bm_skipdata: bool,
    #[bits(0..=15, rw)]
    data: u16,
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct Status {
    /// TX FIFO below the trigger level.
    #[bit(7, r)]
    tx_trigger: bool,
    /// RX FIFO above or equals the trigger level.
    #[bit(6, r)]
    rx_trigger: bool,
    #[bit(5, r)]
    rx_data_first: bool,
    #[bit(4, r)]
    busy: bool,
    #[bit(3, r)]
    rx_full: bool,
    #[bit(2, r)]
    rx_not_empty: bool,
    #[bit(1, r)]
    tx_not_full: bool,
    #[bit(0, r)]
    tx_empty: bool,
}

/// Clock divisor value. Bit 0 is ignored and always 0. This means that only the even values
/// are used as clock divisor values, and uneven values are truncated to the next even value.
/// A value of 0 acts as a 1 for the divisor value.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ClockPrescaler(arbitrary_int::UInt<u32, 8>);

impl ClockPrescaler {
    pub const fn new(value: u8) -> Self {
        ClockPrescaler(arbitrary_int::UInt::<u32, 8>::new(value as u32))
    }
    pub const fn value(&self) -> u8 {
        self.0.value() as u8
    }
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct InterruptControl {
    /// TX FIFO count <= TX FIFO trigger level.
    #[bit(3, rw)]
    tx: bool,
    /// RX FIFO count >= RX FIFO trigger level.
    #[bit(2, rw)]
    rx: bool,
    /// Occurs when the RX FIFO has not been read within 32 clock ticks of the SPICLKx2 clock
    /// within the RX FIFO not being empty. Clearing the RX interrupt or reading data from the
    /// FIFO resets the timeout counter.
    #[bit(1, rw)]
    rx_timeout: bool,
    #[bit(0, rw)]
    rx_overrun: bool,
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct InterruptStatus {
    /// TX FIFO count <= TX FIFO trigger level.
    #[bit(3, r)]
    tx: bool,
    /// RX FIFO count >= RX FIFO trigger level.
    #[bit(2, r)]
    rx: bool,
    /// Occurs when the RX FIFO has not been read within 32 clock ticks of the SPICLKx2 clock
    /// within the RX FIFO not being empty. Clearing the RX interrupt or reading data from the
    /// FIFO resets the timeout counter.
    #[bit(1, r)]
    rx_timeout: bool,
    #[bit(0, r)]
    rx_overrun: bool,
}

#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct InterruptClear {
    /// Clearing the RX interrupt or reading data from the FIFO resets the timeout counter.
    #[bit(1, w)]
    rx_timeout: bool,
    #[bit(0, w)]
    rx_overrun: bool,
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct State {
    #[bits(0..=7, r)]
    rx_state: u8,
    #[bits(8..=15, r)]
    rx_fifo: u8,
    #[bits(24..=31, r)]
    tx_fifo: u8,
}

#[derive(derive_mmio::Mmio)]
#[mmio(no_ctors)]
#[repr(C)]
pub struct Spi {
    ctrl0: Control0,
    ctrl1: Control1,
    data: Data,
    #[mmio(PureRead)]
    status: Status,
    clkprescale: ClockPrescaler,
    irq_enb: InterruptControl,
    /// Raw interrupt status.
    #[mmio(PureRead)]
    irq_raw: InterruptStatus,
    /// Enabled interrupt status.
    #[mmio(PureRead)]
    irq_status: InterruptStatus,
    #[mmio(Write)]
    irq_clear: InterruptClear,
    rx_fifo_trigger: TriggerLevel,
    tx_fifo_trigger: TriggerLevel,
    #[mmio(Write)]
    fifo_clear: FifoClear,
    #[mmio(PureRead)]
    state: u32,
    #[cfg(feature = "vor1x")]
    _reserved: [u32; 0x3F2],
    #[cfg(feature = "vor4x")]
    _reserved: [u32; 0xF2],
    /// Vorago 1x: 0x0113_07E1. Vorago 4x: 0x0213_07E9.
    #[mmio(PureRead)]
    perid: u32,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "vor1x")] {
        static_assertions::const_assert_eq!(core::mem::size_of::<Spi>(), 0x1000);
    } else if #[cfg(feature = "vor4x")] {
        static_assertions::const_assert_eq!(core::mem::size_of::<Spi>(), 0x400);
    }
}

impl Spi {
    fn new_mmio_at(base: usize) -> MmioSpi<'static> {
        MmioSpi {
            ptr: base as *mut _,
            phantom: PhantomData,
        }
    }

    pub fn new_mmio(bank: Bank) -> MmioSpi<'static> {
        match bank {
            Bank::Spi0 => Self::new_mmio_at(BASE_ADDR_0),
            Bank::Spi1 => Self::new_mmio_at(BASE_ADDR_1),
            Bank::Spi2 => Self::new_mmio_at(BASE_ADDR_2),
            #[cfg(feature = "vor4x")]
            Bank::Spi3 => Self::new_mmio_at(BASE_ADDR_2),
        }
    }
}
