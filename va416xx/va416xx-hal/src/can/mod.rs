//! # CAN peripheral driver.
//!
//! The VA416xx CAN module is based on the CP3UB26 module.
//!
//! Using the CAN bus generally involves the following steps:
//!
//!  1. Create a [Can] instance
//!  2. The [CanChannels] resource management singleton can be retrieved by using
//!     [Can::take_channels].
//!  3. Individual [CanRx] and [CanTx] channels can be created using the [CanChannels::take]
//!     function. These allow to send or receive CAN frames on individual channels.
//!  4. The [asynch::CanTxAsync] structure can be created to transmit frames asynchronously.
//!     The [asynch::on_interrupt_can] function should be called in the user interrupt handler
//!     for CAN0 and CAN1 for this to work properly. The interrupt handler can also take care of
//!     receiving frames on [CanRx] channels with enabled interrupts.
//!
//! # Example
//!
//! - [CAN example](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/embassy/src/bin/can.rs)
use core::sync::atomic::AtomicBool;

use arbitrary_int::{prelude::*, u11, u15, u2, u3, u4, u7};
use embedded_can::Frame;
use ll::CanChannelLowLevel;
use regs::{BaseId, BufferState, Control, MmioCan, TimingConfig};
use vorago_shared_hal::enable_nvic_interrupt;

use crate::{clock::Clocks, enable_peripheral_clock, time::Hertz, PeripheralSelect};
use libm::roundf;

pub mod frame;
pub use frame::*;

pub mod asynch;
pub mod ll;
pub mod regs;

pub const PRESCALER_MIN: u8 = 2;
pub const PRESCALER_MAX: u8 = 128;
/// 1 is the minimum value, but not recommended by Vorago.
pub const TSEG1_MIN: u8 = 1;
pub const TSEG1_MAX: u8 = 16;
pub const TSEG2_MAX: u8 = 8;
/// In addition, SJW may not be larger than TSEG2.
pub const SJW_MAX: u8 = 4;

pub const MIN_SAMPLE_POINT: f32 = 0.5;
pub const MAX_BITRATE_DEVIATION: f32 = 0.005;

static CHANNELS_TAKEN: [AtomicBool; 2] = [AtomicBool::new(false), AtomicBool::new(false)];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanId {
    Can0 = 0,
    Can1 = 1,
}

impl CanId {
    /// Steal the register block for the CAN ID.
    ///
    /// # Safety
    ///
    /// See safety of the [regs::Can::new_mmio_fixed_0].
    #[inline]
    pub const unsafe fn steal_regs(&self) -> regs::MmioCan<'static> {
        match self {
            CanId::Can0 => unsafe { regs::Can::new_mmio_fixed_0() },
            CanId::Can1 => unsafe { regs::Can::new_mmio_fixed_1() },
        }
    }

    #[inline]
    pub const fn irq_id(&self) -> va416xx::Interrupt {
        match self {
            CanId::Can0 => va416xx::Interrupt::CAN0,
            CanId::Can1 => va416xx::Interrupt::CAN1,
        }
    }
}

/// Sample point between 0 and 1.0 for the given time segments.
pub const fn calculate_sample_point(tseg1: u8, tseg2: u8) -> f32 {
    let tseg1_val = tseg1 as f32;
    (tseg1_val + 1.0) / (1.0 + tseg1_val + tseg2 as f32)
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClockConfig {
    prescaler: u8,
    tseg1: u8,
    tseg2: u8,
    sjw: u8,
}

impl ClockConfig {
    /// New clock configuration from the raw configuration values.
    ///
    /// The values specified here are not the register values, but the actual numerical values
    /// relevant for calculations.
    ///
    /// The values have the following requirements:
    ///
    /// - Prescaler must be between 2 and 128.
    /// - TSEG1 must be smaller than 16 and should be larger than 1.
    /// - TSEG2 must be smaller than 8 and small enough so that the calculated sample point
    ///   is larger than 0.5 (50 %).
    /// - SJW (Synchronization Jump Width) must be smaller than the smaller of the time segment
    ///   configuration values and smaller than 4.
    pub fn new(prescaler: u8, tseg1: u8, tseg2: u8, sjw: u8) -> Result<Self, ClockConfigError> {
        if !(PRESCALER_MIN..=PRESCALER_MAX).contains(&prescaler.value()) {
            return Err(ClockConfigError::CanNotFindPrescaler);
        }
        if tseg1 == 0 || tseg2 == 0 {
            return Err(ClockConfigError::TsegIsZero);
        }
        if tseg1 > TSEG1_MAX {
            return Err(ClockConfigError::InvalidTseg1);
        }
        if tseg2 > TSEG2_MAX {
            return Err(ClockConfigError::InvalidTseg2);
        }
        let smaller_tseg = core::cmp::min(tseg1.value(), tseg2.value());
        if sjw.value() > smaller_tseg || sjw > SJW_MAX {
            return Err(InvalidSjwError(sjw).into());
        }
        let sample_point = calculate_sample_point(tseg1, tseg2);
        if sample_point < MIN_SAMPLE_POINT {
            return Err(InvalidSamplePointError { sample_point }.into());
        }
        Ok(Self {
            prescaler,
            tseg1,
            tseg2,
            sjw,
        })
    }

    /// Calculate the clock configuration for the given input clock, the target bitrate and for a
    /// set of timing parameters. The CAN controller uses the APB1 clock.
    ///
    /// This function basically calculates the necessary prescaler to achieve the given timing
    /// parameters. It also performs sanity and validity checks for the calculated prescaler:
    /// The bitrate error for the given prescaler needs to be smaller than 0.5 %.
    pub fn from_bitrate_and_segments(
        clocks: &Clocks,
        bitrate: Hertz,
        tseg1: u8,
        tseg2: u8,
        sjw: u8,
    ) -> Result<ClockConfig, ClockConfigError> {
        if bitrate.raw() == 0 {
            return Err(ClockConfigError::BitrateIsZero);
        }
        let nominal_bit_time = 1 + tseg1 as u32 + tseg2 as u32;
        let prescaler =
            roundf(clocks.apb1().raw() as f32 / (bitrate.raw() as f32 * nominal_bit_time as f32))
                as u32;
        if !(PRESCALER_MIN as u32..=PRESCALER_MAX as u32).contains(&prescaler) {
            return Err(ClockConfigError::CanNotFindPrescaler);
        }

        let actual_bitrate = (clocks.apb1().raw() as f32) / (prescaler * nominal_bit_time) as f32;
        let bitrate_deviation = calculate_bitrate_deviation(actual_bitrate, bitrate);
        if bitrate_deviation > MAX_BITRATE_DEVIATION {
            return Err(ClockConfigError::BitrateErrorTooLarge);
        }
        // The subtractions are fine because we made checks to avoid underflows.
        Self::new(prescaler as u8, tseg1, tseg2, sjw)
    }

    #[inline]
    pub fn sjw_reg_value(&self) -> u2 {
        u2::new(self.sjw.value() - 1)
    }

    #[inline]
    pub fn tseg1_reg_value(&self) -> u4 {
        u4::new(self.tseg1.value() - 1)
    }

    #[inline]
    pub fn tseg2_reg_value(&self) -> u3 {
        u3::new(self.tseg2.value() - 1)
    }

    #[inline]
    pub fn prescaler_reg_value(&self) -> u7 {
        u7::new(self.prescaler.value() - 2)
    }
}

/// Calculate all viable clock configurations for the given input clock, the target bitrate and
/// for a sample point between 0.5 and 1.0.
///
/// There are various recommendations for the sample point when using the CAN bus. The value
/// depends on different parameters like the bus length and propagation time, as well as
/// the information processing time of the nodes. It should always be at least 50 %.
/// In doubt, select a value like 0.75.
///
///  - The [Python CAN library](https://python-can.readthedocs.io/en/stable/bit_timing.html)
///    assumes a default value of 69 % as the sample point if none is specified.
///  - CiA-301 recommends 87.5 %
///  - For simpler setups like laboratory setups, smaller values should work as well.
///
/// A clock configuration is consideres viable when
///
///  - The sample point deviation is less than 5 %.
///  - The bitrate error is less than +-0.5 %.
///
///  SJW will be set to either TSEG2 or 4, whichever is smaller.
#[cfg(feature = "alloc")]
pub fn calculate_all_viable_clock_configs(
    apb1_clock: Hertz,
    bitrate: Hertz,
    sample_point: f32,
) -> Result<alloc::vec::Vec<(ClockConfig, f32)>, InvalidSamplePointError> {
    if sample_point < 0.5 || sample_point > 1.0 {
        return Err(InvalidSamplePointError { sample_point });
    }
    let mut configs = alloc::vec::Vec::new();
    for prescaler in PRESCALER_MIN..PRESCALER_MAX {
        let nom_bit_time = calculate_nominal_bit_time(apb1_clock, bitrate, prescaler);
        // This is taken from the Python CAN library. NBT should not be too small.
        if nom_bit_time < 8 {
            break;
        }
        let actual_bitrate = calculate_actual_bitrate(apb1_clock, prescaler, nom_bit_time);
        let bitrate_deviation = calculate_bitrate_deviation(actual_bitrate, bitrate);
        if bitrate_deviation > 0.05 {
            continue;
        }
        let tseg1 = roundf(sample_point * nom_bit_time as f32) as u32 - 1;
        if tseg1 > TSEG1_MAX as u32 || tseg1 < TSEG1_MIN as u32 {
            continue;
        }
        // limit tseg1, so tseg2 is at least 1 TQ
        let tseg1 = core::cmp::min(tseg1, nom_bit_time - 2) as u8;
        let tseg2 = nom_bit_time - tseg1 as u32 - 1;
        if tseg2 > TSEG2_MAX as u32 {
            continue;
        }
        let tseg2 = tseg2 as u8;
        let sjw = core::cmp::min(tseg2, 4) as u8;
        // Use percent to have a higher resolution for the sample point deviation.
        let sample_point_actual = roundf(calculate_sample_point(tseg1, tseg2) * 100.0) as u32;
        let sample_point = roundf(sample_point * 100.0) as u32;
        let deviation = (sample_point_actual as i32 - sample_point as i32).abs();
        if deviation > 5 {
            continue;
        }
        configs.push((
            ClockConfig {
                prescaler,
                tseg1,
                tseg2,
                sjw,
            },
            bitrate_deviation,
        ));
    }
    Ok(configs)
}

#[inline]
pub const fn calculate_nominal_bit_time(
    apb1_clock: Hertz,
    target_bitrate: Hertz,
    prescaler: u8,
) -> u32 {
    apb1_clock.raw() / (target_bitrate.raw() * prescaler as u32)
}

#[inline]
pub const fn calculate_actual_bitrate(apb1_clock: Hertz, prescaler: u8, nom_bit_time: u32) -> f32 {
    apb1_clock.raw() as f32 / (prescaler as u32 * nom_bit_time) as f32
}

#[inline]
pub const fn calculate_bitrate_deviation(actual_bitrate: f32, target_bitrate: Hertz) -> f32 {
    (actual_bitrate - target_bitrate.raw() as f32).abs() / target_bitrate.raw() as f32
}

pub trait CanInstance {
    const ID: CanId;
    const IRQ: va416xx::Interrupt;
    const PERIPH_SEL: PeripheralSelect;
}

impl CanInstance for va416xx::Can0 {
    const ID: CanId = CanId::Can0;
    const IRQ: va416xx::Interrupt = va416xx::Interrupt::CAN0;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Can0;
}

impl CanInstance for va416xx::Can1 {
    const ID: CanId = CanId::Can1;
    const IRQ: va416xx::Interrupt = va416xx::Interrupt::CAN1;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Can1;
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("invalid buffer index {0}")]
pub struct InvalidBufferIndexError(usize);

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("sjw must be less than or equal to the smaller tseg value")]
pub struct InvalidSjwError(u8);

#[derive(Debug, thiserror::Error)]
#[error("invalid sample point {sample_point}")]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidSamplePointError {
    /// Sample point, should be larger than 0.5 (50 %) but was not.
    sample_point: f32,
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockConfigError {
    #[error("invalid sjw: {0}")]
    InvalidSjw(#[from] InvalidSjwError),
    #[error("TSEG is zero which is not allowed")]
    TsegIsZero,
    #[error("TSEG1 is larger than 16")]
    InvalidTseg1,
    #[error("TSEG1 is larger than 8")]
    InvalidTseg2,
    #[error("invalid sample point: {0}")]
    InvalidSamplePoint(#[from] InvalidSamplePointError),
    #[error("bitrate is zero")]
    BitrateIsZero,
    #[error("bitrate error larger than +-0.5 %")]
    BitrateErrorTooLarge,
    #[error("maximum or minimum allowed prescaler is not sufficient for target bitrate clock")]
    CanNotFindPrescaler,
}

/// The main CAN peripheral driver.
pub struct Can {
    regs: regs::MmioCan<'static>,
    id: CanId,
}

impl Can {
    pub fn new<CanI: CanInstance>(_can: CanI, clk_config: ClockConfig) -> Self {
        enable_peripheral_clock(CanI::PERIPH_SEL);
        let id = CanI::ID;
        let mut regs = if id == CanId::Can0 {
            unsafe { regs::Can::new_mmio_fixed_0() }
        } else {
            unsafe { regs::Can::new_mmio_fixed_1() }
        };
        // Disable the CAN bus before configuring it.
        regs.write_control(Control::new_with_raw_value(0));
        for i in 0..15 {
            regs.cmbs(i).unwrap().reset();
        }
        regs.write_timing(
            TimingConfig::builder()
                .with_tseg2(clk_config.tseg2_reg_value())
                .with_tseg1(clk_config.tseg1_reg_value())
                .with_sync_jump_width(clk_config.sjw_reg_value())
                .with_prescaler(clk_config.prescaler_reg_value())
                .build(),
        );
        Self { regs, id }
    }

    /// This configures the global mask so that acceptance is only determined by an exact match
    /// with the ID in the receive message buffers. This is the default reset configuration for
    /// the global mask as well.
    pub fn set_global_mask_for_exact_id_match(&mut self) {
        self.regs
            .write_gmskx(regs::ExtendedId::new_with_raw_value(0));
        self.regs.write_gmskb(BaseId::new_with_raw_value(0));
    }

    /// Retrieve a resource management singleton for the 15 CAN channels.
    pub fn take_channels(&self) -> Option<CanChannels> {
        if CHANNELS_TAKEN[self.id() as usize].swap(true, core::sync::atomic::Ordering::SeqCst) {
            return None;
        }
        Some(CanChannels::new(self.id))
    }

    /// Similar to [Self::set_global_mask_for_exact_id_match] but masks the XRTR and RTR/SRR bits.
    ///
    /// This is useful for when transmitting remote frames with the RTR bit set. The hardware
    /// will automatically go into the [regs::BufferState::RxReady] state after the transmission,
    /// but the XRTR and RTR/SRR bits need to be masked for the response frame to be accepted
    /// on that buffer.
    pub fn set_global_mask_for_exact_id_match_with_rtr_masked(&mut self) {
        self.regs.write_gmskx(
            regs::ExtendedId::builder()
                .with_mask_14_0(u15::new(0))
                .with_xrtr(true)
                .build(),
        );
        self.regs.write_gmskb(
            BaseId::builder()
                .with_mask_28_18(u11::new(0))
                .with_rtr_or_srr(true)
                .with_ide(false)
                .with_mask_17_15(u3::new(0))
                .build(),
        );
    }

    /// This configures the base mask for buffer 14 so that acceptance is only determined by an
    /// exact match with the ID in the receive message buffers. This is the default reset
    /// configuration for the global mask as well.
    #[inline]
    pub fn set_base_mask_for_exact_id_match(&mut self) {
        self.regs
            .write_bmskx(regs::ExtendedId::new_with_raw_value(0));
        self.regs.write_bmskb(BaseId::new_with_raw_value(0));
    }

    /// This configures the base mask so that all CAN frames which are not handled by any other
    /// buffers are accepted by the base buffer 14.
    #[inline]
    pub fn set_base_mask_for_all_match(&mut self) {
        self.regs
            .write_bmskx(regs::ExtendedId::new_with_raw_value(0xffff));
        self.regs.write_bmskb(BaseId::new_with_raw_value(0xffff));
    }

    #[inline]
    pub fn regs(&mut self) -> &mut MmioCan<'static> {
        &mut self.regs
    }

    /// Clear all interrupts.
    #[inline]
    pub fn clear_interrupts(&mut self) {
        self.regs
            .write_iclr(regs::InterruptClear::new_with_raw_value(0xFFFF_FFFF));
    }

    /// This function only enable the CAN interrupt vector in the NVIC.
    ///
    /// The interrupts for the individual channels or errors still need to be enabled
    /// separately.
    #[inline]
    pub fn enable_nvic_interrupt(&mut self) {
        unsafe {
            enable_nvic_interrupt(self.id().irq_id());
        }
    }

    #[inline]
    pub fn read_error_counters(&self) -> regs::ErrorCounter {
        self.regs.read_error_counter()
    }

    #[inline]
    pub fn read_error_diagnostics(&self) -> regs::DiagnosticRegister {
        self.regs.read_diag()
    }

    #[inline]
    pub fn id(&self) -> CanId {
        self.id
    }

    #[inline]
    pub fn write_ctrl_reg(&mut self, ctrl: Control) {
        self.regs.write_control(ctrl);
    }

    #[inline]
    pub fn modify_control<F>(&mut self, f: F)
    where
        F: FnOnce(Control) -> Control,
    {
        self.regs.modify_control(f);
    }

    #[inline]
    pub fn set_bufflock(&mut self, enable: bool) {
        self.regs.modify_control(|mut ctrl| {
            ctrl.set_bufflock(enable);
            ctrl
        });
    }

    #[inline]
    pub fn enable(&mut self) {
        self.regs.modify_control(|mut ctrl| {
            ctrl.set_enable(true);
            ctrl
        });
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxState {
    Idle,
    TransmittingDataFrame,
    TransmittingRemoteFrame,
    AwaitingRemoteFrameReply,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InvalidTxState {
    State(TxState),
    BufferState(BufferState),
}

impl From<TxState> for InvalidTxState {
    fn from(state: TxState) -> Self {
        InvalidTxState::State(state)
    }
}

impl From<BufferState> for InvalidTxState {
    fn from(state: BufferState) -> Self {
        InvalidTxState::BufferState(state)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid tx state {0:?}")]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidTxStateError(pub InvalidTxState);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxState {
    Idle,
    Receiving,
}

#[derive(Debug, thiserror::Error)]
#[error("invalid rx state {0:?}")]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidRxStateError(pub RxState);

/// Driver instance to use an individual CAN channel as a transmission channel.
#[derive(Debug)]
pub struct CanTx {
    ll: CanChannelLowLevel,
    mode: TxState,
}

impl CanTx {
    pub fn new(mut ll: CanChannelLowLevel, tx_priority: Option<u4>) -> Self {
        ll.reset();
        ll.configure_for_transmission(tx_priority);
        Self {
            ll,
            mode: TxState::Idle,
        }
    }

    #[inline]
    pub fn into_rx_channel(self) -> CanRx {
        CanRx::new(self.ll)
    }

    /// Start transmitting a frame.
    ///
    /// The frame transmission can be polled/awaited to completion using the [Self::transfer_done]
    /// method.
    ///
    /// This function will return a [state error][InvalidTxStateError] if a transmission is already
    /// active and/or the transmit buffer has an invalid state.
    pub fn transmit_frame(&mut self, frame: CanFrame) -> Result<(), InvalidTxStateError> {
        if self.mode == TxState::AwaitingRemoteFrameReply {
            self.ll.configure_for_transmission(None);
            self.mode = TxState::Idle;
        }
        if self.mode != TxState::Idle {
            return Err(InvalidTxStateError(self.mode.into()));
        }
        if !frame.is_remote_frame() {
            self.mode = TxState::TransmittingDataFrame;
        } else {
            self.mode = TxState::TransmittingRemoteFrame;
        }
        if let Ok(state) = self.ll.read_state() {
            if state != BufferState::TxNotActive {
                return Err(InvalidTxStateError(state.into()));
            }
        }
        self.ll.transmit_frame_unchecked(frame);
        Ok(())
    }

    /// Poll whether an active data frame transmission is done.
    ///
    /// Returns a [state error][InvalidTxStateError] if no transmission is active.
    pub fn transfer_done(&mut self) -> nb::Result<(), InvalidTxStateError> {
        if self.mode != TxState::TransmittingDataFrame {
            return Err(nb::Error::Other(InvalidTxStateError(self.mode.into())));
        }
        let status = self.ll.read_state();
        if status.is_err() {
            return Err(nb::Error::WouldBlock);
        }
        let status = status.unwrap();
        if status == BufferState::TxNotActive {
            self.mode = TxState::Idle;
            return Ok(());
        }
        Err(nb::Error::WouldBlock)
    }

    /// Poll whether an active remote frame transmission is done.
    ///
    /// On success, returns the channel re-configured to a [CanRx] channel. This is because the
    /// default behaviour of the hardware will be to re-configure the channel state to
    /// [BufferState::RxReady] once the remote frame has been transmitted so that the response
    /// frame can be awaited.
    ///
    /// If the channel should instead be re-configured for transmission again,
    /// [Self::remote_transfer_done_with_tx_reconfig] can be used.
    ///
    /// Returns a [state error][InvalidTxStateError] if no transmission is active.
    pub fn remote_transfer_done(&mut self) -> nb::Result<CanRx, InvalidTxStateError> {
        if self.mode != TxState::TransmittingRemoteFrame {
            return Err(nb::Error::Other(InvalidTxStateError(self.mode.into())));
        }
        let status = self.ll.read_state();
        if status.is_err() {
            return Err(nb::Error::WouldBlock);
        }
        let status = status.unwrap();
        if status == BufferState::RxReady {
            self.mode = TxState::AwaitingRemoteFrameReply;
            return Ok(CanRx {
                ll: unsafe { self.ll.clone() },
                mode: RxState::Receiving,
            });
        }
        Err(nb::Error::WouldBlock)
    }

    /// Poll whether an active remote frame transmission is done.
    ///
    /// This function will re-configure the buffer back for transmission once the
    /// transmission has completed.
    ///
    /// Returns a [state error][InvalidTxStateError] if no transmission is active.
    pub fn remote_transfer_done_with_tx_reconfig(&mut self) -> nb::Result<(), InvalidTxStateError> {
        if self.mode != TxState::TransmittingRemoteFrame {
            return Err(nb::Error::Other(InvalidTxStateError(self.mode.into())));
        }
        let status = self.ll.read_state();
        if status.is_err() {
            return Err(nb::Error::WouldBlock);
        }
        let status = status.unwrap();
        if status == BufferState::RxReady {
            self.ll.write_state(BufferState::TxNotActive);
            self.mode = TxState::Idle;
            return Ok(());
        }
        Err(nb::Error::WouldBlock)
    }

    pub fn reset(&mut self) {
        self.ll.reset();
        self.mode = TxState::Idle;
    }
}

/// Driver instance to use an individual CAN channel as a reception channel.
pub struct CanRx {
    ll: CanChannelLowLevel,
    mode: RxState,
}

impl CanRx {
    pub fn new(mut ll: CanChannelLowLevel) -> Self {
        ll.reset();
        Self {
            ll,
            mode: RxState::Idle,
        }
    }

    #[inline]
    pub fn into_tx_channel(self, tx_priority: Option<u4>) -> CanTx {
        CanTx::new(self.ll, tx_priority)
    }

    #[inline]
    pub fn enable_interrupt(&mut self, enable_translation: bool) {
        self.ll.enable_interrupt(enable_translation);
    }

    pub fn configure_for_reception_with_standard_id(
        &mut self,
        standard_id: embedded_can::StandardId,
        set_rtr: bool,
    ) {
        self.ll.set_standard_id(standard_id, set_rtr);
        self.configure_for_reception();
    }

    pub fn configure_for_reception_with_extended_id(
        &mut self,
        extended_id: embedded_can::ExtendedId,
        set_rtr: bool,
    ) {
        self.ll.set_extended_id(extended_id, set_rtr);
        self.configure_for_reception();
    }

    pub fn configure_for_reception(&mut self) {
        self.ll.configure_for_reception();
        self.mode = RxState::Receiving;
    }

    #[inline]
    pub fn frame_available(&self) -> bool {
        self.ll
            .read_state()
            .is_ok_and(|state| state == BufferState::RxFull || state == BufferState::RxOverrun)
    }

    /// Poll for frame reception. Returns the frame if one is available.
    pub fn receive(
        &mut self,
        reconfigure_for_reception: bool,
    ) -> nb::Result<CanFrame, InvalidRxStateError> {
        if self.mode != RxState::Receiving {
            return Err(nb::Error::Other(InvalidRxStateError(self.mode)));
        }
        let status = self.ll.read_state();
        if status.is_err() {
            return Err(nb::Error::WouldBlock);
        }
        let status = status.unwrap();
        if status == BufferState::RxFull || status == BufferState::RxOverrun {
            self.mode = RxState::Idle;
            if reconfigure_for_reception {
                self.ll.write_state(BufferState::RxReady);
            }
            return Ok(self.ll.read_frame_unchecked());
        }
        Err(nb::Error::WouldBlock)
    }
}

pub struct CanChannels {
    id: CanId,
    channels: [Option<CanChannelLowLevel>; 15],
}

impl CanChannels {
    const fn new(id: CanId) -> Self {
        // Safety: Private function, ownership rules enforced by public API.
        unsafe {
            Self {
                id,
                channels: [
                    Some(CanChannelLowLevel::steal_unchecked(id, 0)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 1)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 2)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 3)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 4)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 5)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 6)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 7)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 8)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 9)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 10)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 11)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 12)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 13)),
                    Some(CanChannelLowLevel::steal_unchecked(id, 14)),
                ],
            }
        }
    }

    pub const fn can_id(&self) -> CanId {
        self.id
    }

    /// Take the indidivual CAN channel low level driver instance.
    pub fn take(&mut self, idx: usize) -> Option<CanChannelLowLevel> {
        if idx > 14 {
            return None;
        }
        self.channels[idx].take()
    }

    pub fn give(&mut self, idx: usize, channel: CanChannelLowLevel) {
        if idx > 14 {
            panic!("invalid buffer index for CAN channel");
        }
        self.channels[idx] = Some(channel);
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "alloc")]
    use std::println;

    #[cfg(feature = "alloc")]
    #[test]
    pub fn test_clock_calculator_example_1() {
        let configs = super::calculate_all_viable_clock_configs(
            crate::time::Hertz::from_raw(50_000_000),
            crate::time::Hertz::from_raw(25_000),
            0.75,
        )
        .expect("clock calculation failed");
        // Bitrate: 25278.05 Hz. Sample point: 0.7391
        assert_eq!(configs[0].prescaler, 84);
        assert_eq!(configs[0].tseg1, 16);
        assert_eq!(configs[0].tseg2, 6);
        assert_eq!(configs[0].sjw, 4);
        // Vorago sample value.
        let sample_cfg = configs
            .iter()
            .find(|c| c.prescaler == 100)
            .expect("clock config not found");
        // Slightly different distribution because we use a different sample point, but
        // the sum of TSEG1 and TSEG2 is the same as the Vorago example 1.
        assert_eq!(sample_cfg.tseg1, 14);
        assert_eq!(sample_cfg.tseg2, 5);
    }
}
