use core::marker::PhantomData;

use arbitrary_int::{u4, u5, u9, u10, u11, u20};

pub use crate::shared::{FifoClear, TriggerLevel};

cfg_if::cfg_if! {
    if #[cfg(feature = "vor1x")] {
        /// I2C A base address
        pub const BASE_ADDR_0: usize = 0x4006_0000;
        /// I2C B base address
        pub const BASE_ADDR_1: usize = 0x4006_1000;
    } else if #[cfg(feature = "vor4x")] {
        /// I2C 0 base address
        pub const BASE_ADDR_0: usize = 0x4001_6000;
        /// I2C 1 base address
        pub const BASE_ADDR_1: usize = 0x4001_6400;
        /// I2C 2 base address
        pub const BASE_ADDR_2: usize = 0x4001_6800;
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bank {
    I2c0 = 0,
    I2c1 = 1,
    #[cfg(feature = "vor4x")]
    I2c2 = 2,
}

impl Bank {
    /// Unsafely steal the I2C peripheral block for the given port.
    ///
    /// # Safety
    ///
    /// Circumvents ownership and safety guarantees by the HAL.
    pub unsafe fn steal_regs(&self) -> MmioI2c<'static> {
        I2c::new_mmio(*self)
    }
}

#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxFifoEmptyMode {
    /// I2C clock is stretched until data is available.
    #[default]
    Stall = 0,
    EndTransaction = 1,
}

#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxFifoFullMode {
    /// I2C clock is stretched until data is available.
    #[default]
    Stall = 0,
    Nack = 1,
}

#[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
pub struct Control {
    #[bit(0, r)]
    clk_enabled: bool,
    #[bit(1, r)]
    enabled: bool,
    #[bit(2, rw)]
    enable: bool,
    #[bit(3, rw)]
    tx_fifo_empty_mode: TxFifoEmptyMode,
    #[bit(4, rw)]
    rx_fifo_full_mode: RxFifoFullMode,
    /// Enables the analog delay glitch filter.
    #[bit(5, rw)]
    analog_filter: bool,
    /// Enables the digital glitch filter.
    #[bit(6, rw)]
    digital_filter: bool,
    #[bit(8, rw)]
    loopback: bool,
    #[bit(9, rw)]
    enable_timing_config: bool,
}

#[derive(Debug, PartialEq, Eq)]
#[bitbybit::bitenum(u1, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cSpeed {
    Regular100khz = 0,
    Fast400khz = 1,
}

#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_fields(feature = "defmt"))]
pub struct ClockScale {
    /// Clock divide value. Reset value: 0x18.
    #[bits(0..=7, rw)]
    div: u8,
    #[bit(31, rw)]
    fastmode: I2cSpeed,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Words(arbitrary_int::UInt<u32, 11>);

impl Words {
    pub const fn new(value: u11) -> Self {
        Words(arbitrary_int::UInt::<u32, 11>::new(value.value() as u32))
    }
    pub const fn value(&self) -> u11 {
        u11::new(self.0.value() as u16)
    }
}

#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Direction {
    #[default]
    Send = 0,
    Receive = 1,
}

#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
pub struct Address {
    #[bit(0, rw)]
    direction: Direction,
    #[bits(1..=10, rw)]
    address: u10,
    /// Enables 10-bit addressing mode.
    #[bit(15, rw)]
    a10_mode: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Data(arbitrary_int::UInt<u32, 8>);

impl Data {
    pub const fn new(value: u8) -> Self {
        Data(arbitrary_int::UInt::<u32, 8>::new(value as u32))
    }

    pub const fn data(&self) -> u8 {
        self.0.value() as u8
    }
}

#[bitbybit::bitfield(u32, default = 0x0)]
#[derive(Debug)]
pub struct Command {
    #[bit(0, w)]
    start: bool,
    #[bit(1, w)]
    stop: bool,
    #[bit(2, w)]
    cancel: bool,
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct Status {
    #[bit(0, r)]
    i2c_idle: bool,
    #[bit(1, r)]
    idle: bool,
    #[bit(2, r)]
    waiting: bool,
    #[bit(3, r)]
    stalled: bool,
    #[bit(4, r)]
    arb_lost: bool,
    #[bit(5, r)]
    nack_addr: bool,
    #[bit(6, r)]
    nack_data: bool,
    #[bit(8, r)]
    rx_not_empty: bool,
    #[bit(9, r)]
    rx_full: bool,
    #[bit(11, r)]
    rx_trigger: bool,
    #[bit(12, r)]
    tx_empty: bool,
    #[bit(13, r)]
    tx_not_full: bool,
    #[bit(15, r)]
    tx_trigger: bool,
    #[bit(30, r)]
    raw_sda: bool,
    #[bit(31, r)]
    raw_scl: bool,
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct State {
    #[bits(0..=3, rw)]
    state: u4,
    #[bits(4..=7, rw)]
    step: u4,
    #[bits(8..=12, rw)]
    rx_fifo: u5,
    #[bits(14..=18, rw)]
    tx_fifo: u5,
    #[bits(20..=28, rw)]
    bitstate: u9,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DataCount(arbitrary_int::UInt<u32, 11>);

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct InterruptControl {
    #[bit(0, rw)]
    i2c_idle: bool,
    #[bit(1, rw)]
    idle: bool,
    #[bit(2, rw)]
    waiting: bool,
    #[bit(3, rw)]
    stalled: bool,
    #[bit(4, rw)]
    arb_lost: bool,
    #[bit(5, rw)]
    nack_addr: bool,
    #[bit(6, rw)]
    nack_data: bool,
    #[bit(7, rw)]
    clock_timeout: bool,
    #[bit(10, rw)]
    tx_overflow: bool,
    #[bit(11, rw)]
    rx_overflow: bool,
    #[bit(12, rw)]
    tx_ready: bool,
    #[bit(13, rw)]
    rx_ready: bool,
    #[bit(14, rw)]
    tx_empty: bool,
    #[bit(15, rw)]
    rx_full: bool,
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct InterruptStatus {
    #[bit(0, r)]
    i2c_idle: bool,
    #[bit(1, r)]
    idle: bool,
    #[bit(2, r)]
    waiting: bool,
    #[bit(3, r)]
    stalled: bool,
    #[bit(4, r)]
    arb_lost: bool,
    #[bit(5, r)]
    nack_addr: bool,
    #[bit(6, r)]
    nack_data: bool,
    #[bit(7, r)]
    clock_timeout: bool,
    #[bit(10, r)]
    tx_overflow: bool,
    #[bit(11, r)]
    rx_overflow: bool,
    #[bit(12, r)]
    tx_ready: bool,
    #[bit(13, r)]
    rx_ready: bool,
    #[bit(14, r)]
    tx_empty: bool,
    #[bit(15, r)]
    rx_full: bool,
}

#[bitbybit::bitfield(u32, default = 0x0)]
#[derive(Debug)]
pub struct InterruptClear {
    #[bit(7, w)]
    clock_timeout: bool,
    #[bit(10, w)]
    tx_overflow: bool,
    #[bit(11, w)]
    rx_overflow: bool,
}

#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct TimingConfig {
    /// Rise time.
    #[bits(0..=3, rw)]
    t_rise: u4,
    /// Fall time.
    #[bits(4..=7, rw)]
    t_fall: u4,
    /// Duty cycle high time of SCL.
    #[bits(8..=11, rw)]
    t_high: u4,
    /// Duty cycle low time of SCL.
    #[bits(12..=15, rw)]
    t_low: u4,
    /// Setup time for STOP.
    #[bits(16..=19, rw)]
    tsu_stop: u4,
    /// Setup time for START.
    #[bits(20..=23, rw)]
    tsu_start: u4,
    /// Data hold time.
    #[bits(24..=27, rw)]
    thd_start: u4,
    /// TBus free time between STOP and START.
    #[bits(28..=31, rw)]
    t_buf: u4,
}

pub struct ClockTimeoutLimit(pub arbitrary_int::UInt<u32, 20>);

impl ClockTimeoutLimit {
    pub fn new(value: u20) -> Self {
        ClockTimeoutLimit(arbitrary_int::UInt::<u32, 20>::new(value.value()))
    }
    pub fn value(&self) -> u20 {
        self.0
    }
}

pub mod slave {
    use super::{Data, DataCount, FifoClear, RxFifoFullMode, TriggerLevel, TxFifoEmptyMode};
    use arbitrary_int::{u3, u4, u5, u10, u11};

    #[bitbybit::bitfield(u32)]
    #[derive(Debug)]
    pub struct Control {
        #[bit(0, r)]
        clk_enabled: bool,
        #[bit(1, r)]
        enabled: bool,
        #[bit(2, rw)]
        enable: bool,
        #[bit(3, rw)]
        tx_fifo_empty_mode: TxFifoEmptyMode,
        #[bit(4, rw)]
        rx_fifo_full_mode: RxFifoFullMode,
    }

    #[bitbybit::bitfield(u32)]
    #[derive(Debug)]
    pub struct Maxwords {
        #[bits(0..=10, rw)]
        maxwords: u11,
        #[bit(31, rw)]
        enable: bool,
    }

    #[bitbybit::bitfield(u32)]
    #[derive(Debug)]
    pub struct Address {
        #[bit(0, rw)]
        rw: bool,
        #[bits(1..=10, rw)]
        address: u10,
        #[bit(15, rw)]
        a10_mode: bool,
    }

    #[bitbybit::bitfield(u32)]
    #[derive(Debug)]
    pub struct AddressMask {
        /// Will normally be 0 to match both read and write addresses.
        #[bit(0, rw)]
        rw_mask: bool,
        /// Reset value 0x3FF.
        #[bits(1..=10, rw)]
        mask: u10,
    }

    #[bitbybit::bitenum(u1, exhaustive = true)]
    #[derive(Default, Debug, PartialEq, Eq)]
    pub enum Direction {
        #[default]
        MasterSend = 0,
        MasterReceive = 1,
    }

    #[bitbybit::bitfield(u32)]
    #[derive(Debug)]
    pub struct LastAddress {
        #[bit(0, rw)]
        direction: Direction,
        #[bits(1..=10, rw)]
        address: u10,
    }

    #[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
    pub struct Status {
        #[bit(0, r)]
        completed: bool,
        #[bit(1, r)]
        idle: bool,
        #[bit(2, r)]
        waiting: bool,
        #[bit(3, r)]
        tx_stalled: bool,
        #[bit(4, r)]
        rx_stalled: bool,
        #[bit(5, r)]
        address_match: bool,
        #[bit(6, r)]
        nack_data: bool,
        #[bit(7, r)]
        rx_data_first: bool,
        #[bit(8, r)]
        rx_not_empty: bool,
        #[bit(9, r)]
        rx_full: bool,
        #[bit(11, r)]
        rx_trigger: bool,
        #[bit(12, r)]
        tx_empty: bool,
        #[bit(13, r)]
        tx_not_full: bool,
        #[bit(15, r)]
        tx_trigger: bool,
        #[bit(28, r)]
        raw_busy: bool,
        #[bit(30, r)]
        raw_sda: bool,
        #[bit(31, r)]
        raw_scl: bool,
    }

    #[bitbybit::bitfield(u32)]
    #[derive(Debug)]
    pub struct State {
        #[bits(0..=2, rw)]
        state: u3,
        #[bits(4..=7, rw)]
        step: u4,
        #[bits(8..=12, rw)]
        rx_fifo: u5,
        #[bits(14..=18, rw)]
        tx_fifo: u5,
    }

    #[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
    pub struct InterruptControl {
        #[bit(0, rw)]
        completed: bool,
        #[bit(1, rw)]
        idle: bool,
        #[bit(2, rw)]
        waiting: bool,
        #[bit(3, rw)]
        tx_stalled: bool,
        #[bit(4, rw)]
        rx_stalled: bool,
        #[bit(5, rw)]
        address_match: bool,
        #[bit(6, rw)]
        nack_data: bool,
        #[bit(7, rw)]
        rx_data_first: bool,

        #[bit(8, rw)]
        i2c_start: bool,
        #[bit(9, rw)]
        i2c_stop: bool,
        #[bit(10, rw)]
        tx_underflow: bool,
        #[bit(11, rw)]
        rx_underflow: bool,
        #[bit(12, rw)]
        tx_ready: bool,
        #[bit(13, rw)]
        rx_ready: bool,
        #[bit(14, rw)]
        tx_empty: bool,
        #[bit(15, rw)]
        rx_full: bool,
    }

    #[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
    pub struct InterruptStatus {
        #[bit(0, r)]
        completed: bool,
        #[bit(1, r)]
        idle: bool,
        #[bit(2, r)]
        waiting: bool,
        #[bit(3, r)]
        tx_stalled: bool,
        #[bit(4, r)]
        rx_stalled: bool,
        #[bit(5, r)]
        address_match: bool,
        #[bit(6, r)]
        nack_data: bool,
        #[bit(7, r)]
        rx_data_first: bool,

        #[bit(8, r)]
        i2c_start: bool,
        #[bit(9, r)]
        i2c_stop: bool,
        #[bit(10, r)]
        tx_underflow: bool,
        #[bit(11, r)]
        rx_underflow: bool,
        #[bit(12, r)]
        tx_ready: bool,
        #[bit(13, r)]
        rx_ready: bool,
        #[bit(14, r)]
        tx_empty: bool,
        #[bit(15, r)]
        rx_full: bool,
    }

    #[bitbybit::bitfield(u32, default = 0x0)]
    #[derive(Debug)]
    pub struct InterruptClear {
        #[bit(0, w)]
        completed: bool,
        #[bit(1, w)]
        idle: bool,
        #[bit(2, w)]
        waiting: bool,
        #[bit(3, w)]
        tx_stalled: bool,
        #[bit(4, w)]
        rx_stalled: bool,
        #[bit(5, w)]
        address_match: bool,
        #[bit(6, w)]
        nack_data: bool,
        #[bit(7, w)]
        rx_data_first: bool,

        #[bit(8, w)]
        i2c_start: bool,
        #[bit(9, w)]
        i2c_stop: bool,
        #[bit(10, w)]
        tx_underflow: bool,
        #[bit(11, w)]
        rx_underflow: bool,
        #[bit(12, w)]
        tx_ready: bool,
        #[bit(13, w)]
        rx_ready: bool,
        #[bit(14, w)]
        tx_empty: bool,
        #[bit(15, w)]
        rx_full: bool,
    }

    #[derive(derive_mmio::Mmio)]
    #[repr(C)]
    pub struct I2cSlave {
        s0_ctrl: Control,
        s0_maxwords: Maxwords,
        s0_address: Address,
        s0_addressmask: AddressMask,
        s0_data: Data,
        s0_lastaddress: LastAddress,
        #[mmio(PureRead)]
        s0_status: Status,
        #[mmio(PureRead)]
        s0_state: State,
        #[mmio(PureRead)]
        s0_tx_count: DataCount,
        #[mmio(PureRead)]
        s0_rx_count: DataCount,
        s0_irq_enb: InterruptControl,
        #[mmio(PureRead)]
        s0_irq_raw: InterruptStatus,
        #[mmio(PureRead)]
        s0_irq_status: InterruptStatus,
        #[mmio(Write)]
        s0_irq_clear: InterruptClear,
        s0_rx_fifo_trigger: TriggerLevel,
        s0_tx_fifo_trigger: TriggerLevel,
        #[mmio(Write)]
        s0_fifo_clear: FifoClear,
        s0_address_b: Address,
        s0_addressmask_b: AddressMask,
    }
}
#[derive(derive_mmio::Mmio)]
#[mmio(no_ctors)]
#[repr(C)]
pub struct I2c {
    control: Control,
    clkscale: ClockScale,
    words: Words,
    address: Address,
    data: Data,
    #[mmio(Write)]
    cmd: Command,
    #[mmio(PureRead)]
    status: Status,
    #[mmio(PureRead)]
    state: State,
    #[mmio(PureRead)]
    tx_count: DataCount,
    #[mmio(PureRead)]
    rx_count: DataCount,
    irq_enb: InterruptControl,
    #[mmio(PureRead)]
    irq_raw: InterruptStatus,
    #[mmio(PureRead)]
    irq_status: InterruptStatus,
    #[mmio(Write)]
    irq_clear: InterruptClear,
    rx_fifo_trigger: TriggerLevel,
    tx_fifo_trigger: TriggerLevel,
    #[mmio(Write)]
    fifo_clear: FifoClear,
    timing_config: TimingConfig,
    clk_timeout_limit: ClockTimeoutLimit,

    _reserved_0: [u32; 0x2D],

    #[mmio(Inner)]
    slave: slave::I2cSlave,

    #[cfg(feature = "vor1x")]
    _reserved_1: [u32; 0x3AC],
    #[cfg(feature = "vor4x")]
    _reserved_1: [u32; 0xAC],

    /// Vorago 4x: 0x0214_07E9. Vorago 1x: 0x0014_07E1.
    #[mmio(PureRead)]
    perid: u32,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "vor1x")] {
        static_assertions::const_assert_eq!(core::mem::size_of::<I2c>(), 0x1000);
    } else if #[cfg(feature = "vor4x")] {
        static_assertions::const_assert_eq!(core::mem::size_of::<I2c>(), 0x400);
    }
}

impl I2c {
    fn new_mmio_at(base: usize) -> MmioI2c<'static> {
        MmioI2c {
            ptr: base as *mut _,
            phantom: PhantomData,
        }
    }

    pub fn new_mmio(bank: Bank) -> MmioI2c<'static> {
        match bank {
            Bank::I2c0 => Self::new_mmio_at(BASE_ADDR_0),
            Bank::I2c1 => Self::new_mmio_at(BASE_ADDR_1),
            #[cfg(feature = "vor4x")]
            Bank::I2c2 => Self::new_mmio_at(BASE_ADDR_2),
        }
    }
}
