//! RTIC minimal blinky
#![no_main]
#![no_std]

use va416xx_hal::time::Hertz;

const EXTCLK_FREQ: Hertz = Hertz::from_raw(40_000_000);

#[rtic::app(device = pac, dispatchers = [U1, U2, U3])]
mod app {
    use super::*;
    // Import panic provider.
    use panic_probe as _;
    // Import logger.
    use cortex_m::asm;
    use defmt_rtt as _;
    use rtic_monotonics::systick::prelude::*;
    use rtic_monotonics::Monotonic;
    use va416xx_hal::{
        clock::ClockConfigurator,
        gpio::{Output, PinState},
        pac,
        pins::PinsG,
    };

    #[local]
    struct Local {
        led: Output,
    }

    #[shared]
    struct Shared {}

    rtic_monotonics::systick_monotonic!(Mono, 1_000);

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::println!("-- Vorago RTIC example application --");
        // Use the external clock connected to XTAL_N.
        let clocks = ClockConfigurator::new(cx.device.clkgen)
            .xtal_n_clk_with_src_freq(EXTCLK_FREQ)
            .freeze()
            .unwrap();
        Mono::start(cx.core.SYST, clocks.sysclk().raw());
        let pinsg = PinsG::new(cx.device.portg);
        let led = Output::new(pinsg.pg5, PinState::Low);
        blinky::spawn().ok();
        (Shared {}, Local { led })
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
        local=[led],
    )]
    async fn blinky(cx: blinky::Context) {
        loop {
            cx.local.led.toggle();
            Mono::delay(200.millis()).await;
        }
    }
}
