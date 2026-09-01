#![no_std]
#![no_main]
use embassy_example as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker};
use once_cell::sync::OnceCell;

use va108xx_hal::{
    gpio::{Output, PinState},
    pac::{self, interrupt},
    pins::{PinsA, PinsB},
    prelude::*,
    spi,
};

const SYSCLK_FREQ: Hertz = Hertz::from_raw(50_000_000);

/// Token identifying the SPI0 peripheral, set once at construction and read by the interrupt
/// handler, which does not have access to the [spi::asynch::Spi] driver itself.
static SPI_TOKEN: OnceCell<spi::Bank> = OnceCell::new();

// main is itself an async function.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::println!("-- VA108xx Async SPI Example --");

    let dp = pac::Peripherals::take().unwrap();

    // Safety: Only called once here.
    va108xx_hal::embassy_time::init(dp.tim23, dp.tim22, SYSCLK_FREQ);

    let porta = PinsA::new(dp.porta);
    let _portb = PinsB::new(dp.portb);
    let mut led0 = Output::new(porta.pa10, PinState::Low);
    let mut led1 = Output::new(porta.pa7, PinState::Low);
    let mut led2 = Output::new(porta.pa6, PinState::Low);

    let spi_clk_cfg = spi::ClockConfig::from_clk(50.MHz(), 1.MHz()).unwrap();
    let mut spi_cfg = spi::Config::default();
    spi_cfg.clock = spi_clk_cfg;
    let (sck, mosi, miso) = (porta.pa31, porta.pa30, porta.pa29);
    let spi = spi::Spi::<u8>::new_for_spi0(dp.spia, (sck, miso, mosi), spi_cfg);

    let mut spi = spi.into_async(Some(va108xx_hal::InterruptConfig::new(
        va108xx_hal::pac::Interrupt::OC2,
        true,
        true,
    )));
    SPI_TOKEN.set(spi.bank_id()).unwrap();
    let mut ticker = Ticker::every(Duration::from_secs(1));
    let buf: [u8; 4] = [0xAA; 4];
    loop {
        spi.write(&buf).await.expect("spi transfer failed");
        defmt::info!("async SPI transfer done");
        ticker.next().await;
        led0.toggle();
        led1.toggle();
        led2.toggle();
    }
}

#[interrupt]
fn OC2() {
    spi::asynch::Spi::on_interrupt(*SPI_TOKEN.get().unwrap());
}
