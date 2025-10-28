//! Blinky button application for the REB1 board
#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::{
    clock::{set_clk_div_register, FilterClockSelect},
    gpio::{FilterType, InterruptEdge},
    pac::{self, interrupt},
    pins::PinsA,
    timer::InterruptConfig,
};
use vorago_reb1::button::Button;
use vorago_reb1::leds::Leds;

static LEDS: Mutex<RefCell<Option<Leds>>> = Mutex::new(RefCell::new(None));
static BUTTON: Mutex<RefCell<Option<Button>>> = Mutex::new(RefCell::new(None));

#[derive(Debug, PartialEq)]
pub enum PressMode {
    Toggle,
    Keep,
}

// You can change the press mode here
const PRESS_MODE: PressMode = PressMode::Keep;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- Vorago Button IRQ Example --");
    let mut dp = pac::Peripherals::take().unwrap();
    let pinsa = PinsA::new(dp.porta);
    let edge_irq = match PRESS_MODE {
        PressMode::Toggle => InterruptEdge::HighToLow,
        PressMode::Keep => InterruptEdge::BothEdges,
    };

    // Configure an edge interrupt on the button and route it to interrupt vector 15
    let mut button = Button::new(pinsa.pa11);

    if PRESS_MODE == PressMode::Toggle {
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
    // Make both button and LEDs accessible from the IRQ handler as well
    cortex_m::interrupt::free(|cs| {
        LEDS.borrow(cs).replace(Some(leds));
        BUTTON.borrow(cs).replace(Some(button));
    });
    loop {
        cortex_m::asm::nop();
    }
}

#[interrupt]
fn OC15() {
    cortex_m::interrupt::free(|cs| {
        if PRESS_MODE == PressMode::Toggle {
            if let Some(ref mut leds) = LEDS.borrow(cs).borrow_mut().as_deref_mut() {
                leds[0].toggle();
            }
        } else if let (Some(ref mut leds), Some(ref mut button)) = (
            LEDS.borrow(cs).borrow_mut().as_deref_mut(),
            BUTTON.borrow(cs).borrow_mut().as_mut(),
        ) {
            if button.released() {
                leds[0].off();
            } else {
                leds[0].on();
            }
        }
    });
}
