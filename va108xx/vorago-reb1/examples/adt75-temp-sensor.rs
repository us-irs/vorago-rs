#![no_main]
#![no_std]
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::{pac, prelude::*, timer::CountdownTimer};
use vorago_reb1::temp_sensor::Adt75TempSensor;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- Vorago Temperature Sensor and I2C Example --");
    let dp = pac::Peripherals::take().unwrap();
    let mut delay = CountdownTimer::new(dp.tim0, 50.MHz());
    let mut temp_sensor =
        Adt75TempSensor::new(50.MHz(), dp.i2ca).expect("Creating temperature sensor struct failed");
    loop {
        let temp = temp_sensor
            .read_temperature()
            .expect("Failed reading temperature");
        rprintln!("Temperature in Celcius: {}", temp);
        delay.delay_ms(500);
    }
}
