//! MAX11619 ADC example application.
//!
//! You can turn the potentiometer knob of the REB1 board to measure
//! different ADC values.
#![no_main]
#![no_std]

use core::convert::Infallible;

use cortex_m_rt::entry;
use embedded_hal::spi::{SpiBus, SpiDevice, MODE_0};
use embedded_hal::{delay::DelayNs, spi};
use max116xx_10bit::VoltageRefMode;
use max116xx_10bit::{AveragingConversions, AveragingResults};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::gpio::{Input, Output, PinState, Port};
use va108xx_hal::pins::PinsA;
use va108xx_hal::spi::{configure_pin_as_hw_cs_pin, SpiClockConfig};
use va108xx_hal::timer::CountdownTimer;
use va108xx_hal::{
    pac,
    prelude::*,
    spi::{HwChipSelectId, Spi, SpiConfig},
};
use va108xx_hal::{port_function_select, FunctionSelect};
use vorago_reb1::max11619::{
    max11619_externally_clocked_no_wakeup, max11619_externally_clocked_with_wakeup,
    max11619_internally_clocked, AN2_CHANNEL, POTENTIOMETER_CHANNEL,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ExampleMode {
    UsingEoc,
    NotUsingEoc,
    NotUsingEocWithDelay,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ReadMode {
    Single,
    Multiple,
    MultipleNToHighest,
    AverageN,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MuxMode {
    None,
    PortB19to17,
}

const EXAMPLE_MODE: ExampleMode = ExampleMode::NotUsingEoc;
const READ_MODE: ReadMode = ReadMode::Multiple;
const MUX_MODE: MuxMode = MuxMode::None;

// This is probably more or less a re-implementation of https://docs.rs/embedded-hal-bus/latest/embedded_hal_bus/spi/struct.ExclusiveDevice.html.
// Users should look at the embedded-hal-bus crate for sharing the bus.
pub struct SpiWithHwCs<Delay> {
    inner: Spi<u8>,
    delay_provider: Delay,
    hw_cs_id: HwChipSelectId,
}

impl<Delay: DelayNs> SpiWithHwCs<Delay> {
    pub fn new(spi: Spi<u8>, hw_cs_id: HwChipSelectId, delay_provider: Delay) -> Self {
        Self {
            inner: spi,
            hw_cs_id,
            delay_provider,
        }
    }
}

impl<Delay> embedded_hal::spi::ErrorType for SpiWithHwCs<Delay> {
    type Error = Infallible;
}

impl<Delay: DelayNs> SpiDevice for SpiWithHwCs<Delay> {
    fn transaction(
        &mut self,
        operations: &mut [spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        // Only the HW CS is configured here. This is not really necessary, but showcases
        // that we could scale this multiple SPI devices.
        self.inner.cfg_hw_cs(self.hw_cs_id);
        for operation in operations {
            match operation {
                spi::Operation::Read(buf) => self.inner.read(buf).ok().unwrap(),
                spi::Operation::Write(buf) => self.inner.write(buf).ok().unwrap(),
                spi::Operation::Transfer(read, write) => {
                    self.inner.transfer(read, write).ok().unwrap()
                }
                spi::Operation::TransferInPlace(buf) => {
                    self.inner.transfer_in_place(buf).ok().unwrap()
                }
                spi::Operation::DelayNs(delay) => self.delay_provider.delay_ns(*delay),
            };
        }
        self.inner.cfg_hw_cs_disable();
        Ok(())
    }
}

const SYS_CLK: Hertz = Hertz::from_raw(50_000_000);

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- Vorago ADC Example --");

    let mut dp = pac::Peripherals::take().unwrap();
    let mut delay = CountdownTimer::new(dp.tim0, SYS_CLK);
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::OC0);
    }

    let pinsa = PinsA::new(dp.porta);
    let spi_cfg = SpiConfig::default()
        .clk_cfg(SpiClockConfig::from_clk(SYS_CLK, 3.MHz()).unwrap())
        .mode(MODE_0)
        .blockmode(true);
    let (sck, mosi, miso) = (pinsa.pa20, pinsa.pa19, pinsa.pa18);

    if MUX_MODE == MuxMode::PortB19to17 {
        port_function_select(&mut dp.ioconfig, Port::B, 19, FunctionSelect::Sel1).ok();
        port_function_select(&mut dp.ioconfig, Port::B, 18, FunctionSelect::Sel2).ok();
        port_function_select(&mut dp.ioconfig, Port::B, 17, FunctionSelect::Sel1).ok();
        port_function_select(&mut dp.ioconfig, Port::B, 16, FunctionSelect::Sel1).ok();
    }
    // Set the accelerometer chip select low in case the board slot is populated
    Output::new(pinsa.pa16, PinState::Low);

    let hw_cs_id = configure_pin_as_hw_cs_pin(pinsa.pa17);
    let spi = Spi::new(dp.spib, (sck, miso, mosi), spi_cfg).unwrap();

    let delay_spi = CountdownTimer::new(dp.tim1, SYS_CLK);
    let spi_with_hwcs = SpiWithHwCs::new(spi, hw_cs_id, delay_spi);
    match EXAMPLE_MODE {
        ExampleMode::NotUsingEoc => spi_example_externally_clocked(spi_with_hwcs, &mut delay),
        ExampleMode::UsingEoc => {
            spi_example_internally_clocked(
                spi_with_hwcs,
                &mut delay,
                Input::new_floating(pinsa.pa14),
            );
        }
        ExampleMode::NotUsingEocWithDelay => {
            spi_example_externally_clocked_with_delay(spi_with_hwcs, &mut delay);
        }
    }
}

/// Use the SPI clock as the conversion clock
fn spi_example_externally_clocked(spi: impl SpiDevice, delay: &mut impl DelayNs) -> ! {
    let mut adc = max11619_externally_clocked_no_wakeup(spi)
        .expect("Creating externally clocked MAX11619 device failed");
    if READ_MODE == ReadMode::AverageN {
        adc.averaging(
            AveragingConversions::FourConversions,
            AveragingResults::FourResults,
        )
        .expect("Error setting up averaging register");
    }
    let mut cmd_buf: [u8; 32] = [0; 32];
    let mut counter = 0;
    loop {
        rprintln!("-- Measurement {} --", counter);

        match READ_MODE {
            ReadMode::Single => {
                rprintln!("Reading single potentiometer channel");
                let pot_val = adc
                    .read_single_channel(&mut cmd_buf, POTENTIOMETER_CHANNEL)
                    .expect("Creating externally clocked MAX11619 ADC failed");
                rprintln!("Single channel read:");
                rprintln!("\tPotentiometer value: {}", pot_val);
            }
            ReadMode::Multiple => {
                let mut res_buf: [u16; 4] = [0; 4];
                adc.read_multiple_channels_0_to_n(
                    &mut cmd_buf,
                    &mut res_buf.iter_mut(),
                    POTENTIOMETER_CHANNEL,
                )
                .expect("Multi-Channel read failed");
                print_res_buf(&res_buf);
            }
            ReadMode::MultipleNToHighest => {
                let mut res_buf: [u16; 2] = [0; 2];
                adc.read_multiple_channels_n_to_highest(
                    &mut cmd_buf,
                    &mut res_buf.iter_mut(),
                    AN2_CHANNEL,
                )
                .expect("Multi-Channel read failed");
                rprintln!("Multi channel read from 2 to 3:");
                rprintln!("\tAN2 value: {}", res_buf[0]);
                rprintln!("\tAN3 / Potentiometer value: {}", res_buf[1]);
            }
            ReadMode::AverageN => {
                rprintln!("Scanning and averaging not possible for externally clocked mode");
            }
        }
        counter += 1;
        delay.delay_ms(500);
    }
}

fn spi_example_externally_clocked_with_delay(spi: impl SpiDevice, delay: &mut impl DelayNs) -> ! {
    let mut adc =
        max11619_externally_clocked_with_wakeup(spi).expect("Creating MAX116xx device failed");
    let mut cmd_buf: [u8; 32] = [0; 32];
    let mut counter = 0;
    loop {
        rprintln!("-- Measurement {} --", counter);

        match READ_MODE {
            ReadMode::Single => {
                rprintln!("Reading single potentiometer channel");
                let pot_val = adc
                    .read_single_channel(&mut cmd_buf, POTENTIOMETER_CHANNEL, delay)
                    .expect("Creating externally clocked MAX11619 ADC failed");
                rprintln!("Single channel read:");
                rprintln!("\tPotentiometer value: {}", pot_val);
            }
            ReadMode::Multiple => {
                let mut res_buf: [u16; 4] = [0; 4];
                adc.read_multiple_channels_0_to_n(
                    &mut cmd_buf,
                    &mut res_buf.iter_mut(),
                    POTENTIOMETER_CHANNEL,
                    delay,
                )
                .expect("Multi-Channel read failed");
                print_res_buf(&res_buf);
            }
            ReadMode::MultipleNToHighest => {
                let mut res_buf: [u16; 2] = [0; 2];
                adc.read_multiple_channels_n_to_highest(
                    &mut cmd_buf,
                    &mut res_buf.iter_mut(),
                    AN2_CHANNEL,
                    delay,
                )
                .expect("Multi-Channel read failed");
                rprintln!("Multi channel read from 2 to 3:");
                rprintln!("\tAN2 value: {}", res_buf[0]);
                rprintln!("\tAN3 / Potentiometer value: {}", res_buf[1]);
            }
            ReadMode::AverageN => {
                rprintln!("Scanning and averaging not possible for externally clocked mode");
            }
        }
        counter += 1;
        delay.delay_ms(500);
    }
}

/// This function uses the EOC pin to determine whether the conversion finished
fn spi_example_internally_clocked(
    spi: impl SpiDevice,
    delay: &mut impl DelayNs,
    eoc_pin: Input,
) -> ! {
    let mut adc = max11619_internally_clocked(
        spi,
        eoc_pin,
        VoltageRefMode::ExternalSingleEndedNoWakeupDelay,
    )
    .expect("Creating MAX116xx device failed");
    let mut counter = 0;
    loop {
        rprintln!("-- Measurement {} --", counter);

        match READ_MODE {
            ReadMode::Single => {
                adc.request_single_channel(POTENTIOMETER_CHANNEL)
                    .expect("Requesting single channel value  failed");

                let pot_val = nb::block!(adc.get_single_channel())
                    .expect("Reading single channel value  failed");
                rprintln!("\tPotentiometer value: {}", pot_val);
            }
            ReadMode::Multiple => {
                adc.request_multiple_channels_0_to_n(POTENTIOMETER_CHANNEL)
                    .expect("Requesting single channel value  failed");
                let mut res_buf: [u16; 4] = [0; 4];
                nb::block!(adc.get_multi_channel(&mut res_buf.iter_mut()))
                    .expect("Requesting multiple channel values failed");
                print_res_buf(&res_buf);
            }
            ReadMode::MultipleNToHighest => {
                adc.request_multiple_channels_n_to_highest(AN2_CHANNEL)
                    .expect("Requesting single channel value  failed");
                let mut res_buf: [u16; 4] = [0; 4];
                nb::block!(adc.get_multi_channel(&mut res_buf.iter_mut()))
                    .expect("Requesting multiple channel values failed");
                rprintln!("Multi channel read from 2 to 3:");
                rprintln!("\tAN2 value: {}", res_buf[0]);
                rprintln!("\tAN3 / Potentiometer value: {}", res_buf[1]);
            }
            ReadMode::AverageN => {
                adc.request_channel_n_repeatedly(POTENTIOMETER_CHANNEL)
                    .expect("Reading channel multiple times failed");
                let mut res_buf: [u16; 16] = [0; 16];
                nb::block!(adc.get_multi_channel(&mut res_buf.iter_mut()))
                    .expect("Requesting multiple channel values failed");
                rprintln!("Reading potentiometer 4 times");
                rprintln!("\tValue 0: {}", res_buf[0]);
                rprintln!("\tValue 1: {}", res_buf[1]);
                rprintln!("\tValue 2: {}", res_buf[2]);
                rprintln!("\tValue 3: {}", res_buf[3]);
            }
        }

        counter += 1;
        delay.delay_ms(500);
    }
}

fn print_res_buf(buf: &[u16; 4]) {
    rprintln!("Multi channel read from 0 to 3:");
    rprintln!("\tAN0 value: {}", buf[0]);
    rprintln!("\tAN1 value: {}", buf[1]);
    rprintln!("\tAN2 value: {}", buf[2]);
    rprintln!("\tAN3 / Potentiometer value: {}", buf[3]);
}
