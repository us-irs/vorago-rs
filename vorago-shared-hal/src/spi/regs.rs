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

/// SPI peripheral bank.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bank {
    /// SPI0.
    Spi0 = 0,
    /// SPI1.
    Spi1 = 1,
    /// SPI2.
    Spi2 = 2,
    /// SPI3.
    #[cfg(feature = "vor4x")]
    Spi3 = 3,
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

/// SPI word size, encoded as the bit position of the most significant bit.
#[bitbybit::bitenum(u4)]
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WordSize {
    /// 1 bit words.
    OneBit = 0x00,
    /// 4 bit words.
    FourBits = 0x03,
    /// 8 bit words.
    EightBits = 0x07,
    /// 16 bit words.
    SixteenBits = 0x0f,
}

/// ID of a hardware chip select line.
#[derive(Debug, PartialEq, Eq)]
#[bitbybit::bitenum(u3, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HwChipSelectId {
    /// Chip select 0.
    Id0 = 0,
    /// Chip select 1.
    Id1 = 1,
    /// Chip select 2.
    Id2 = 2,
    /// Chip select 3.
    Id3 = 3,
    /// Chip select 4.
    Id4 = 4,
    /// Chip select 5.
    Id5 = 5,
    /// Chip select 6.
    Id6 = 6,
    /// Chip select 7.
    Id7 = 7,
}

/// CTRL0 register, controlling the clock and word size.
#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_fields(feature = "defmt"))]
pub struct Control0 {
    /// Serial clock rate divisor.
    #[bits(8..=15, rw)]
    scrdv: u8,
    /// Clock phase.
    #[bit(7, rw)]
    sph: bool,
    /// Clock polarity.
    #[bit(6, rw)]
    spo: bool,
    /// Word size.
    #[bits(0..=3, rw)]
    word_size: Option<WordSize>,
}

/// CTRL1 register, controlling the operating mode.
#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
pub struct Control1 {
    /// Pause the master transmitter.
    #[bit(11, rw)]
    mtxpause: bool,
    /// Master delayer capture mode.
    #[bit(10, rw)]
    mdlycap: bool,
    /// Blockmode stall: stall the clock while the FIFO is empty during blockmode.
    #[bit(9, rw)]
    bm_stall: bool,
    /// Blockmode start: the peripheral is in the middle of a blockmode frame.
    #[bit(8, rw)]
    bm_start: bool,
    /// Enable blockmode.
    #[bit(7, rw)]
    blockmode: bool,
    /// Hardware chip select to use.
    #[bits(4..=6, rw)]
    ss: HwChipSelectId,
    /// Slave output disable.
    #[bit(3, rw)]
    sod: bool,
    /// Enable slave mode.
    #[bit(2, rw)]
    slave_mode: bool,
    /// Enable the SPI peripheral.
    #[bit(1, rw)]
    enable: bool,
    /// Loopback mode.
    #[bit(0, rw)]
    lbm: bool,
}

/// DATA register, used to read from and write to the FIFOs.
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
    /// The data word.
    #[bits(0..=15, rw)]
    data: u16,
}

/// STATUS register.
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct Status {
    /// TX FIFO below the trigger level.
    #[bit(7, r)]
    tx_trigger: bool,
    /// RX FIFO above or equals the trigger level.
    #[bit(6, r)]
    rx_trigger: bool,
    /// The next word read from the RX FIFO is the first word of a blockmode frame.
    #[bit(5, r)]
    rx_data_first: bool,
    /// The SPI peripheral is currently busy transferring data.
    #[bit(4, r)]
    busy: bool,
    /// The RX FIFO is full.
    #[bit(3, r)]
    rx_full: bool,
    /// The RX FIFO is not empty.
    #[bit(2, r)]
    rx_not_empty: bool,
    /// The TX FIFO is not full.
    #[bit(1, r)]
    tx_not_full: bool,
    /// The TX FIFO is empty.
    #[bit(0, r)]
    tx_empty: bool,
}

/// Clock divisor value. Bit 0 is ignored and always 0. This means that only the even values
/// are used as clock divisor values, and uneven values are truncated to the next even value.
/// A value of 0 acts as a 1 for the divisor value.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ClockPrescaler(arbitrary_int::UInt<u32, 8>);

impl ClockPrescaler {
    /// Create a new clock prescaler value.
    pub const fn new(value: u8) -> Self {
        ClockPrescaler(arbitrary_int::UInt::<u32, 8>::new(value as u32))
    }

    /// The raw prescaler value.
    pub const fn value(&self) -> u8 {
        self.0.value() as u8
    }
}

/// Interrupt enable register.
#[bitbybit::bitfield(u32, debug, default = 0x0, defmt_bitfields(feature = "defmt"))]
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
    /// RX FIFO overrun.
    #[bit(0, rw)]
    rx_overrun: bool,
}

impl InterruptControl {
    /// Disable all interrupts.
    pub const DISABLE_ALL: Self = Self::ZERO;
    /// Enable all interrupts.
    pub const ENABLE_ALL: Self = Self::builder()
        .with_tx(true)
        .with_rx(true)
        .with_rx_timeout(true)
        .with_rx_overrun(true)
        .build();
}

/// Interrupt status register.
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct InterruptStatus {
    /// TX FIFO count < TX FIFO trigger level.
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
    /// RX FIFO overrun.
    #[bit(0, r)]
    rx_overrun: bool,
}

/// Interrupt clear register.
#[bitbybit::bitfield(u32, default = 0x0)]
#[derive(Debug)]
pub struct InterruptClear {
    /// Clearing the RX interrupt or reading data from the FIFO resets the timeout counter.
    #[bit(1, w)]
    rx_timeout: bool,
    /// Clear the RX FIFO overrun flag.
    #[bit(0, w)]
    rx_overrun: bool,
}

impl InterruptClear {
    /// Clear all interrupts.
    pub const ALL: Self = Self::builder()
        .with_rx_timeout(true)
        .with_rx_overrun(true)
        .build();
}

/// STATE register, exposing the raw FIFO fill levels.
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct State {
    /// Raw receiver state machine state.
    #[bits(0..=7, r)]
    rx_state: u8,
    /// RX FIFO fill level.
    #[bits(8..=15, r)]
    rx_fifo: u8,
    /// TX FIFO fill level.
    #[bits(24..=31, r)]
    tx_fifo: u8,
}

/// SPI peripheral register block.
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
    interrupt_control: InterruptControl,
    /// Raw interrupt status.
    #[mmio(PureRead)]
    interrupt_status_raw: InterruptStatus,
    /// Enabled interrupt status.
    #[mmio(PureRead)]
    interrupt_status: InterruptStatus,
    #[mmio(Write)]
    interrupt_clear: InterruptClear,
    rx_fifo_trigger: TriggerLevel,
    tx_fifo_trigger: TriggerLevel,
    #[mmio(Write)]
    fifo_clear: FifoClear,
    #[mmio(PureRead)]
    state: State,
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

    /// Get an MMIO accessor for the register block of the given bank.
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
