#![no_std]
#![no_main]
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use embassy_example::EXTCLK_FREQ;
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Ticker};
use va416xx_hal::{
    clock::ClockConfigurator,
    gpio::{Output, PinState},
    pac,
    pins::PinsG,
    time::Hertz,
};

cfg_if::cfg_if! {
    if #[cfg(feature = "custom-irqs")] {
        use va416xx_hal::pac::interrupt;
        va416xx_embassy::embassy_time_driver_irqs!(timekeeper_irq = TIM12, alarm_irq = TIM11);
    }
}

// main is itself an async function.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::println!("VA416xx Embassy Demo");

    let dp = pac::Peripherals::take().unwrap();

    // Initialize the systick interrupt & obtain the token to prove that we did
    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(Hertz::from_raw(EXTCLK_FREQ))
        .freeze()
        .unwrap();
    // Safety: Only called once here.
    cfg_if::cfg_if! {
        if #[cfg(not(feature = "custom-irqs"))] {
            va416xx_embassy::init(
                dp.tim15,
                dp.tim14,
                &clocks
            );
        } else {
            va416xx_embassy::init(
                dp.tim12,
                dp.tim11,
                &clocks
            );
        }
    }
    let pinsg = PinsG::new(dp.portg);
    let mut led = Output::new(pinsg.pg5, PinState::Low);
    let mut ticker = Ticker::every(Duration::from_secs(1));
    loop {
        ticker.next().await;
        defmt::info!("Current time: {}", Instant::now().as_secs());
        led.toggle();
    }
}
