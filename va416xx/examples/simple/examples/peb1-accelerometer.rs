//! Example code for the PEB1 development board accelerometer.
#![no_main]
#![no_std]

// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use accelerometer::{Accelerometer, RawAccelerometer};
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use simple_examples::peb1;
use va416xx_hal::{
    clock::ClockConfigurator,
    i2c,
    pac::{self},
    prelude::*,
    timer::CountdownTimer,
};
use vorago_peb1::lis2dh12::{self, detect_i2c_addr, FullScale, Odr};

pub enum DisplayMode {
    Raw,
    Normalized,
}

const DISPLAY_MODE: DisplayMode = DisplayMode::Normalized;

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    defmt::println!("-- Vorago PEB1 accelerometer example --");
    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze()
        .unwrap();
    let mut i2c_master = i2c::I2cMaster::new(
        dp.i2c0,
        &clocks,
        i2c::MasterConfig::default(),
        i2c::I2cSpeed::Regular100khz,
    )
    .expect("creating I2C master failed");
    let mut delay_provider = CountdownTimer::new(dp.tim1, &clocks);
    // Detect the I2C address of the accelerometer by scanning all possible values.
    let slave_addr = detect_i2c_addr(&mut i2c_master).expect("detecting I2C address failed");
    // Create the accelerometer driver using the PEB1 BSP.
    let mut accelerometer = vorago_peb1::accelerometer::new_with_i2cm(i2c_master, slave_addr)
        .expect("creating accelerometer driver failed");
    let device_id = accelerometer.get_device_id().unwrap();
    accelerometer
        .set_mode(lis2dh12::Mode::Normal)
        .expect("setting mode failed");
    accelerometer
        .set_odr(Odr::Hz100)
        .expect("setting ODR failed");
    accelerometer
        .set_fs(FullScale::G4)
        .expect("setting full scale failed");
    // This function also enabled BDU.
    accelerometer
        .enable_temp(true)
        .expect("enabling temperature sensor failed");
    defmt::info!("Device ID: 0x{:02X}", device_id);
    // Start reading the accelerometer periodically.
    loop {
        let temperature = accelerometer
            .get_temp_outf()
            .expect("reading temperature failed");
        match DISPLAY_MODE {
            DisplayMode::Normalized => {
                let value = accelerometer
                    .accel_norm()
                    .expect("reading normalized accelerometer data failed");
                defmt::info!(
                    "Accel Norm F32x3 {{ x: {:05}, y: {:05}, z:{:05}}} | Temp {} °C",
                    value.x,
                    value.y,
                    value.z,
                    temperature
                );
            }
            DisplayMode::Raw => {
                let value_raw = accelerometer
                    .accel_raw()
                    .expect("reading raw accelerometer data failed");
                defmt::info!(
                    "Accel Raw I32x3 {{ x: {:05}, y: {:05}, z:{:05}}} | Temp {} °C",
                    value_raw.x,
                    value_raw.y,
                    value_raw.z,
                    temperature
                );
            }
        }
        delay_provider.delay_ms(100);
    }
}
