//! This module provides a thin REB1 specific layer on top of the `max116xx_10bit` driver crate
//!
//! ## Examples
//!
//! - [ADC example](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/vorago-reb1/examples/max11619-adc.rs)
use core::convert::Infallible;
use embedded_hal::spi::SpiDevice;
use max116xx_10bit::{
    Error, ExternallyClocked, InternallyClockedInternallyTimedSerialInterface, Max116xx10Bit,
    Max116xx10BitEocExt, VoltageRefMode, WithWakeupDelay, WithoutWakeupDelay,
};
use va108xx_hal::gpio::Input;

pub type Max11619ExternallyClockedNoWakeup<Spi> =
    Max116xx10Bit<Spi, ExternallyClocked, WithoutWakeupDelay>;
pub type Max11619ExternallyClockedWithWakeup<Spi> =
    Max116xx10Bit<Spi, ExternallyClocked, WithWakeupDelay>;
pub type Max11619InternallyClocked<Spi, Eoc> =
    Max116xx10BitEocExt<Spi, Eoc, InternallyClockedInternallyTimedSerialInterface>;

pub const AN0_CHANNEL: u8 = 0;
pub const AN1_CHANNEL: u8 = 1;
pub const AN2_CHANNEL: u8 = 2;
pub const POTENTIOMETER_CHANNEL: u8 = 3;

pub fn max11619_externally_clocked_no_wakeup<Spi: SpiDevice>(
    spi: Spi,
) -> Result<Max11619ExternallyClockedNoWakeup<Spi>, Error<Spi::Error, Infallible>> {
    let mut adc = Max116xx10Bit::max11619(spi)?;
    adc.reset(false)?;
    adc.setup()?;
    Ok(adc)
}

pub fn max11619_externally_clocked_with_wakeup<Spi: SpiDevice>(
    spi: Spi,
) -> Result<Max11619ExternallyClockedWithWakeup<Spi>, Error<Spi::Error, Infallible>> {
    let mut adc = Max116xx10Bit::max11619(spi)?.into_ext_clkd_with_int_ref_wakeup_delay();
    adc.reset(false)?;
    adc.setup()?;
    Ok(adc)
}

pub fn max11619_internally_clocked<Spi: SpiDevice>(
    spi: Spi,
    eoc: Input,
    v_ref: VoltageRefMode,
) -> Result<Max11619InternallyClocked<Spi, Input>, Error<Spi::Error, Infallible>> {
    let mut adc = Max116xx10Bit::max11619(spi)?
        .into_int_clkd_int_timed_through_ser_if_without_wakeup(v_ref, eoc)?;
    adc.reset(false)?;
    adc.setup()?;
    Ok(adc)
}
