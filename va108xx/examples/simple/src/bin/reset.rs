//! Dummy app which does not do anything.
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use va108xx_hal as _;

#[entry]
fn main() -> ! {
    cortex_m::peripheral::SCB::sys_reset();
}
