//! Asynchronous UART reception example application.
//!
//! This application receives data on a UART permanently and echoes it back.
//! It uses PORTG0 as TX pin and PORTG1 as RX pin, which is the UART0 on the PEB1 board.
//!
//! [uart::RxWithInterrupt::on_interrupt] drains the RX FIFO into a small stack buffer directly in
//! the interrupt handler, and forwards the bytes into an [embassy_sync::pipe::Pipe]. The main
//! loop just awaits that pipe, decoupling reception (which must keep running regardless of
//! whether anyone is currently reading) from how often the loop gets around to reading it. See
//! [uart::asynch] for why there is no dedicated async RX driver instead.
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
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pipe::Pipe};
use embassy_time::Instant;
use embedded_io::Write;
use once_cell::sync::OnceCell;
use va416xx_hal::{
    clock, gpio,
    pac::{self, interrupt},
    pins,
    prelude::*,
    time, uart,
};

static PIPE_UART_A: Pipe<CriticalSectionRawMutex, 256> = Pipe::new();

/// Token identifying the UART A peripheral, set once at construction and read by the interrupt
/// handler, which does not have access to an owned [uart::RxWithInterrupt] instance.
static UART_A_TOKEN: OnceCell<uart::Bank> = OnceCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::println!("-- VA108xx Async UART RX Demo --");

    let dp = pac::Peripherals::take().unwrap();

    // Initialize the systick interrupt & obtain the token to prove that we did
    // Use the external clock connected to XTAL_N.
    let clocks = clock::ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(time::Hertz::from_raw(EXTCLK_FREQ))
        .freeze()
        .unwrap();
    va416xx_hal::embassy_time::init(dp.tim15, dp.tim14, &clocks);

    let portg = pins::PinsG::new(dp.portg);
    let mut led = gpio::Output::new(portg.pg5, gpio::PinState::Low);

    let clock_config = uart::ClockConfig::calculate_with_clocks(
        uart::Bank::Uart0,
        &clocks,
        115200.Hz(),
        uart::BaudMode::_16,
    );
    let uart_config = uart::Config::new_with_clock_config(clock_config);
    let uarta = uart::Uart::new_for_uart0(dp.uart0, portg.pg0, portg.pg1, uart_config);

    let (mut tx_uart_a, rx_uart_a) = uarta.split();

    let mut rx_uart_a = rx_uart_a.into_rx_with_interrupt();
    rx_uart_a.start();
    UART_A_TOKEN.set(rx_uart_a.bank_id()).unwrap();

    let mut buf = [0u8; 256];
    loop {
        defmt::info!("Current time UART A: {}", Instant::now().as_secs());
        led.toggle();
        let bytes_read = PIPE_UART_A.read(&mut buf).await;
        let read_str = core::str::from_utf8(&buf[..bytes_read]).unwrap();
        defmt::info!(
            "Read {} bytes asynchronously on UART A: {:?}",
            bytes_read,
            read_str
        );
        tx_uart_a.write_all(read_str.as_bytes()).unwrap();
    }
}

/// `Pipe` has no `try_write_all` yet, and `try_write` can write fewer bytes than given (e.g. when
/// the write wraps around the ring buffer). Retry until everything is written or the pipe
/// reports it is full.
fn pipe_try_write_all(pipe: &Pipe<CriticalSectionRawMutex, 256>, mut buf: &[u8]) {
    while !buf.is_empty() {
        match pipe.try_write(buf) {
            Ok(n) if n > 0 => buf = &buf[n..],
            _ => break,
        }
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn UART0_RX() {
    let mut buf = [0u8; 16];
    let result = uart::RxWithInterrupt::on_interrupt(*UART_A_TOKEN.get().unwrap(), &mut buf);
    if result.bytes_read > 0 {
        pipe_try_write_all(&PIPE_UART_A, &buf[..result.bytes_read]);
    }
    if let Some(errors) = result.errors {
        defmt::warn!("UART A errors: {:?}", errors);
    }
}
