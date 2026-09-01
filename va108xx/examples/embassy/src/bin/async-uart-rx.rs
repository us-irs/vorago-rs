//! Asynchronous UART reception example application.
//!
//! This application receives data on two UARTs permanently. UART A is received on ports PA8 and
//! PA9. UART B is received on ports PA2 and PA3.
//!
//! Both UARTs use [uart::RxWithInterrupt::on_interrupt] to drain the RX FIFO into a small stack
//! buffer directly in the interrupt handler, and forward the bytes into an
//! [embassy_sync::pipe::Pipe]. The async tasks below just await that pipe, decoupling reception
//! (which must keep running regardless of whether anyone is currently reading) from how often the
//! echo tasks get around to reading it. See [uart::asynch] for why there is no dedicated async RX
//! driver instead.
//!
//! Instructions:
//!
//! 1. Tie a USB to UART converter with RX to PA9 and TX to PA8 for UART A.
//!    Tie a USB to UART converter with RX to PA3 and TX to PA2 for UART B.
//! 2. Connect to the serial interface by using an application like Putty or picocom. You can
//!    type something in the terminal and check if the data is echoed back. You can also check the
//!    RTT logs to see received data.
#![no_std]
#![no_main]
// This imports the logger and the panic handler.
use embassy_example as _;

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pipe::Pipe};
use embassy_time::Instant;
use embedded_io::Write;
use once_cell::sync::OnceCell;
use va108xx_hal::{
    gpio,
    pac::{self, interrupt},
    pins,
    prelude::*,
    uart, InterruptConfig,
};

const SYSCLK_FREQ: Hertz = Hertz::from_raw(50_000_000);

static PIPE_UART_A: Pipe<CriticalSectionRawMutex, 256> = Pipe::new();
static PIPE_UART_B: Pipe<CriticalSectionRawMutex, 256> = Pipe::new();

/// Tokens identifying the UART A/B peripherals, set once at construction and read by the
/// interrupt handlers, which do not have access to an owned [uart::RxWithInterrupt] instance.
static UART_A_TOKEN: OnceCell<uart::Bank> = OnceCell::new();
static UART_B_TOKEN: OnceCell<uart::Bank> = OnceCell::new();

// main is itself an async function.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::println!("-- VA108xx Async UART RX Demo --");

    let dp = pac::Peripherals::take().unwrap();

    // Safety: Only called once here.
    va108xx_hal::embassy_time::init(dp.tim23, dp.tim22, SYSCLK_FREQ);

    let porta = pins::PinsA::new(dp.porta);
    let mut led0 = gpio::Output::new(porta.pa10, gpio::PinState::Low);
    let mut led1 = gpio::Output::new(porta.pa7, gpio::PinState::Low);
    let mut led2 = gpio::Output::new(porta.pa6, gpio::PinState::Low);

    let tx_uart_a = porta.pa9;
    let rx_uart_a = porta.pa8;

    let clock_config = uart::ClockConfig::calculate(50.MHz(), 115200.Hz(), uart::BaudMode::_16);
    let uart_config = uart::Config::new_with_clock_config(clock_config);
    let uarta = uart::Uart::new_with_interrupt_uart0(
        dp.uarta,
        tx_uart_a,
        rx_uart_a,
        uart_config,
        InterruptConfig::new(pac::Interrupt::OC2, true, true),
    );

    let tx_uart_b = porta.pa3;
    let rx_uart_b = porta.pa2;

    let uartb = uart::Uart::new_with_interrupt_uart1(
        dp.uartb,
        tx_uart_b,
        rx_uart_b,
        uart_config,
        InterruptConfig::new(pac::Interrupt::OC3, true, true),
    );
    let (mut tx_uart_a, rx_uart_a) = uarta.split();
    let (tx_uart_b, rx_uart_b) = uartb.split();

    let mut rx_uart_a = rx_uart_a.into_rx_with_interrupt();
    rx_uart_a.start();
    UART_A_TOKEN.set(rx_uart_a.bank_id()).unwrap();

    let mut rx_uart_b = rx_uart_b.into_rx_with_interrupt();
    rx_uart_b.start();
    UART_B_TOKEN.set(rx_uart_b.bank_id()).unwrap();
    spawner.spawn(uart_b_echo_task(tx_uart_b).unwrap());

    let mut buf = [0u8; 256];
    loop {
        defmt::info!("Current time UART A: {}", Instant::now().as_secs());
        led0.toggle();
        led1.toggle();
        led2.toggle();
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

#[embassy_executor::task]
async fn uart_b_echo_task(mut tx: uart::Tx) {
    let mut buf = [0u8; 256];
    loop {
        let bytes_read = PIPE_UART_B.read(&mut buf).await;
        let read_str = core::str::from_utf8(&buf[..bytes_read]).unwrap();
        defmt::info!(
            "Read {} bytes asynchronously on UART B: {:?}",
            bytes_read,
            read_str
        );
        tx.write_all(read_str.as_bytes()).unwrap();
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
fn OC2() {
    let mut buf = [0u8; 16];
    let result = uart::RxWithInterrupt::on_interrupt(*UART_A_TOKEN.get().unwrap(), &mut buf);
    if result.bytes_read > 0 {
        pipe_try_write_all(&PIPE_UART_A, &buf[..result.bytes_read]);
    }
    if let Some(errors) = result.errors {
        defmt::warn!("UART A errors: {:?}", errors);
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn OC3() {
    let mut buf = [0u8; 16];
    let result = uart::RxWithInterrupt::on_interrupt(*UART_B_TOKEN.get().unwrap(), &mut buf);
    if result.bytes_read > 0 {
        pipe_try_write_all(&PIPE_UART_B, &buf[..result.bytes_read]);
    }
    if let Some(errors) = result.errors {
        defmt::warn!("UART B errors: {:?}", errors);
    }
}
