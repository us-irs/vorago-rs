//! # API for clock related functionality
//!
//! This also includes functionality to enable the peripheral clocks
pub use vorago_shared_hal::gpio::FilterClockSelect;
pub use vorago_shared_hal::sysconfig::{disable_peripheral_clock, enable_peripheral_clock};

pub fn set_clk_div_register(syscfg: &mut va108xx::Sysconfig, clk_sel: FilterClockSelect, div: u32) {
    match clk_sel {
        FilterClockSelect::SysClk => (),
        FilterClockSelect::Clk1 => {
            syscfg.ioconfig_clkdiv1().write(|w| unsafe { w.bits(div) });
        }
        FilterClockSelect::Clk2 => {
            syscfg.ioconfig_clkdiv2().write(|w| unsafe { w.bits(div) });
        }
        FilterClockSelect::Clk3 => {
            syscfg.ioconfig_clkdiv3().write(|w| unsafe { w.bits(div) });
        }
        FilterClockSelect::Clk4 => {
            syscfg.ioconfig_clkdiv4().write(|w| unsafe { w.bits(div) });
        }
        FilterClockSelect::Clk5 => {
            syscfg.ioconfig_clkdiv5().write(|w| unsafe { w.bits(div) });
        }
        FilterClockSelect::Clk6 => {
            syscfg.ioconfig_clkdiv6().write(|w| unsafe { w.bits(div) });
        }
        FilterClockSelect::Clk7 => {
            syscfg.ioconfig_clkdiv7().write(|w| unsafe { w.bits(div) });
        }
    }
}
