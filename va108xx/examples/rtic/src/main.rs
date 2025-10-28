//! RTIC minimal blinky
#![no_main]
#![no_std]

#[rtic::app(device = pac, dispatchers = [OC31, OC30, OC29])]
mod app {
    use cortex_m::asm;
    use rtic_example::SYSCLK_FREQ;
    use rtic_monotonics::systick::prelude::*;
    use rtic_monotonics::Monotonic;
    // Import panic provider.
    use panic_probe as _;
    // Import global logger.
    use defmt_rtt as _;
    use va108xx_hal::{
        gpio::{Output, PinState},
        pac,
        pins::PinsA,
    };

    #[local]
    struct Local {
        led0: Output,
        led1: Output,
        led2: Output,
    }

    #[shared]
    struct Shared {}

    rtic_monotonics::systick_monotonic!(Mono, 1_000);

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::println!("-- Vorago VA108xx RTIC template --");

        Mono::start(cx.core.SYST, SYSCLK_FREQ.raw());

        let porta = PinsA::new(cx.device.porta);
        let led0 = Output::new(porta.pa10, PinState::Low);
        let led1 = Output::new(porta.pa7, PinState::Low);
        let led2 = Output::new(porta.pa6, PinState::Low);
        blinky::spawn().ok();
        (Shared {}, Local { led0, led1, led2 })
    }

    // `shared` cannot be accessed from this context
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            asm::nop();
        }
    }

    #[task(
        priority = 3,
        local=[led0, led1, led2],
    )]
    async fn blinky(cx: blinky::Context) {
        loop {
            defmt::println!("toggling LEDs");
            cx.local.led0.toggle();
            cx.local.led1.toggle();
            cx.local.led2.toggle();
            Mono::delay(1000.millis()).await;
        }
    }
}
