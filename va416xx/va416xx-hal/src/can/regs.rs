//! Custom register definitions for the CAN register block to circumvent PAC API / SVD
//! shortcomings.

use arbitrary_int::{prelude::*, u2, u3, u4, u6, u7, u11, u15};

/// CAN0 base address.
pub const CAN_0_BASE: usize = 0x4001_4000;
/// CAN1 base address.
pub const CAN_1_BASE: usize = 0x4001_4400;

/// State of an individual CAN message buffer.
#[derive(Debug, PartialEq, Eq)]
#[bitbybit::bitenum(u4)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BufferState {
    /// Passive channel.
    RxNotActive = 0b0000,
    /// This condition indicated that SW wrote RxNotActive to a buffer when a data copy
    /// process is still active.
    RxBusy = 0b0001,
    /// Buffer is ready to receive a frame.
    RxReady = 0b0010,
    /// Indicated that data is being copied for the first time (RxRead -> RxBusy0).
    RxBusy0 = 0b0011,
    /// Buffer holds a received frame.
    RxFull = 0b0100,
    /// Indicated that data is being copied for the second time (RxFull -> RxBusy2).
    RxBusy1 = 0b0101,
    /// A new frame was received before the previous one was read out.
    RxOverrun = 0b0110,
    /// Data is being copied out of a full buffer.
    RxBusy2 = 0b0111,
    /// Passive channel.
    TxNotActive = 0b1000,
    /// Automatical response to a remote frame.
    TxRtr = 0b1010,
    /// Transmit one frame.
    TxOnce = 0b1100,
    /// Data is being copied into the buffer for transmission.
    TxBusy0 = 0b1101,
    /// Transmit one frame, and changes to TxRtr after that. This can either be written by
    /// software, or it will be written by the hardware after an auto response of the
    /// [BufferState::TxRtr] state.
    TxOnceRtr = 0b1110,
    /// Data is being copied into the buffer for a repeated auto response.
    TxBusy2 = 0b1111,
}

/// Status control register for individual message buffers.
#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_fields(feature = "defmt"))]
pub struct BufStatusAndControl {
    /// Data length code.
    #[bits(12..=15, rw)]
    dlc: u4,
    /// Buffer priority, used to arbitrate between buffers ready to transmit at the same time.
    #[bits(4..=7, rw)]
    priority: u4,
    /// Current buffer state.
    #[bits(0..=3, rw)]
    state: Option<BufferState>,
}

/// Timestamp of a CAN message buffer.
#[derive(Debug)]
pub struct Timestamp(arbitrary_int::UInt<u32, 16>);

impl Timestamp {
    /// Create a new timestamp value.
    pub fn new(value: u16) -> Self {
        Self(value.into())
    }

    /// The raw timestamp value.
    pub fn value(&self) -> u16 {
        self.0.value() as u16
    }

    /// Overwrite the timestamp value.
    pub fn write(&mut self, value: u16) {
        self.0 = value.into();
    }
}

/// Two bytes of CAN frame data.
#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
pub struct TwoBytesData {
    /// Lower data byte.
    #[bits(0..=7, rw)]
    data_lower_byte: u8,
    /// Upper data byte.
    #[bits(8..=15, rw)]
    data_upper_byte: u8,
}

/// Individual CAN message buffer register block.
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
    /// Reset all registers of this message buffer to 0.
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

/// Logic level of a CAN pin.
#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PinLogicLevel {
    /// The dominant bus state is represented by a logic 0.
    DominantIsZero = 0b0,
    /// The dominant bus state is represented by a logic 1.
    DominantIsOne = 0b1,
}

/// Determines when the error interrupt pending bit is set.
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

/// Byte order used for CAN frame data.
#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataDirection {
    /// The first data byte is stored at the highest address.
    FirstByteAtHighestAddr = 0b0,
    /// The last data byte is stored at the highest address.
    LastByteAtHighestAddr = 0b1,
}

/// CONTROL register.
#[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
pub struct Control {
    /// Determines when the error interrupt pending bit is set.
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
    /// Byte order used for CAN frame data.
    #[bit(5, rw)]
    data_dir: DataDirection,
    /// Enable the timestamp counter.
    #[bit(4, rw)]
    timestamp_enable: bool,
    /// Lock all message buffers while a data copy operation is in progress.
    #[bit(3, rw)]
    bufflock: bool,
    /// Logic level of the CANTX pin.
    #[bit(2, rw)]
    tx_logic_level: PinLogicLevel,
    /// Logic level of the CANRX pin.
    #[bit(1, rw)]
    rx_logic_level: PinLogicLevel,
    /// Enable the CAN module.
    #[bit(0, rw)]
    enable: bool,
}

/// TIMING register.
#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_bitfields(feature = "defmt"))]
pub struct TimingConfig {
    /// Phase segment 2 length in time quanta.
    #[bits(0..=2, rw)]
    tseg2: u3,
    /// Phase segment 1 length in time quanta.
    #[bits(3..=6, rw)]
    tseg1: u4,
    /// Synchronization jump width in time quanta.
    #[bits(7..=8, rw)]
    sync_jump_width: u2,
    /// Baud rate prescaler.
    #[bits(9..=15, rw)]
    prescaler: u7,
}

/// IEN register.
#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct InterruptEnable {
    /// Enable the error interrupt.
    #[bit(15, rw)]
    error: bool,
    /// Enable the interrupt for each message buffer.
    #[bit(0, rw)]
    buffer: [bool; 15],
}

/// ICLR register.
#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct InterruptClear {
    /// Clear the error interrupt.
    #[bit(15, w)]
    error: bool,
    /// Clear the interrupt for each message buffer.
    #[bit(0, w)]
    buffer: [bool; 15],
}

/// IPND register.
#[bitbybit::bitfield(u32)]
#[derive(Debug)]
pub struct InterruptPending {
    /// Error interrupt is pending.
    #[bit(15, r)]
    error: bool,
    /// Interrupt for each message buffer is pending.
    #[bit(0, r)]
    buffer: [bool; 15],
}

/// Identifies the source of a pending CAN interrupt, see [StatusPending::interrupt_id].
#[derive(Debug)]
#[repr(usize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanInterruptId {
    /// No interrupt is pending.
    None = 0b00000,
    /// An error interrupt is pending.
    Error = 0b10000,
    /// An interrupt for the message buffer with the given index is pending.
    Buffer(usize),
}

/// STATUSPEND register.
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct StatusPending {
    /// Node status.
    #[bits(5..=7, r)]
    ns: u3,
    /// An error interrupt is pending.
    #[bit(4, r)]
    irq: bool,
    /// Index of the message buffer with a pending interrupt, if any.
    #[bits(0..=3, r)]
    ist: u4,
}

impl StatusPending {
    /// Identify the source of the pending interrupt, if any.
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

/// ERRCNT register.
#[bitbybit::bitfield(u32, debug, defmt_bitfields(feature = "defmt"))]
pub struct ErrorCounter {
    /// Transmit error counter.
    #[bits(0..=7, r)]
    transmit: u8,
    /// Receive error counter.
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

/// Base identifier register, used for standard frames and the lower bits of extended frames.
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

/// Identifies the CAN protocol field being processed when an error occurred.
#[derive(Debug, PartialEq, Eq)]
#[bitbybit::bitenum(u4, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrorFieldId {
    /// Error frame.
    Error = 0b0000,
    /// Error delimiter.
    ErrorDel = 0b0001,
    /// Error echo.
    ErrorEcho = 0b0010,
    /// Bus idle.
    BusIdle = 0b0011,
    /// Acknowledge field.
    Ack = 0b0100,
    /// End of frame.
    Eof = 0b0101,
    /// Intermission field.
    Intermission = 0b0110,
    /// Suspend transmission field.
    SuspendTransmission = 0b0111,
    /// Start of frame.
    Sof = 0b1000,
    /// Arbitration field.
    Arbitration = 0b1001,
    /// Identifier extension bit.
    Ide = 0b1010,
    /// Extended arbitration field.
    ExtendedArbitration = 0b1011,
    /// Reserved bits R1/R0.
    R1R0 = 0b1100,
    /// Data length code field.
    Dlc = 0b1101,
    /// Data field.
    Data = 0b1110,
    /// CRC field.
    Crc = 0b1111,
}

/// DIAG register.
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
    /// Bit ID of the bit being processed at the time of the error.
    #[bits(4..=9, r)]
    ebid: u6,
    /// Protocol field being processed at the time of the error.
    #[bits(0..=3, r)]
    efid: ErrorFieldId,
}

/// CAN peripheral register block.
#[derive(derive_mmio::Mmio)]
#[mmio(const_inner)]
#[repr(C)]
pub struct Can {
    /// The 15 regular CAN message buffers.
    #[mmio(Inner)]
    cmbs: [CanMessageBuffer; 15],
    /// Hidden CAN message buffer. Only allowed to be used internally by the peripheral.
    #[mmio(Inner)]
    _hcmb: CanMessageBuffer,
    /// CONTROL register.
    control: Control,
    /// TIMING register.
    timing: TimingConfig,
    /// Global mask extension used for buffers 0 to 13.
    gmskx: ExtendedId,
    /// Global mask base used for buffers 0 to 13.
    gmskb: BaseId,
    /// Basic mask extension used for buffer 14.
    bmskx: ExtendedId,
    /// Basic mask base used for buffer 14.
    bmskb: BaseId,
    /// Interrupt Enable Register.
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
    /// Free-running timestamp timer.
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
        unsafe { Self::new_mmio_at(CAN_0_BASE) }
    }

    /// Create a new CAN MMIO instance for peripheral 1.
    ///
    /// # Safety
    ///
    /// This API can be used to potentially create a driver to the same peripheral structure
    /// from multiple threads. The user must ensure that concurrent accesses are safe and do not
    /// interfere with each other.
    pub const unsafe fn new_mmio_fixed_1() -> MmioCan<'static> {
        unsafe { Self::new_mmio_at(CAN_1_BASE) }
    }
}
