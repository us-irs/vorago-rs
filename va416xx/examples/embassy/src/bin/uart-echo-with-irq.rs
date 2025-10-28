//! This is an example of using the UART HAL abstraction with the IRQ support and embassy.
//!
//! It uses the UART0 for communication with another MCU or a host computer (recommended).
//! You can connect a USB-to-Serial converter to the UART0 pins and then use a serial terminal
//! application like picocom to send data to the microcontroller, which should be echoed
//! back to the sender.
//!
//! This application uses the interrupt support of the VA416xx to read the data arriving
//! on the UART without requiring polling.
#![no_std]
#![no_main]
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use core::cell::RefCell;

use embassy_example::EXTCLK_FREQ;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Duration, Ticker};
use embedded_io::Write;
use ringbuf::{
    traits::{Consumer, Observer, Producer},
    StaticRb,
};
use va416xx_hal::{
    clock::ClockConfigurator,
    gpio::{Output, PinState},
    pac::{self, interrupt},
    pins::PinsG,
    time::Hertz,
    uart,
};

pub type SharedUart = Mutex<CriticalSectionRawMutex, RefCell<Option<uart::RxWithInterrupt>>>;
static RX: SharedUart = Mutex::new(RefCell::new(None));

const BAUDRATE: u32 = 115200;

// Ring buffer size.
const RING_BUF_SIZE: usize = 2048;

pub type SharedRingBuf =
    Mutex<CriticalSectionRawMutex, RefCell<Option<StaticRb<u8, RING_BUF_SIZE>>>>;
// Ring buffers to handling variable sized telemetry
static RINGBUF: SharedRingBuf = Mutex::new(RefCell::new(None));

// See https://embassy.dev/book/#_sharing_using_a_mutex for background information about sharing
// a peripheral with embassy.
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
    // Safety: Only called once here.
    va416xx_embassy::init(dp.tim15, dp.tim14, &clocks);

    let portg = PinsG::new(dp.portg);

    let uart0 = uart::Uart::new(
        dp.uart0,
        portg.pg0,
        portg.pg1,
        &clocks,
        Hertz::from_raw(BAUDRATE).into(),
    )
    .unwrap();
    let (mut tx, rx) = uart0.split();
    let mut rx = rx.into_rx_with_irq();
    rx.start();
    RX.lock(|static_rx| {
        static_rx.borrow_mut().replace(rx);
    });
    RINGBUF.lock(|static_rb| {
        static_rb.borrow_mut().replace(StaticRb::default());
    });

    let led = Output::new(portg.pg5, PinState::Low);
    let mut ticker = Ticker::every(Duration::from_millis(50));
    let mut processing_buf: [u8; RING_BUF_SIZE] = [0; RING_BUF_SIZE];
    let mut read_bytes = 0;
    spawner.spawn(blinky(led)).expect("failed to spawn blinky");
    loop {
        RINGBUF.lock(|static_rb| {
            let mut rb_borrow = static_rb.borrow_mut();
            let rb_mut = rb_borrow.as_mut().unwrap();
            read_bytes = rb_mut.occupied_len();
            rb_mut.pop_slice(&mut processing_buf[0..read_bytes]);
        });
        // Simply send back all received data.
        tx.write_all(&processing_buf[0..read_bytes])
            .expect("sending back read data failed");
        ticker.next().await;
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

#[interrupt]
#[allow(non_snake_case)]
fn UART0_RX() {
    let mut buf: [u8; 16] = [0; 16];
    let mut read_len: usize = 0;
    let mut errors = None;
    RX.lock(|static_rx| {
        let mut rx_borrow = static_rx.borrow_mut();
        let rx_mut_ref = rx_borrow.as_mut().unwrap();
        let result = rx_mut_ref.on_interrupt(&mut buf);
        read_len = result.bytes_read;
        if result.errors.is_some() {
            errors = result.errors;
        }
    });
    let mut ringbuf_full = false;
    if read_len > 0 {
        // Send the received buffer to the main thread for processing via a ring buffer.
        RINGBUF.lock(|static_rb| {
            let mut rb_borrow = static_rb.borrow_mut();
            let rb_mut_ref = rb_borrow.as_mut().unwrap();
            if rb_mut_ref.vacant_len() < read_len {
                ringbuf_full = true;
                for _ in rb_mut_ref.pop_iter() {}
            }
            rb_mut_ref.push_slice(&buf[0..read_len]);
        });
    }

    if errors.is_some() {
        defmt::info!("UART error: {:?}", errors);
    }
    if ringbuf_full {
        defmt::info!("ringbuffer is full, deleted oldest data");
    }
}
