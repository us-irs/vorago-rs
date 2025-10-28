//! Custom register definitions for the CAN register block to circumvent PAC API / SVD
//! shortcomings.

use arbitrary_int::{prelude::*, u11, u15, u2, u3, u4, u6, u7};

pub const CAN_0_BASE: usize = 0x4001_4000;
pub const CAN_1_BASE: usize = 0x4001_4400;

#[derive(Debug, PartialEq, Eq)]
#[bitbybit::bitenum(u4)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BufferState {
    /// Passive channel.
    RxNotActive = 0b0000,
    /// This condition indicated that SW wrote RxNotActive to a buffer when a data copy
    /// process is still active.
    RxBusy = 0b0001,
    RxReady = 0b0010,
    /// Indicated that data is being copied for the first time (RxRead -> RxBusy0).
    RxBusy0 = 0b0011,
    RxFull = 0b0100,
    /// Indicated that data is being copied for the second time (RxFull -> RxBusy2).
    RxBusy1 = 0b0101,
    RxOverrun = 0b0110,
    RxBusy2 = 0b0111,
    TxNotActive = 0b1000,
    /// Automatical response to a remote frame.
    TxRtr = 0b1010,
    /// Transmit one frame.
    TxOnce = 0b1100,
    TxBusy0 = 0b1101,
    /// Transmit one frame, and changes to TxRtr after that. This can either be written by
    /// software, or it will be written by the hardware after an auto response of the
    /// [BufferState::TxRtr] state.
    TxOnceRtr = 0b1110,
    TxBusy2 = 0b1111,
}

/// Status control register for individual message buffers.
#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_fields(feature = "defmt"))]
pub struct BufStatusAndControl {
    /// Data length code.
    #[bits(12..=15, rw)]
    dlc: u4,
    #[bits(4..=7, rw)]
    priority: u4,
    #[bits(0..=3, rw)]
    state: Option<BufferState>,
}

#[derive(Debug)]
pub struct Timestamp(arbitrary_int::UInt<u32, 16>);

impl Timestamp {
    pub fn new(value: u16) -> Self {
        Self(value.into())
    }
    pub fn value(&self) -> u16 {
        self.0.value() as u16
    }

    pub fn write(&mut self, value: u16) {
        self.0 = value.into();
    }
}

#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
pub struct TwoBytesData {
    #[bits(0..=7, rw)]
    data_lower_byte: u8,
    #[bits(8..=15, rw)]
    data_upper_byte: u8,
}

#[derive(derive_mmio::Mmio)]
#[repr(C)]
pub struct CanMessageBuffer {
    stat_ctrl: BufStatusAndControl,
    timestamp: Timestamp,
    data3: TwoBytesData,
    data2: TwoBytesData,
    data1: TwoBytesData,
    data0: TwoBytesData,
    id0: ExtendedId,
    id1: BaseId,
}

static_assertions::const_assert_eq!(core::mem::size_of::<CanMessageBuffer>(), 0x20);

impl MmioCanMessageBuffer<'_> {
    pub fn reset(&mut self) {
        self.write_stat_ctrl(BufStatusAndControl::new_with_raw_value(0));
        self.write_timestamp(Timestamp::new(0));
        self.write_data0(TwoBytesData::new_with_raw_value(0));
        self.write_data1(TwoBytesData::new_with_raw_value(0));
        self.write_data2(TwoBytesData::new_with_raw_value(0));
        self.write_data3(TwoBytesData::new_with_raw_value(0));
        self.write_id1(BaseId::new_with_raw_value(0));
        self.write_id0(ExtendedId::new_with_raw_value(0));
    }
}

#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PinLogicLevel {
    DominantIsZero = 0b0,
    DominantIsOne = 0b1,
}

#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrorInterruptType {
    /// EIPND bit is set on every error.
    EveryError = 0b0,
    /// EIPND bit is only set if error state changes as a result of a receive or transmit
    /// error counter increment.
    ErrorOnRxTxCounterChange = 0b1,
}

#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataDirection {
    FirstByteAtHighestAddr = 0b0,
    LastByteAtHighestAddr = 0b1,
}

#[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
pub struct Control {
    #[bit(11, rw)]
    error_interrupt_type: ErrorInterruptType,
    /// Enables special diagnostics features of the CAN like LO, IGNACK, LOOPBACK, INTERNAL.
    #[bit(10, rw)]
    diag_enable: bool,
    /// CANTX and CANRX pins are internally connected to each other.
    #[bit(9, rw)]
    internal: bool,
    /// All messages sent by the CAN controller can also be received by a CAN buffer with a
    /// matching buffer ID.
    #[bit(8, rw)]
    loopback: bool,
    /// IGNACK feature. The CAN does not expect to receive an ACK bit.
    #[bit(7, rw)]
    ignore_ack: bool,
    /// LO feature. The CAN is only configured as a receiver.
    #[bit(6, rw)]
    listen_only: bool,
    #[bit(5, rw)]
    data_dir: DataDirection,
    #[bit(4, rw)]
    timestamp_enable: bool,
    #[bit(3, rw)]
    bufflock: bool,
    #[bit(2, rw)]
    tx_logic_level: PinLogicLevel,
    #[bit(1, rw)]
    rx_logic_level: PinLogicLevel,
    #[bit(0, rw)]
    enable: bool,
}

#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
pub struct TimingConfig {
    #[bits(0..=2, rw)]
    tseg2: u3,
    #[bits(3..=6, rw)]
    tseg1: u4,
    #[bits(7..=8, rw)]
    sync_jump_width: u2,
    #[bits(9..=15, rw)]
    prescaler: u7,
}

#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct InterruptEnable {
    #[bit(15, rw)]
    error: bool,
    #[bit(0, rw)]
    buffer: [bool; 15],
}

#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct InterruptClear {
    #[bit(15, w)]
    error: bool,
    #[bit(0, w)]
    buffer: [bool; 15],
}

#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct InterruptPending {
    #[bit(15, r)]
    error: bool,
    #[bit(0, r)]
    buffer: [bool; 15],
}

#[derive(Debug)]
#[repr(usize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanInterruptId {
    None = 0b00000,
    Error = 0b10000,
    Buffer(usize),
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct StatusPending {
    #[bits(5..=7, r)]
    ns: u3,
    #[bit(4, r)]
    irq: bool,
    #[bits(0..=3, r)]
    ist: u4,
}

impl StatusPending {
    pub fn interrupt_id(&self) -> Option<CanInterruptId> {
        if !self.irq() && self.ist().value() == 0 {
            return Some(CanInterruptId::None);
        }

        if self.irq() && self.ist().value() == 0 {
            return Some(CanInterruptId::Error);
        }
        if !self.irq() {
            return None;
        }
        Some(CanInterruptId::Buffer(self.ist().as_usize() - 1))
    }
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct ErrorCounter {
    #[bits(0..=7, r)]
    transmit: u8,
    #[bits(8..=15, r)]
    receive: u8,
}

/// This register is unused for standard frames.
#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
pub struct ExtendedId {
    /// Mask for ID bits \[14:0\] of extended frames.
    #[bits(1..=15, rw)]
    mask_14_0: u15,
    /// CAN XRTR bit.
    #[bit(0, rw)]
    xrtr: bool,
}

#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
pub struct BaseId {
    /// This will contain ID\[10:0\] for standard frames and bits \[28:18\] for extended frames.
    #[bits(5..=15, rw)]
    mask_28_18: u11,
    /// This is the RTR bit for standard frames, and the SRR bit for extended frames.
    #[bit(4, rw)]
    rtr_or_srr: bool,
    /// Identifier extension bit.
    #[bit(3, rw)]
    ide: bool,
    /// Mask for ID bits \[17:15\] of extended frames.
    #[bits(0..=2, rw)]
    mask_17_15: u3,
}

#[derive(Debug, PartialEq, Eq)]
#[bitbybit::bitenum(u4, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrorFieldId {
    Error = 0b0000,
    ErrorDel = 0b0001,
    ErrorEcho = 0b0010,
    BusIdle = 0b0011,
    Ack = 0b0100,
    Eof = 0b0101,
    Intermission = 0b0110,
    SuspendTransmission = 0b0111,
    Sof = 0b1000,
    Arbitration = 0b1001,
    Ide = 0b1010,
    ExtendedArbitration = 0b1011,
    R1R0 = 0b1100,
    Dlc = 0b1101,
    Data = 0b1110,
    Crc = 0b1111,
}

#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct DiagnosticRegister {
    /// Shows the output value on the CAN TX pin at the time of the error.
    #[bit(14, r)]
    drive: bool,
    /// Shows the bus value on the CAN RX pin as sampled by the CAN module at the time of the
    /// error.
    #[bit(13, r)]
    mon: bool,
    /// Indicated whether the CRC is invalid. This bit should only be checked if the EFID field
    /// is [ErrorFieldId::Ack].
    #[bit(12, r)]
    crc: bool,
    /// Indicated whether the bit stuffing rule was violated at the time the error occured.
    #[bit(11, r)]
    stuff: bool,
    /// Indicated whether the CAN module was an active transmitter at the time the error occured.
    #[bit(10, r)]
    txe: bool,
    #[bits(4..=9, r)]
    ebid: u6,
    #[bits(0..=3, r)]
    efid: ErrorFieldId,
}

#[derive(derive_mmio::Mmio)]
#[mmio(const_inner)]
#[repr(C)]
pub struct Can {
    #[mmio(Inner)]
    cmbs: [CanMessageBuffer; 15],
    /// Hidden CAN message buffer. Only allowed to be used internally by the peripheral.
    #[mmio(Inner)]
    _hcmb: CanMessageBuffer,
    control: Control,
    timing: TimingConfig,
    /// Global mask extension used for buffers 0 to 13.
    gmskx: ExtendedId,
    /// Global mask base used for buffers 0 to 13.
    gmskb: BaseId,
    /// Basic mask extension used for buffer 14.
    bmskx: ExtendedId,
    /// Basic mask base used for buffer 14.
    bmskb: BaseId,
    ien: InterruptEnable,
    #[mmio(PureRead)]
    ipnd: InterruptPending,
    #[mmio(Write)]
    iclr: InterruptClear,
    /// Interrupt Code Enable Register.
    icen: InterruptEnable,
    #[mmio(PureRead)]
    status_pending: StatusPending,
    #[mmio(PureRead)]
    error_counter: ErrorCounter,
    #[mmio(PureRead)]
    diag: DiagnosticRegister,
    #[mmio(PureRead)]
    timer: u32,
}

static_assertions::const_assert_eq!(core::mem::size_of::<Can>(), 0x238);

impl Can {
    /// Create a new CAN MMIO instance for peripheral 0.
    ///
    /// # Safety
    ///
    /// This API can be used to potentially create a driver to the same peripheral structure
    /// from multiple threads. The user must ensure that concurrent accesses are safe and do not
    /// interfere with each other.
    pub const unsafe fn new_mmio_fixed_0() -> MmioCan<'static> {
        Self::new_mmio_at(CAN_0_BASE)
    }

    /// Create a new CAN MMIO instance for peripheral 1.
    ///
    /// # Safety
    ///
    /// This API can be used to potentially create a driver to the same peripheral structure
    /// from multiple threads. The user must ensure that concurrent accesses are safe and do not
    /// interfere with each other.
    pub const unsafe fn new_mmio_fixed_1() -> MmioCan<'static> {
        Self::new_mmio_at(CAN_1_BASE)
    }
}
