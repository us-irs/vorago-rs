//! This is an example of using the UART HAL abstraction with the IRQ support and embassy.
//!
//! It uses the UART0 for communication with another MCU or a host computer (recommended).
//! You can connect a USB-to-Serial converter to the UART0 pins and then use a serial terminal
//! application like picocom to send data to the microcontroller, which should be echoed
//! back to the sender.
//!
//! This application uses the interrupt support of the VA416xx to read the data arriving
//! on the UART without requiring polling. [uart::RxWithInterrupt::on_interrupt] drains the RX
//! FIFO into a small stack buffer directly in the interrupt handler, and forwards the bytes into
//! an [embassy_sync::pipe::Pipe]. The main loop just awaits that pipe. See [uart::asynch] for why
//! there is no dedicated async RX driver instead.
#![no_std]
#![no_main]
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use embassy_example::EXTCLK_FREQ;
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pipe::Pipe};
use embassy_time::{Duration, Ticker};
use embedded_io::Write;
use once_cell::sync::OnceCell;
use va416xx_hal::{
    clock::ClockConfigurator,
    gpio::{Output, PinState},
    pac::{self, interrupt},
    pins::PinsG,
    time::Hertz,
    uart,
};

const BAUDRATE: u32 = 115200;

const PIPE_SIZE: usize = 2048;
static PIPE: Pipe<CriticalSectionRawMutex, PIPE_SIZE> = Pipe::new();

/// Token identifying the UART0 peripheral, set once at construction and read by the interrupt
/// handler, which does not have access to an owned [uart::RxWithInterrupt] instance.
static UART0_TOKEN: OnceCell<uart::Bank> = OnceCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::println!("VA416xx UART-Embassy Example");

    let dp = pac::Peripherals::take().unwrap();

    // Initialize the systick interrupt & obtain the token to prove that we did
    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(Hertz::from_raw(EXTCLK_FREQ))
        .freeze()
        .unwrap();
    va416xx_hal::embassy_time::init(dp.tim15, dp.tim14, &clocks);

    let portg = PinsG::new(dp.portg);

    let clock_config = uart::ClockConfig::calculate_with_clocks(
        uart::Bank::Uart0,
        &clocks,
        Hertz::from_raw(BAUDRATE),
        uart::BaudMode::_16,
    );
    let uart_config = uart::Config::new_with_clock_config(clock_config);
    let uart0 = uart::Uart::new_for_uart0(dp.uart0, portg.pg0, portg.pg1, uart_config);
    let (mut tx, rx) = uart0.split();
    let mut rx = rx.into_rx_with_interrupt();
    rx.start();
    UART0_TOKEN.set(rx.bank_id()).unwrap();

    let led = Output::new(portg.pg5, PinState::Low);
    spawner.spawn(blinky(led).expect("failed to spawn blinky"));

    let mut buf = [0u8; PIPE_SIZE];
    loop {
        let bytes_read = PIPE.read(&mut buf).await;
        // Simply send back all received data.
        tx.write_all(&buf[..bytes_read])
            .expect("sending back read data failed");
    }
}

#[embassy_executor::task]
async fn blinky(mut led: Output) {
    let mut ticker = Ticker::every(Duration::from_millis(500));
    loop {
        led.toggle();
        ticker.next().await;
    }
}

/// `Pipe` has no `try_write_all` yet, and `try_write` can write fewer bytes than given (e.g. when
/// the write wraps around the ring buffer). Retry until everything is written or the pipe
/// reports it is full.
fn pipe_try_write_all(pipe: &Pipe<CriticalSectionRawMutex, PIPE_SIZE>, mut buf: &[u8]) {
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
    let result = uart::RxWithInterrupt::on_interrupt(*UART0_TOKEN.get().unwrap(), &mut buf);
    if result.bytes_read > 0 {
        pipe_try_write_all(&PIPE, &buf[..result.bytes_read]);
    }
    if let Some(errors) = result.errors {
        defmt::info!("UART error: {:?}", errors);
    }
}
