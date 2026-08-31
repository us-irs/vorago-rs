//! Asynchronous SPI example application.
//!
//! Uses SPI2 on PORTA with PA5 as SCK, PA6 as MISO and PA7 as MOSI. These are the pin
//! assignments for the PMOD_SPI_2A connector (J35) on the PEB1 board.
//!
//! PA4 is used as a hardware chip select. The SPI peripheral asserts it for the duration of a
//! transfer and deasserts it after the last word, so the application never toggles it.
//!
//! If you do not use loopback mode, MOSI and MISO need to be tied together on the board.
//!
//! Setting [TEST_TRANSFER_CANCELLATION] adds a cancelled transfer to each cycle to check what
//! the chip select does when a transfer is dropped in flight. See the constant for details.
#![no_std]
#![no_main]
use defmt_rtt as _;
use panic_probe as _;

use embassy_example::EXTCLK_FREQ;
use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker};

use va416xx_hal::{
    clock::ClockConfigurator,
    gpio::{Output, PinState},
    pac::{self, interrupt},
    pins::PinsA,
    prelude::*,
    spi::{self, Bank, SpiClockConfig},
    time::Hertz,
};

/// Drop a transfer future which is still in flight, once per cycle.
///
/// The future prefills the FIFO and releases the TX pause when it is constructed, so the frame
/// is already running before the first poll. Dropping it without awaiting therefore cancels a
/// real in-flight transfer. The cancelled transfer is longer than the 16 word FIFO, so it can
/// not have completed on its own.
///
/// Enable this to check that the chip select is released on cancellation. It should go inactive
/// with the drop and stay there for the idle second which follows. Clearing the FIFOs alone does
/// not do that, so the `Drop` implementation ends the frame with an explicit BMSTOP word.
const TEST_TRANSFER_CANCELLATION: bool = false;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::println!("-- VA416xx Async SPI Example --");

    let dp = pac::Peripherals::take().unwrap();

    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(Hertz::from_raw(EXTCLK_FREQ))
        .freeze()
        .unwrap();
    // Safety: Only called once here.
    va416xx_hal::embassy_time::init(dp.tim15, dp.tim14, &clocks);

    let porta = PinsA::new(dp.porta);
    let mut led = Output::new(porta.pa10, PinState::Low);
    spi::HwCsPin::new(porta.pa4);

    let spi_clk_cfg = SpiClockConfig::from_clks(&clocks, 1.MHz()).unwrap();
    let spi_cfg = spi::SpiConfig::default().clk_cfg(spi_clk_cfg);
    let (sck, miso, mosi) = (porta.pa5, porta.pa6, porta.pa7);
    let spi = spi::Spi::<u8>::new_for_spi2(dp.spi2, (sck, miso, mosi), spi_cfg);

    let mut spi = spi::asynch::SpiAsync::new(spi);
    // Safety: We enable the two interrupt vectors used by the SPI2 driver here, once, before
    // the driver is used.
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::SPI2_TX);
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::SPI2_RX);
    }

    let mut ticker = Ticker::every(Duration::from_secs(1));
    let buf: [u8; 4] = [0xAA; 4];
    let cancel_buf: [u8; 32] = [0x55; 32];
    loop {
        spi.write(&buf).await.expect("spi transfer failed");
        defmt::info!("async SPI transfer done");

        if TEST_TRANSFER_CANCELLATION {
            drop(spi.write(&cancel_buf));
            defmt::info!("dropped in-flight SPI write, probe CS until the next transfer");
        }

        ticker.next().await;
        led.toggle();
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn SPI2_TX() {
    spi::asynch::on_interrupt(Bank::Spi2);
}

#[interrupt]
#[allow(non_snake_case)]
fn SPI2_RX() {
    spi::asynch::on_interrupt(Bank::Spi2);
}
