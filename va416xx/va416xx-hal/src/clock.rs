//! API for using the [crate::pac::Clkgen] peripheral.
//!
//! It also includes functionality to enable the peripheral clocks.
//! Calling [ClockConfigurator::new] returns a builder structure which allows
//! setting up the clock.
//!
//! Calling [ClockConfigurator::freeze] returns the frozen clock configuration inside the [Clocks]
//! structure. This structure can also be used to configure other structures provided by this HAL.
//!
//! # Examples
//!
//! - [UART example on the PEB1 board](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/uart.rs)
#[cfg(not(feature = "va41628"))]
use crate::adc::ADC_MAX_CLK;
use crate::pac;

use crate::time::Hertz;
pub use vorago_shared_hal::clock::{Clocks, HBO_FREQ};
use vorago_shared_hal::{enable_peripheral_clock, PeripheralSelect};

pub const XTAL_OSC_TSTART_MS: u32 = 15;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterClockSelect {
    SysClk = 0,
    Clk1 = 1,
    Clk2 = 2,
    Clk3 = 3,
    Clk4 = 4,
    Clk5 = 5,
    Clk6 = 6,
    Clk7 = 7,
}

/// Refer to chapter 8 (p.57) of the programmers guide for detailed information.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockSelect {
    // Internal Heart-Beat Osciallator. Not tightly controlled (+/-20 %). Not recommended as the regular clock!
    Hbo = 0b00,
    // External clock signal on XTAL_N line, 1-100 MHz
    XtalN = 0b01,
    // Internal Phase-Locked Loop.
    Pll = 0b10,
    // Crystal oscillator amplified, 4-10 MHz.
    XtalOsc = 0b11,
}

/// This selects the input clock to the the CLKGEN peripheral in addition to the HBO clock.
///
/// This can either be a clock connected directly on the XTAL_N line or a chrystal on the XTAL_P
/// line which goes through an oscillator amplifier.
///
/// Refer to chapter 8 (p.57) of the programmers guide for detailed information.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReferenceClockSelect {
    #[default]
    None = 0b00,
    XtalOsc = 0b01,
    XtalN = 0b10,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockDivisorSelect {
    #[default]
    Div1 = 0b00,
    Div2 = 0b01,
    Div4 = 0b10,
    Div8 = 0b11,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcClockDivisorSelect {
    Div8 = 0b00,
    Div4 = 0b01,
    Div2 = 0b10,
    Div1 = 0b11,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PllConfig {
    /// Reference clock divider.
    pub clkr: u8,
    /// Clock divider on feedback path
    pub clkf: u8,
    // Output clock divider.
    pub clkod: u8,
    /// Bandwidth adjustment
    pub bwadj: u8,
}

#[inline]
pub const fn clock_after_division(clk: Hertz, div_sel: ClockDivisorSelect) -> Hertz {
    match div_sel {
        ClockDivisorSelect::Div1 => clk,
        ClockDivisorSelect::Div2 => Hertz::from_raw(clk.raw() / 2),
        ClockDivisorSelect::Div4 => Hertz::from_raw(clk.raw() / 4),
        ClockDivisorSelect::Div8 => Hertz::from_raw(clk.raw() / 8),
    }
}

/// Wait for 500 reference clock cycles like specified in the datasheet.
pub fn pll_setup_delay() {
    for _ in 0..500 {
        cortex_m::asm::nop()
    }
}

pub trait ClkgenExt {
    fn constrain(self) -> ClockConfigurator;
}

impl ClkgenExt for pac::Clkgen {
    fn constrain(self) -> ClockConfigurator {
        ClockConfigurator {
            source_clk: None,
            ref_clk_sel: ReferenceClockSelect::None,
            clksel_sys: ClockSelect::Hbo,
            clk_div_sel: ClockDivisorSelect::Div1,
            clk_lost_detection: false,
            pll_lock_lost_detection: false,
            pll_cfg: None,
            clkgen: self,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClockSourceFrequencyNotSet;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockConfigError {
    ClkSourceFreqNotSet,
    PllConfigNotSet,
    PllInitError,
    InconsistentCfg,
}

pub struct ClockConfigurator {
    ref_clk_sel: ReferenceClockSelect,
    clksel_sys: ClockSelect,
    clk_div_sel: ClockDivisorSelect,
    /// The source clock frequency which is either an external clock connected to XTAL_N, or a
    /// crystal connected to the XTAL_OSC input.
    source_clk: Option<Hertz>,
    pll_cfg: Option<PllConfig>,
    clk_lost_detection: bool,
    /// Feature only works on revision B of the board.
    #[cfg(feature = "revb")]
    pll_lock_lost_detection: bool,
    clkgen: pac::Clkgen,
}

/// Delays a given amount of milliseconds.
///
/// Taken from the HAL implementation. This implementation is probably not precise and it
/// also blocks!
pub fn hbo_clock_delay_ms(ms: u32) {
    let wdt = unsafe { pac::WatchDog::steal() };
    for _ in 0..ms {
        for _ in 0..10_000 {
            cortex_m::asm::nop();
        }
        wdt.wdogintclr().write(|w| unsafe { w.bits(1) });
    }
}

impl ClockConfigurator {
    /// Create a new clock configuration instance.
    pub fn new(clkgen: pac::Clkgen) -> Self {
        ClockConfigurator {
            source_clk: None,
            ref_clk_sel: ReferenceClockSelect::None,
            clksel_sys: ClockSelect::Hbo,
            clk_div_sel: ClockDivisorSelect::Div1,
            clk_lost_detection: false,
            pll_lock_lost_detection: false,
            pll_cfg: None,
            clkgen,
        }
    }

    /// Steals a new [ClockConfigurator] instance.
    ///
    /// # Safety
    ///
    /// Circumvents HAL ownership rules.
    pub unsafe fn steal() -> Self {
        Self::new(unsafe { pac::Clkgen::steal() })
    }

    #[inline]
    pub fn source_clk(mut self, src_clk: Hertz) -> Self {
        self.source_clk = Some(src_clk);
        self
    }

    /// This function can be used to utilize the XTAL_N clock input directly without the
    /// oscillator.
    ///
    /// It sets the internal configuration to [ClkselSys::XtalN] and [RefClkSel::XtalN].
    #[inline]
    pub fn xtal_n_clk(mut self) -> Self {
        self.clksel_sys = ClockSelect::XtalN;
        self.ref_clk_sel = ReferenceClockSelect::XtalN;
        self
    }

    #[inline]
    pub fn xtal_n_clk_with_src_freq(mut self, src_clk: Hertz) -> Self {
        self = self.xtal_n_clk();
        self.source_clk(src_clk)
    }

    #[inline]
    pub fn clksel_sys(mut self, clksel_sys: ClockSelect) -> Self {
        self.clksel_sys = clksel_sys;
        self
    }

    #[inline]
    pub fn pll_cfg(mut self, pll_cfg: PllConfig) -> Self {
        self.pll_cfg = Some(pll_cfg);
        self
    }

    #[inline]
    pub fn ref_clk_sel(mut self, ref_clk_sel: ReferenceClockSelect) -> Self {
        self.ref_clk_sel = ref_clk_sel;
        self
    }

    /// Configures all clocks and return a clock configuration structure containing the final
    /// frozen clocks.
    ///
    /// Internal implementation details: This implementation is based on the HAL implementation
    /// which performs a lot of delays. I do not know if all of those are necessary, but
    /// I am going to be conservative here and assume that the vendor has tested though and
    /// might have had a reason for those, so I am going to keep them. Chances are, this
    /// process only has to be performed once, and it does not matter if it takes a few
    /// microseconds or milliseconds longer.
    pub fn freeze(self) -> Result<Clocks, ClockConfigError> {
        // Sanitize configuration.
        if self.source_clk.is_none() {
            return Err(ClockConfigError::ClkSourceFreqNotSet);
        }
        if self.clksel_sys == ClockSelect::XtalOsc
            && self.ref_clk_sel != ReferenceClockSelect::XtalOsc
        {
            return Err(ClockConfigError::InconsistentCfg);
        }
        if self.clksel_sys == ClockSelect::XtalN && self.ref_clk_sel != ReferenceClockSelect::XtalN
        {
            return Err(ClockConfigError::InconsistentCfg);
        }
        if self.clksel_sys == ClockSelect::Pll && self.pll_cfg.is_none() {
            return Err(ClockConfigError::PllConfigNotSet);
        }

        enable_peripheral_clock(PeripheralSelect::Clkgen);
        let mut final_sysclk = self.source_clk.unwrap();
        // The HAL forces back the HBO clock here with a delay.. Even though this is
        // not stricly necessary when coming from a fresh start, it could be still become relevant
        // later if the clock lost detection mechanism require a re-configuration of the clocks.
        // Therefore, we do it here as well.
        self.clkgen
            .ctrl0()
            .modify(|_, w| unsafe { w.clksel_sys().bits(ClockSelect::Hbo as u8) });
        pll_setup_delay();
        self.clkgen
            .ctrl0()
            .modify(|_, w| unsafe { w.clk_div_sel().bits(ClockDivisorSelect::Div1 as u8) });

        // Set up oscillator and PLL input clock.
        self.clkgen
            .ctrl0()
            .modify(|_, w| unsafe { w.ref_clk_sel().bits(self.ref_clk_sel as u8) });
        self.clkgen.ctrl1().modify(|_, w| {
            w.xtal_en().clear_bit();
            w.xtal_n_en().clear_bit();
            w
        });
        match self.ref_clk_sel {
            ReferenceClockSelect::None => pll_setup_delay(),
            ReferenceClockSelect::XtalOsc => {
                self.clkgen.ctrl1().modify(|_, w| w.xtal_en().set_bit());
                hbo_clock_delay_ms(XTAL_OSC_TSTART_MS);
            }
            ReferenceClockSelect::XtalN => {
                self.clkgen.ctrl1().modify(|_, w| w.xtal_n_en().set_bit());
                pll_setup_delay()
            }
        }

        // Set up PLL configuration.
        match self.pll_cfg {
            Some(cfg) => {
                self.clkgen.ctrl0().modify(|_, w| w.pll_pwdn().clear_bit());
                // Done in C HAL. I guess this gives the PLL some time to power down properly.
                cortex_m::asm::nop();
                cortex_m::asm::nop();
                self.clkgen.ctrl0().modify(|_, w| {
                    unsafe {
                        w.pll_clkf().bits(cfg.clkf);
                    }
                    unsafe {
                        w.pll_clkr().bits(cfg.clkr);
                    }
                    unsafe {
                        w.pll_clkod().bits(cfg.clkod);
                    }
                    unsafe {
                        w.pll_bwadj().bits(cfg.bwadj);
                    }
                    w.pll_test().clear_bit();
                    w.pll_bypass().clear_bit();
                    w.pll_intfb().set_bit()
                });
                // Taken from SystemCoreClockUpdate implementation from Vorago.
                final_sysclk /= cfg.clkr as u32 + 1;
                final_sysclk *= cfg.clkf as u32 + 1;
                final_sysclk /= cfg.clkod as u32 + 1;

                // Reset PLL.
                self.clkgen.ctrl0().modify(|_, w| w.pll_reset().set_bit());
                // The HAL does this, the datasheet specifies a delay of 5 us. I guess it does not
                // really matter because the PLL lock detect is used later..
                pll_setup_delay();
                self.clkgen.ctrl0().modify(|_, w| w.pll_reset().clear_bit());
                pll_setup_delay();

                // check for lock
                let stat = self.clkgen.stat().read();
                if stat.fbslip().bit() || stat.rfslip().bit() {
                    pll_setup_delay();
                    if stat.fbslip().bit() || stat.rfslip().bit() {
                        // This is what the HAL does. We could continue, but then we would at least
                        // have to somehow report a partial error.. Chances are, the user does not
                        // want to continue with a broken PLL clock.
                        return Err(ClockConfigError::PllInitError);
                    }
                }
            }
            None => {
                self.clkgen.ctrl0().modify(|_, w| w.pll_pwdn().set_bit());
            }
        }

        if self.clk_lost_detection {
            rearm_sysclk_lost_with_periph(&self.clkgen)
        }
        #[cfg(feature = "revb")]
        if self.pll_lock_lost_detection {
            rearm_pll_lock_lost_with_periph(&self.clkgen)
        }

        self.clkgen
            .ctrl0()
            .modify(|_, w| unsafe { w.clk_div_sel().bits(self.clk_div_sel as u8) });
        final_sysclk = clock_after_division(final_sysclk, self.clk_div_sel);

        // The HAL does this. I don't know why..
        pll_setup_delay();

        self.clkgen
            .ctrl0()
            .modify(|_, w| unsafe { w.clksel_sys().bits(self.clksel_sys as u8) });

        Ok(Clocks::__new(
            final_sysclk,
            #[cfg(not(feature = "va41628"))]
            self.cfg_adc_clk_div(final_sysclk),
        ))
    }

    #[cfg(not(feature = "va41628"))]
    fn cfg_adc_clk_div(&self, final_sysclk: Hertz) -> Hertz {
        // I will just do the ADC stuff like Vorago does it.
        // ADC clock (must be 2-12.5 MHz)
        // NOTE: Not using divide by 1 or /2 ratio in REVA silicon because of triggering issue
        // For this reason, keep SYSCLK above 8MHz to have the ADC /4 ratio in range)
        if final_sysclk.raw() <= ADC_MAX_CLK.raw() * 4 {
            self.clkgen.ctrl1().modify(|_, w| unsafe {
                w.adc_clk_div_sel().bits(AdcClockDivisorSelect::Div4 as u8)
            });
            final_sysclk / 4
        } else {
            self.clkgen.ctrl1().modify(|_, w| unsafe {
                w.adc_clk_div_sel().bits(AdcClockDivisorSelect::Div8 as u8)
            });
            final_sysclk / 8
        }
    }
}

pub fn rearm_sysclk_lost() {
    rearm_sysclk_lost_with_periph(&unsafe { pac::Clkgen::steal() })
}

fn rearm_sysclk_lost_with_periph(clkgen: &pac::Clkgen) {
    clkgen
        .ctrl0()
        .modify(|_, w| w.sys_clk_lost_det_en().set_bit());
    clkgen
        .ctrl1()
        .write(|w| w.sys_clk_lost_det_rearm().set_bit());
    clkgen
        .ctrl1()
        .write(|w| w.sys_clk_lost_det_rearm().clear_bit());
}

#[cfg(feature = "revb")]
pub fn rearm_pll_lock_lost() {
    rearm_pll_lock_lost_with_periph(&unsafe { pac::Clkgen::steal() })
}

fn rearm_pll_lock_lost_with_periph(clkgen: &pac::Clkgen) {
    clkgen
        .ctrl1()
        .modify(|_, w| w.pll_lost_lock_det_en().set_bit());
    clkgen.ctrl1().write(|w| w.pll_lck_det_rearm().set_bit());
    clkgen.ctrl1().write(|w| w.pll_lck_det_rearm().clear_bit());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_basic_div() {
        assert_eq!(
            clock_after_division(Hertz::from_raw(10_000_000), super::ClockDivisorSelect::Div2),
            Hertz::from_raw(5_000_000)
        );
    }
}
