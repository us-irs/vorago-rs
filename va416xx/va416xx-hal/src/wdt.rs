//! # API for the Watchdog peripheral
use vorago_shared_hal::{PeripheralSelect, enable_peripheral_clock, reset_peripheral_for_cycles};

use crate::time::Hertz;
use crate::{clock::Clocks, pac};
use crate::{disable_nvic_interrupt, enable_nvic_interrupt};

/// Value written to WDOGLOCK to unlock the watchdog registers.
pub const WDT_UNLOCK_VALUE: u32 = 0x1ACC_E551;

/// Watchdog peripheral driver.
pub struct Wdt {
    clock_freq: Hertz,
    wdt: pac::WatchDog,
}

/// Type alias for backwards compatibility
#[deprecated(since = "0.2.0", note = "Please use `Wdt` instead")]
pub type WdtController = Wdt;

/// Enable the watchdog interrupt
///
/// # Safety
///
/// This function is `unsafe` because it can break mask-based critical sections.
#[inline]
pub unsafe fn enable_wdt_interrupts() {
    unsafe { enable_nvic_interrupt(pac::Interrupt::WATCHDOG) }
}

/// Disable the watchdog interrupt.
#[inline]
pub fn disable_wdt_interrupts() {
    disable_nvic_interrupt(pac::Interrupt::WATCHDOG)
}

impl Wdt {
    /// Create and start a new watchdog driver with the given timeout in milliseconds.
    pub fn new(wdt: pac::WatchDog, clocks: &Clocks, wdt_freq_ms: u32) -> Self {
        Self::start(wdt, clocks, wdt_freq_ms)
    }

    /// Create and start a new watchdog driver with the given timeout in milliseconds.
    pub fn start(wdt: pac::WatchDog, clocks: &Clocks, wdt_freq_ms: u32) -> Self {
        enable_peripheral_clock(PeripheralSelect::Watchdog);
        reset_peripheral_for_cycles(PeripheralSelect::Watchdog, 2);

        let wdt_clock = clocks.apb2();
        let mut wdt_ctrl = Self {
            clock_freq: wdt_clock,
            wdt,
        };
        wdt_ctrl.set_freq(wdt_freq_ms);
        wdt_ctrl.wdt.wdogcontrol().write(|w| w.inten().set_bit());
        wdt_ctrl.feed();
        // Unmask the watchdog interrupt
        unsafe {
            enable_wdt_interrupts();
        }
        wdt_ctrl
    }

    /// Set the watchdog timeout in milliseconds.
    #[inline]
    pub fn set_freq(&mut self, freq_ms: u32) {
        let counter = (self.clock_freq.to_raw() / 1000) * freq_ms;
        self.wdt.wdogload().write(|w| unsafe { w.bits(counter) });
    }

    /// Disable the watchdog reset on timeout.
    #[inline]
    pub fn disable_reset(&mut self) {
        self.wdt.wdogcontrol().modify(|_, w| w.resen().clear_bit());
    }

    /// Enable the watchdog reset on timeout.
    #[inline]
    pub fn enable_reset(&mut self) {
        self.wdt.wdogcontrol().modify(|_, w| w.resen().set_bit());
    }

    /// Current watchdog counter value.
    #[inline]
    pub fn counter(&self) -> u32 {
        self.wdt.wdogvalue().read().bits()
    }

    /// Feed (reset) the watchdog counter.
    #[inline]
    pub fn feed(&self) {
        self.wdt.wdogintclr().write(|w| unsafe { w.bits(1) });
    }

    /// Lock the watchdog registers against further writes.
    #[inline]
    pub fn lock(&self) {
        self.wdt.wdoglock().write(|w| unsafe { w.bits(0) });
    }

    /// Unlock the watchdog registers for writing.
    #[inline]
    pub fn unlock(&self) {
        self.wdt
            .wdoglock()
            .write(|w| unsafe { w.bits(WDT_UNLOCK_VALUE) });
    }

    /// Whether the watchdog registers are currently locked.
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.wdt.wdogload().read().bits() == 1
    }
}
