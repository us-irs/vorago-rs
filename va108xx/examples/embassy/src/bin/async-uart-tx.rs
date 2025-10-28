//! Asynchronous UART transmission example application.
//!
//! This application receives sends 4 strings with different sizes permanently using UART A.
//! Ports PA8 and PA9 are used for this.
//!
//! Instructions:
//!
//! 1. Tie a USB to UART converter with RX to PA9 and TX to PA8 for UART A.
//! 2. Connect to the serial interface by using an application like Putty or picocom. You can
//!    can verify the correctness of the sent strings.
#![no_std]
#![no_main]
// This imports the logger and the panic handler.
use embassy_example as _;

use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Ticker};
use embedded_io_async::Write;
use va108xx_hal::{
    gpio::{Output, PinState},
    pac::{self, interrupt},
    pins::PinsA,
    prelude::*,
    uart::{self, on_interrupt_tx, Bank, TxAsync},
    InterruptConfig,
};

const SYSCLK_FREQ: Hertz = Hertz::from_raw(50_000_000);

const STR_LIST: &[&str] = &[
    "Hello World\r\n",
    "Smoll\r\n",
    "A string which is larger than the FIFO size\r\n",
    "A really large string which is significantly larger than the FIFO size\r\n",
];

// main is itself an async function.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::println!("-- VA108xx Async UART TX Demo --");

    let dp = pac::Peripherals::take().unwrap();

    // Safety: Only called once here.
    va108xx_embassy::init(dp.tim23, dp.tim22, SYSCLK_FREQ);

    let porta = PinsA::new(dp.porta);

    let mut led0 = Output::new(porta.pa10, PinState::Low);
    let mut led1 = Output::new(porta.pa7, PinState::Low);
    let mut led2 = Output::new(porta.pa6, PinState::Low);

    let tx = porta.pa9;
    let rx = porta.pa8;

    let uarta = uart::Uart::new_with_interrupt(
        dp.uarta,
        tx,
        rx,
        50.MHz(),
        115200.Hz().into(),
        InterruptConfig::new(pac::Interrupt::OC2, true, true),
    )
    .unwrap();
    let (tx, _rx) = uarta.split();
    let mut async_tx = TxAsync::new(tx);
    let mut ticker = Ticker::every(Duration::from_secs(1));
    let mut idx = 0;
    loop {
        defmt::info!("Current time: {}", Instant::now().as_secs());
        led0.toggle();
        led1.toggle();
        led2.toggle();
        let _written = async_tx
            .write(STR_LIST[idx].as_bytes())
            .await
            .expect("writing failed");
        idx += 1;
        if idx == STR_LIST.len() {
            idx = 0;
        }
        ticker.next().await;
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn OC2() {
    on_interrupt_tx(Bank::Uart0);
}
