//! UART example application. Sends a test string over a UART and then enters
//! echo mode.
//!
//! Instructions:
//!
//! 1. Tie a USB to UART converter with RX to PA9 and TX to PA8.
//! 2. Connect to the serial interface by using an application like Putty or picocom.
//!    You should set a "Hello World" print when the application starts. After that, everything
//!    typed on the console should be printed back by the echo application.
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal_nb::{nb, serial::Read};
use embedded_io::Write as _;
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;
use va108xx_hal::{pac, pins::PinsA, prelude::*, uart};

#[entry]
fn main() -> ! {
    defmt::println!("-- VA108xx UART example application--");

    let dp = pac::Peripherals::take().unwrap();

    let gpioa = PinsA::new(dp.porta);
    let tx = gpioa.pa9;
    let rx = gpioa.pa8;
    let uart =
        uart::Uart::new_without_interrupt(dp.uarta, tx, rx, 50.MHz(), 115200.Hz().into()).unwrap();

    let (mut tx, mut rx) = uart.split();
    writeln!(tx, "Hello World\r").unwrap();
    loop {
        // Echo what is received on the serial link.
        match rx.read() {
            Ok(recv) => {
                nb::block!(embedded_hal_nb::serial::Write::write(&mut tx, recv))
                    .expect("TX send error");
            }
            Err(nb::Error::WouldBlock) => (),
        }
    }
}
