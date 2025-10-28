//! Blinky button application for the REB1 board using RTIC
#![no_main]
#![no_std]

#[rtic::app(device = pac)]
mod app {
    use rtic_example::SYSCLK_FREQ;
    // Import panic provider.
    use panic_probe as _;
    // Import global logger.
    use defmt_rtt as _;
    use va108xx_hal::{
        clock::{set_clk_div_register, FilterClockSelect},
        gpio::{FilterType, InterruptEdge},
        pac,
        pins::PinsA,
        timer::InterruptConfig,
    };
    use vorago_reb1::button::Button;
    use vorago_reb1::leds::Leds;

    rtic_monotonics::systick_monotonic!(Mono, 1_000);

    #[derive(Debug, PartialEq, defmt::Format)]
    pub enum PressMode {
        Toggle,
        Keep,
    }

    // You can change the press mode here
    const DEFAULT_MODE: PressMode = PressMode::Toggle;

    #[local]
    struct Local {
        leds: Leds,
        button: Button,
        mode: PressMode,
    }

    #[shared]
    struct Shared {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::println!("-- Vorago Button IRQ Example --");
        Mono::start(cx.core.SYST, SYSCLK_FREQ.raw());

        let mode = DEFAULT_MODE;
        defmt::info!("Using {:?} mode", mode);

        let mut dp = cx.device;
        let pinsa = PinsA::new(dp.porta);
        let edge_irq = match mode {
            PressMode::Toggle => InterruptEdge::HighToLow,
            PressMode::Keep => InterruptEdge::BothEdges,
        };

        // Configure an edge interrupt on the button and route it to interrupt vector 15
        let mut button = Button::new(pinsa.pa11);

        if mode == PressMode::Toggle {
            // This filter debounces the switch for edge based interrupts
            button.configure_filter_type(FilterType::FilterFourCycles, FilterClockSelect::Clk1);
            set_clk_div_register(&mut dp.sysconfig, FilterClockSelect::Clk1, 50_000);
        }
        button.configure_and_enable_edge_interrupt(
            edge_irq,
            InterruptConfig::new(pac::interrupt::OC15, true, true),
        );
        let mut leds = Leds::new(pinsa.pa10, pinsa.pa7, pinsa.pa6);
        for led in leds.iter_mut() {
            led.off();
        }
        (Shared {}, Local { leds, button, mode })
    }

    // `shared` cannot be accessed from this context
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = OC15, local=[button, leds, mode])]
    fn button_task(cx: button_task::Context) {
        let leds = cx.local.leds;
        let button = cx.local.button;
        let mode = cx.local.mode;
        if *mode == PressMode::Toggle {
            leds[0].toggle();
        } else if button.released() {
            leds[0].off();
        } else {
            leds[0].on();
        }
    }
}
