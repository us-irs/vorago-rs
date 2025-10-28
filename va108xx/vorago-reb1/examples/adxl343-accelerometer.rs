//! ADXL343 accelerometer example
//!
//! Please note that the default REB1 board is not populated with the ADXL343BCCZ-RL7.
//! To use this example, this chip needs to be soldered onto the board.
#![no_main]
#![no_std]
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::spi::{SpiBus, MODE_3};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::gpio::{Output, PinState};
use va108xx_hal::pins::PinsA;
use va108xx_hal::spi::{configure_pin_as_hw_cs_pin, SpiClockConfig};
use va108xx_hal::timer::CountdownTimer;
use va108xx_hal::{
    pac,
    prelude::*,
    spi::{Spi, SpiConfig},
};

const READ_MASK: u8 = 1 << 7;
const _MULTI_BYTE_MASK: u8 = 1 << 6;
const DEVID_REG: u8 = 0x00;

const POWER_CTL_REG: u8 = 0x2D;
const PWR_MEASUREMENT_MODE_MASK: u8 = 1 << 3;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- Vorago Accelerometer Example --");
    let dp = pac::Peripherals::take().unwrap();
    let pinsa = PinsA::new(dp.porta);
    let mut delay = CountdownTimer::new(dp.tim0, 50.MHz());
    let (sck, mosi, miso) = (pinsa.pa20, pinsa.pa19, pinsa.pa18);
    let cs_pin = pinsa.pa16;
    let hw_cs_id = configure_pin_as_hw_cs_pin(cs_pin);

    // Need to set the ADC chip select low
    Output::new(pinsa.pa17, PinState::Low);

    let spi_cfg = SpiConfig::default()
        .clk_cfg(
            SpiClockConfig::from_clk(50.MHz(), 1.MHz()).expect("creating SPI clock config failed"),
        )
        .mode(MODE_3)
        .slave_output_disable(true);
    let mut spi = Spi::new(dp.spib, (sck, miso, mosi), spi_cfg).unwrap();
    spi.cfg_hw_cs(hw_cs_id);

    let mut tx_rx_buf: [u8; 3] = [0; 3];
    tx_rx_buf[0] = READ_MASK | DEVID_REG;
    spi.transfer_in_place(&mut tx_rx_buf[0..2])
        .expect("Reading DEVID register failed");
    rprintln!("DEVID register: {}", tx_rx_buf[1]);

    tx_rx_buf[0] = POWER_CTL_REG;
    tx_rx_buf[1] = PWR_MEASUREMENT_MODE_MASK;
    spi.write(&tx_rx_buf[0..2])
        .expect("Enabling measurement mode failed");

    loop {
        delay.delay_ms(500);
    }
}
