//! Empty RTIC project template
#![no_main]
#![no_std]

#[rtic::app(device = pac)]
mod app {
    // Import panic provider.
    use panic_probe as _;
    // Import global logger.
    use defmt_rtt as _;
    use va108xx_hal::pac;

    #[local]
    struct Local {}

    #[shared]
    struct Shared {}

    #[init]
    fn init(_ctx: init::Context) -> (Shared, Local) {
        defmt::println!("-- Vorago RTIC template --");
        (Shared {}, Local {})
    }

    // `shared` cannot be accessed from this context
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        #[allow(clippy::empty_loop)]
        loop {}
    }
}
