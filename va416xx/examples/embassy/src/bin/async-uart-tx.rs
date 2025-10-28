//! Asynchronous UART transmission example application.
//!
//! This application receives sends 4 strings with different sizes permanently.
//! It uses PORTG0 as TX pin and PORTG1 as RX pin, which is the UART0 on the PEB1 board.
//!
//! Instructions:
//!
//! 1. Tie a USB to UART converter with RX to PORTG0 and TX to PORTG1.
//! 2. Connect to the serial interface by using an application like Putty or picocom. You can
//!    type something in the terminal and check if the data is echoed back. You can also check the
//!    RTT logs to see received data.
#![no_std]
#![no_main]
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use embassy_example::EXTCLK_FREQ;
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Ticker};
use embedded_io_async::Write;
use va416xx_hal::{
    clock::ClockConfigurator,
    gpio::{Output, PinState},
    pac::{self, interrupt},
    pins::PinsG,
    prelude::*,
    time::Hertz,
    uart::{
        self,
        tx_asynch::{on_interrupt_tx, TxAsync},
        Bank,
    },
};

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

    // Initialize the systick interrupt & obtain the token to prove that we did
    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(Hertz::from_raw(EXTCLK_FREQ))
        .freeze()
        .unwrap();
    // Safety: Only called once here.
    va416xx_embassy::init(dp.tim15, dp.tim14, &clocks);

    let pinsg = PinsG::new(dp.portg);
    let mut led = Output::new(pinsg.pg5, PinState::Low);

    let uarta =
        uart::Uart::new(dp.uart0, pinsg.pg0, pinsg.pg1, &clocks, 115200.Hz().into()).unwrap();
    let (tx, _rx) = uarta.split();
    let mut async_tx = TxAsync::new(tx);
    let mut ticker = Ticker::every(Duration::from_secs(1));
    let mut idx = 0;
    loop {
        defmt::println!("Current time: {}", Instant::now().as_secs());
        led.toggle();
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
fn UART0_TX() {
    on_interrupt_tx(Bank::Uart0);
}
