//! Analog to Digital Converter (ADC) driver.
//!
//! ## Examples
//!
//! - [ADC and DAC example](https://github.com/us-irs/va416xx-rs/blob/main/examples/simple/examples/dac-adc.rs)
//! - [ADC](https://github.com/us-irs/va416xx-rs/blob/main/examples/simple/examples/adc.rs)
use core::marker::PhantomData;

use crate::clock::Clocks;
use crate::pac;
use crate::time::Hertz;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use vorago_shared_hal::{enable_peripheral_clock, PeripheralSelect};

pub const ADC_MIN_CLK: Hertz = Hertz::from_raw(2_000_000);
pub const ADC_MAX_CLK: Hertz = Hertz::from_raw(12_500_000);

#[derive(Debug, PartialEq, Eq, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ChannelSelect {
    /// Analogue Input 0 external channel
    AnIn0 = 0,
    /// Analogue Input 1 external channel
    AnIn1 = 1,
    /// Analogue Input 2 external channel
    AnIn2 = 2,
    /// Analogue Input 3 external channel
    AnIn3 = 3,
    /// Analogue Input 4 external channel
    AnIn4 = 4,
    /// Analogue Input 5 external channel
    AnIn5 = 5,
    /// Analogue Input 6 external channel
    AnIn6 = 6,
    /// Analogue Input 7 external channel
    AnIn7 = 7,
    /// DAC 0 internal channel
    Dac0 = 8,
    /// DAC 1 internal channel
    Dac1 = 9,
    /// Internal temperature sensor
    TempSensor = 10,
    /// Internal bandgap 1 V reference
    Bandgap1V = 11,
    /// Internal bandgap 1.5 V reference
    Bandgap1_5V = 12,
    Avdd1_5 = 13,
    Dvdd1_5 = 14,
    /// Internally generated Voltage equal to VREFH / 2
    Vrefp5 = 15,
}

bitflags::bitflags! {
    /// This structure is used by the ADC multi-select API to
    /// allow selecting multiple channels in a convenient manner.
    #[derive(Debug)]
    pub struct MultiChannelSelect: u16 {
        const AnIn0 = 1;
        const AnIn1 = 1 << 1;
        const AnIn2 = 1 << 2;
        const AnIn3 = 1 << 3;
        const AnIn4 = 1 << 4;
        const AnIn5 = 1 << 5;
        const AnIn6 = 1 << 6;
        const AnIn7 = 1 << 7;
        const Dac0 = 1 << 8;
        const Dac1 = 1 << 9;
        const TempSensor = 1 << 10;
        const Bandgap1V = 1 << 11;
        const Bandgap1_5V = 1 << 12;
        const Avdd1_5 = 1 << 13;
        const Dvdd1_5 = 1 << 14;
        const Vrefp5 = 1 << 15;
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("ADC empty error")]
pub struct AdcEmptyError;

#[derive(Debug, PartialEq, Eq, Copy, Clone, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("invalid channel range error")]
pub struct InvalidChannelRangeError;

#[derive(Debug, PartialEq, Eq, Copy, Clone, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("buffer too small")]
pub struct BufferTooSmallError;

#[derive(Debug, PartialEq, Eq, Copy, Clone, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcRangeReadError {
    #[error("invalid channel range: {0}")]
    InvalidChannelRange(#[from] InvalidChannelRangeError),
    #[error("buffer too small: {0}")]
    BufferTooSmall(#[from] BufferTooSmallError),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ChannelValue {
    /// If the channel tag is enabled, this field will contain the determined channel tag.
    channel: ChannelSelect,
    /// Raw value.
    value: u16,
}

impl Default for ChannelValue {
    fn default() -> Self {
        Self {
            channel: ChannelSelect::AnIn0,
            value: Default::default(),
        }
    }
}

impl ChannelValue {
    #[inline]
    pub fn value(&self) -> u16 {
        self.value
    }

    #[inline]
    pub fn channel(&self) -> ChannelSelect {
        self.channel
    }
}

pub enum ChannelTagEnabled {}
pub enum ChannelTagDisabled {}

/// ADC driver structure.
///
/// Currently, this structure supports three primary ways to measure channel value(s):
///
/// * Trigger and read a single value
/// * Trigger and read a range of ADC values using the sweep functionality
/// * Trigger and read multiple ADC values using the sweep functionality
///
/// The ADC channel tag feature is enabled or disabled at compile time using the
/// [ChannelTagEnabled] and [ChannelTagDisabled]. The [Adc::new] method returns a driver instance
/// with the channel tag enabled, while the [Adc::new_with_channel_tag] method can be used to
/// return an instance with the channel tag enabled.
pub struct Adc<TagEnabled = ChannelTagDisabled> {
    adc: pac::Adc,
    phantom: PhantomData<TagEnabled>,
}

impl Adc<ChannelTagEnabled> {}

impl Adc<ChannelTagDisabled> {
    pub fn new(adc: pac::Adc, clocks: &Clocks) -> Self {
        Self::generic_new(adc, clocks)
    }

    pub fn trigger_and_read_single_channel(&self, ch: ChannelSelect) -> Result<u16, AdcEmptyError> {
        self.generic_trigger_and_read_single_channel(ch)
            .map(|v| v & 0xfff)
    }

    /// Perform a sweep for a specified range of ADC channels.
    ///
    /// Returns the number of read values which were written to the passed RX buffer.
    pub fn sweep_and_read_range(
        &self,
        lower_bound_idx: u8,
        upper_bound_idx: u8,
        rx_buf: &mut [u16],
    ) -> Result<usize, AdcRangeReadError> {
        self.generic_prepare_range_sweep_and_wait_until_ready(
            lower_bound_idx,
            upper_bound_idx,
            rx_buf.len(),
        )?;
        let fifo_entry_count = self.adc.status().read().fifo_entry_cnt().bits();
        for i in 0..core::cmp::min(fifo_entry_count, rx_buf.len() as u8) {
            rx_buf[i as usize] = self.adc.fifo_data().read().bits() as u16 & 0xfff;
        }
        Ok(fifo_entry_count as usize)
    }

    /// Perform a sweep for selected ADC channels.
    ///
    /// Returns the number of read values which were written to the passed RX buffer.
    pub fn sweep_and_read_multiselect(
        &self,
        ch_select: MultiChannelSelect,
        rx_buf: &mut [u16],
    ) -> Result<usize, BufferTooSmallError> {
        self.generic_prepare_multiselect_sweep_and_wait_until_ready(ch_select, rx_buf.len())?;
        let fifo_entry_count = self.adc.status().read().fifo_entry_cnt().bits();
        for i in 0..core::cmp::min(fifo_entry_count, rx_buf.len() as u8) {
            rx_buf[i as usize] = self.adc.fifo_data().read().bits() as u16 & 0xfff;
        }
        Ok(fifo_entry_count as usize)
    }

    pub fn try_read_single_value(&self) -> nb::Result<Option<u16>, ()> {
        self.generic_try_read_single_value()
            .map(|v| v.map(|v| v & 0xfff))
    }

    #[inline(always)]
    pub fn channel_tag_enabled(&self) -> bool {
        false
    }
}

impl Adc<ChannelTagEnabled> {
    pub fn new_with_channel_tag(adc: pac::Adc, clocks: &Clocks) -> Self {
        let mut adc = Self::generic_new(adc, clocks);
        adc.enable_channel_tag();
        adc
    }

    pub fn trigger_and_read_single_channel(
        &self,
        ch: ChannelSelect,
    ) -> Result<ChannelValue, AdcEmptyError> {
        self.generic_trigger_and_read_single_channel(ch)
            .map(|v| self.create_channel_value(v))
    }

    pub fn try_read_single_value(&self) -> nb::Result<Option<ChannelValue>, ()> {
        self.generic_try_read_single_value()
            .map(|v| v.map(|v| self.create_channel_value(v)))
    }

    /// Perform a sweep for a specified range of ADC channels.
    ///
    /// Returns the number of read values which were written to the passed RX buffer.
    pub fn sweep_and_read_range(
        &self,
        lower_bound_idx: u8,
        upper_bound_idx: u8,
        rx_buf: &mut [ChannelValue],
    ) -> Result<usize, AdcRangeReadError> {
        self.generic_prepare_range_sweep_and_wait_until_ready(
            lower_bound_idx,
            upper_bound_idx,
            rx_buf.len(),
        )?;
        let fifo_entry_count = self.adc.status().read().fifo_entry_cnt().bits();
        for i in 0..core::cmp::min(fifo_entry_count, rx_buf.len() as u8) {
            rx_buf[i as usize] =
                self.create_channel_value(self.adc.fifo_data().read().bits() as u16);
        }
        Ok(fifo_entry_count as usize)
    }

    /// Perform a sweep for selected ADC channels.
    ///
    /// Returns the number of read values which were written to the passed RX buffer.
    pub fn sweep_and_read_multiselect(
        &self,
        ch_select: MultiChannelSelect,
        rx_buf: &mut [ChannelValue],
    ) -> Result<usize, BufferTooSmallError> {
        self.generic_prepare_multiselect_sweep_and_wait_until_ready(ch_select, rx_buf.len())?;
        let fifo_entry_count = self.adc.status().read().fifo_entry_cnt().bits();
        for i in 0..core::cmp::min(fifo_entry_count, rx_buf.len() as u8) {
            rx_buf[i as usize] =
                self.create_channel_value(self.adc.fifo_data().read().bits() as u16);
        }
        Ok(fifo_entry_count as usize)
    }

    #[inline]
    pub fn create_channel_value(&self, raw_value: u16) -> ChannelValue {
        ChannelValue {
            value: raw_value & 0xfff,
            channel: ChannelSelect::try_from(((raw_value >> 12) & 0xf) as u8).unwrap(),
        }
    }

    #[inline(always)]
    pub fn channel_tag_enabled(&self) -> bool {
        true
    }
}

impl<TagEnabled> Adc<TagEnabled> {
    fn generic_new(adc: pac::Adc, _clocks: &Clocks) -> Self {
        enable_peripheral_clock(PeripheralSelect::Adc);
        adc.ctrl().write(|w| unsafe { w.bits(0) });
        let adc = Self {
            adc,
            phantom: PhantomData,
        };
        adc.clear_fifo();
        adc
    }

    #[inline(always)]
    fn enable_channel_tag(&mut self) {
        self.adc.ctrl().modify(|_, w| w.chan_tag_en().set_bit());
    }

    #[inline(always)]
    fn disable_channel_tag(&mut self) {
        self.adc.ctrl().modify(|_, w| w.chan_tag_en().clear_bit());
    }

    #[inline(always)]
    pub fn clear_fifo(&self) {
        self.adc.fifo_clr().write(|w| unsafe { w.bits(1) });
    }

    pub fn generic_try_read_single_value(&self) -> nb::Result<Option<u16>, ()> {
        if self.adc.status().read().adc_busy().bit_is_set() {
            return Err(nb::Error::WouldBlock);
        }
        if self.adc.status().read().fifo_entry_cnt().bits() == 0 {
            return Ok(None);
        }
        Ok(Some(self.adc.fifo_data().read().bits() as u16))
    }

    fn generic_trigger_single_channel(&self, ch: ChannelSelect) {
        self.adc.ctrl().modify(|_, w| {
            w.ext_trig_en().clear_bit();
            unsafe {
                // N + 1 conversions, so set set 0 here.
                w.conv_cnt().bits(0);
                w.chan_en().bits(1 << ch as u8)
            }
        });
        self.clear_fifo();

        self.adc.ctrl().modify(|_, w| w.manual_trig().set_bit());
    }

    fn generic_prepare_range_sweep_and_wait_until_ready(
        &self,
        lower_bound_idx: u8,
        upper_bound_idx: u8,
        buf_len: usize,
    ) -> Result<(), AdcRangeReadError> {
        if (lower_bound_idx > 15 || upper_bound_idx > 15) || lower_bound_idx > upper_bound_idx {
            return Err(InvalidChannelRangeError.into());
        }
        let ch_count = upper_bound_idx - lower_bound_idx + 1;
        if buf_len < ch_count as usize {
            return Err(BufferTooSmallError.into());
        }
        let mut ch_select = 0;
        for i in lower_bound_idx..upper_bound_idx + 1 {
            ch_select |= 1 << i;
        }
        self.generic_trigger_sweep(ch_select);
        while self.adc.status().read().adc_busy().bit_is_set() {
            cortex_m::asm::nop();
        }
        Ok(())
    }

    fn generic_prepare_multiselect_sweep_and_wait_until_ready(
        &self,
        ch_select: MultiChannelSelect,
        buf_len: usize,
    ) -> Result<(), BufferTooSmallError> {
        let ch_select = ch_select.bits();
        let ch_count = ch_select.count_ones();
        if buf_len < ch_count as usize {
            return Err(BufferTooSmallError);
        }
        self.generic_trigger_sweep(ch_select);
        while self.adc.status().read().adc_busy().bit_is_set() {
            cortex_m::asm::nop();
        }
        Ok(())
    }

    fn generic_trigger_sweep(&self, ch_select: u16) {
        let ch_num = ch_select.count_ones() as u8;
        assert!(ch_num > 0);
        self.adc.ctrl().modify(|_, w| {
            w.ext_trig_en().clear_bit();
            unsafe {
                // N + 1 conversions.
                w.conv_cnt().bits(0);
                w.chan_en().bits(ch_select);
                w.sweep_en().set_bit()
            }
        });
        self.clear_fifo();

        self.adc.ctrl().modify(|_, w| w.manual_trig().set_bit());
    }

    fn generic_trigger_and_read_single_channel(
        &self,
        ch: ChannelSelect,
    ) -> Result<u16, AdcEmptyError> {
        self.generic_trigger_single_channel(ch);
        nb::block!(self.generic_try_read_single_value())
            .unwrap()
            .ok_or(AdcEmptyError)
    }
}

impl From<Adc<ChannelTagDisabled>> for Adc<ChannelTagEnabled> {
    fn from(value: Adc<ChannelTagDisabled>) -> Self {
        let mut adc = Self {
            adc: value.adc,
            phantom: PhantomData,
        };
        adc.enable_channel_tag();
        adc
    }
}

impl From<Adc<ChannelTagEnabled>> for Adc<ChannelTagDisabled> {
    fn from(value: Adc<ChannelTagEnabled>) -> Self {
        let mut adc = Self {
            adc: value.adc,
            phantom: PhantomData,
        };
        adc.disable_channel_tag();
        adc
    }
}
