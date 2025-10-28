// Code to test the watchdog timer.
#![no_main]
#![no_std]
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;
use va416xx_hal::clock::ClockConfigurator;

use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::entry;
use simple_examples::peb1;
use va416xx_hal::irq_router::enable_and_init_irq_router;
use va416xx_hal::pac::{self, interrupt};
use va416xx_hal::wdt::Wdt;

static WDT_INTRPT_COUNT: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum TestMode {
    // Watchdog is fed by main loop, which runs with high period.
    FedByMain,
    // Watchdog is fed by watchdog IRQ.
    FedByIrq,
    AllowReset,
}
const TEST_MODE: TestMode = TestMode::FedByMain;
const WDT_ROLLOVER_MS: u32 = 100;

#[entry]
fn main() -> ! {
    defmt::println!("-- VA416xx WDT example application--");
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze()
        .unwrap();
    enable_and_init_irq_router();
    let mut delay = cortex_m::delay::Delay::new(cp.SYST, clocks.apb0().raw());

    let mut last_interrupt_counter = 0;
    let mut wdt_ctrl = Wdt::start(dp.watch_dog, &clocks, WDT_ROLLOVER_MS);
    wdt_ctrl.enable_reset();
    let log_divisor = 25;
    let mut counter: u32 = 0;
    loop {
        counter = counter.wrapping_add(1);
        if counter % log_divisor == 0 {
            defmt::info!("wdt example main loop alive");
        }
        if TEST_MODE != TestMode::AllowReset {
            wdt_ctrl.feed();
        }
        let interrupt_counter = WDT_INTRPT_COUNT.load(Ordering::Relaxed);
        if interrupt_counter > last_interrupt_counter {
            defmt::info!("interrupt counter has increased to {}", interrupt_counter);
            last_interrupt_counter = interrupt_counter;
        }
        match TEST_MODE {
            TestMode::FedByMain => delay.delay_ms(WDT_ROLLOVER_MS / 5),
            TestMode::FedByIrq => delay.delay_ms(WDT_ROLLOVER_MS),
            _ => (),
        }
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn WATCHDOG() {
    WDT_INTRPT_COUNT.fetch_add(1, Ordering::Relaxed);
    let wdt = unsafe { pac::WatchDog::steal() };
    // Clear interrupt.
    if TEST_MODE != TestMode::AllowReset {
        wdt.wdogintclr().write(|w| unsafe { w.bits(1) });
    }
}
