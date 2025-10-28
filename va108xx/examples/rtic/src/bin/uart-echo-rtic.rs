//! More complex UART application on UART PA8 (TX) and PA9 (RX).
//!
//! Uses the IRQ capabilities of the VA10820 peripheral and the RTIC framework to poll the UART in
//! a non-blocking way. All received data will be sent back to the sender.
#![no_main]
#![no_std]

use ringbuf::StaticRb;

// Larger buffer for TC to be able to hold the possibly large memory write packets.
const RX_RING_BUF_SIZE: usize = 1024;

#[rtic::app(device = pac, dispatchers = [OC4])]
mod app {
    use super::*;
    use embedded_io::Write;
    use ringbuf::traits::{Consumer, Observer, Producer};
    use rtic_example::SYSCLK_FREQ;
    // Import panic provider.
    use panic_probe as _;
    // Import global logger.
    use defmt_rtt as _;
    use rtic_monotonics::Monotonic;
    use va108xx_hal::{
        pac,
        pins::PinsA,
        prelude::*,
        uart::{self, RxWithInterrupt, Tx},
        InterruptConfig,
    };

    #[local]
    struct Local {
        rx: RxWithInterrupt,
        tx: Tx,
    }

    #[shared]
    struct Shared {
        rb: StaticRb<u8, RX_RING_BUF_SIZE>,
    }

    rtic_monotonics::systick_monotonic!(Mono, 1_000);

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::println!("-- VA108xx UART Echo with IRQ example application--");

        Mono::start(cx.core.SYST, SYSCLK_FREQ.raw());

        let dp = cx.device;
        let gpioa = PinsA::new(dp.porta);
        let tx = gpioa.pa9;
        let rx = gpioa.pa8;

        let irq_uart = uart::Uart::new_with_interrupt(
            dp.uarta,
            tx,
            rx,
            SYSCLK_FREQ,
            115200.Hz().into(),
            InterruptConfig::new(pac::Interrupt::OC3, true, true),
        )
        .unwrap();
        let (tx, rx) = irq_uart.split();
        let mut rx = rx.into_rx_with_irq();

        rx.start();

        echo_handler::spawn().unwrap();
        (
            Shared {
                rb: StaticRb::default(),
            },
            Local { rx, tx },
        )
    }

    // `shared` cannot be accessed from this context
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(
        binds = OC3,
        shared = [rb],
        local = [
            rx,
        ],
    )]
    fn reception_task(mut cx: reception_task::Context) {
        let mut buf: [u8; 16] = [0; 16];
        let mut ringbuf_full = false;
        let result = cx.local.rx.on_interrupt(&mut buf);
        if result.bytes_read > 0 && result.errors.is_none() {
            cx.shared.rb.lock(|rb| {
                if rb.vacant_len() < result.bytes_read {
                    ringbuf_full = true;
                } else {
                    rb.push_slice(&buf[0..result.bytes_read]);
                }
            });
        }
        if ringbuf_full {
            // Could also drop oldest data, but that would require the consumer to be shared.
            defmt::println!("buffer full, data was dropped");
        }
    }

    #[task(shared = [rb], local = [
        buf: [u8; RX_RING_BUF_SIZE] = [0; RX_RING_BUF_SIZE],

        tx
    ], priority=1)]
    async fn echo_handler(mut cx: echo_handler::Context) {
        loop {
            cx.shared.rb.lock(|rb| {
                let bytes_to_read = rb.occupied_len();
                if bytes_to_read > 0 {
                    let actual_read_bytes = rb.pop_slice(&mut cx.local.buf[0..bytes_to_read]);
                    cx.local
                        .tx
                        .write_all(&cx.local.buf[0..actual_read_bytes])
                        .expect("Failed to write to TX");
                }
            });
            Mono::delay(50.millis()).await;
        }
    }
}
