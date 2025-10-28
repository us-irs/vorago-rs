pub mod regs;

use core::convert::Infallible;

#[cfg(feature = "vor1x")]
pub use crate::InterruptConfig;
#[cfg(feature = "vor1x")]
use crate::sysconfig::enable_peripheral_clock;
pub use regs::{CascadeSource, InvalidTimerIndex, TimId};

use crate::{enable_nvic_interrupt, sealed::Sealed, time::Hertz};
use crate::{gpio::DynPinId, ioconfig::regs::FunctionSelect, pins::AnyPin};
use fugit::RateExtU32;

#[cfg(feature = "vor1x")]
use crate::PeripheralSelect;

#[cfg(feature = "vor1x")]
use va108xx as pac;
#[cfg(feature = "vor4x")]
use va416xx as pac;

#[cfg(feature = "vor4x")]
pub const TIM_IRQ_OFFSET: usize = 48;

//==================================================================================================
// Defintions
//==================================================================================================

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CascadeControl {
    /// Enable Cascade 0 signal active as a requirement for counting
    pub enable_src_0: bool,
    /// Invert Cascade 0, making it active low
    pub inv_src_0: regs::CascadeInvert,
    /// Enable Cascade 1 signal active as a requirement for counting
    pub enable_src_1: bool,
    /// Invert Cascade 1, making it active low
    pub inv_src_1: regs::CascadeInvert,
    /// Specify required operation if both Cascade 0 and Cascade 1 are active.
    /// 0 is a logical AND of both cascade signals, 1 is a logical OR
    pub dual_operation: regs::DualCascadeOp,
    /// Enable trigger mode for Cascade 0. In trigger mode, couting will start with the selected
    /// cascade signal active, but once the counter is active, cascade control will be ignored
    pub trigger_mode_0: bool,
    /// Trigger mode, identical to [Self::trigger_mode_0] but for Cascade 1
    pub trigger_mode_1: bool,
    /// Enable Cascade 2 signal active as a requirement to stop counting. This mode is similar
    /// to the REQ_STOP control bit, but signalled by a Cascade source
    pub enable_stop_src_2: bool,
    /// Invert Cascade 2, making it active low
    pub inv_src_2: regs::CascadeInvert,
    /// The counter is automatically disabled if the corresponding Cascade 2 level-sensitive input
    /// souce is active when the count reaches 0. If the counter is not 0, the cascade control is
    /// ignored
    pub trigger_mode_2: bool,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CascadeSelect {
    Csd0 = 0,
    Csd1 = 1,
    Csd2 = 2,
}

//==================================================================================================
// Valid TIM and PIN combinations
//==================================================================================================

pub trait TimPin: AnyPin {
    const PIN_ID: DynPinId;
    const FUN_SEL: FunctionSelect;
    const TIM_ID: TimId;
}

pub trait TimInstance: Sealed {
    // TIM ID ranging from 0 to 23 for 24 TIM peripherals
    const ID: TimId;
    #[cfg(feature = "vor4x")]
    const IRQ: va416xx::Interrupt;

    #[cfg(feature = "vor4x")]
    fn clock(clocks: &crate::clock::Clocks) -> Hertz {
        if Self::ID.value() <= 15 {
            clocks.apb1()
        } else {
            clocks.apb2()
        }
    }
}

macro_rules! tim_marker {
    ($TIMX:path, $ID:expr) => {
        impl TimInstance for $TIMX {
            const ID: TimId = TimId::new_unchecked($ID);
        }

        impl Sealed for $TIMX {}
    };
    ($TIMX:path, $ID:expr, $IrqId:ident) => {
        impl TimInstance for $TIMX {
            const ID: TimId = TimId::new_unchecked($ID);
            const IRQ: va416xx::Interrupt = va416xx::Interrupt::$IrqId;
        }

        impl Sealed for $TIMX {}
    };
}

cfg_if::cfg_if! {
    if #[cfg(feature = "vor1x")] {
        tim_marker!(pac::Tim0, 0);
        tim_marker!(pac::Tim1, 1);
        tim_marker!(pac::Tim2, 2);
        tim_marker!(pac::Tim3, 3);
        tim_marker!(pac::Tim4, 4);
        tim_marker!(pac::Tim5, 5);
        tim_marker!(pac::Tim6, 6);
        tim_marker!(pac::Tim7, 7);
        tim_marker!(pac::Tim8, 8);
        tim_marker!(pac::Tim9, 9);
        tim_marker!(pac::Tim10, 10);
        tim_marker!(pac::Tim11, 11);
        tim_marker!(pac::Tim12, 12);
        tim_marker!(pac::Tim13, 13);
        tim_marker!(pac::Tim14, 14);
        tim_marker!(pac::Tim15, 15);
        tim_marker!(pac::Tim16, 16);
        tim_marker!(pac::Tim17, 17);
        tim_marker!(pac::Tim18, 18);
        tim_marker!(pac::Tim19, 19);
        tim_marker!(pac::Tim20, 20);
        tim_marker!(pac::Tim21, 21);
        tim_marker!(pac::Tim22, 22);
        tim_marker!(pac::Tim23, 23);
    } else if #[cfg(feature = "vor4x")] {
        tim_marker!(pac::Tim0, 0, TIM0);
        tim_marker!(pac::Tim1, 1, TIM1);
        tim_marker!(pac::Tim2, 2, TIM2);
        tim_marker!(pac::Tim3, 3, TIM3);
        tim_marker!(pac::Tim4, 4, TIM4);
        tim_marker!(pac::Tim5, 5, TIM5);
        tim_marker!(pac::Tim6, 6, TIM6);
        tim_marker!(pac::Tim7, 7, TIM7);
        tim_marker!(pac::Tim8, 8, TIM8);
        tim_marker!(pac::Tim9, 9, TIM9);
        tim_marker!(pac::Tim10, 10, TIM10);
        tim_marker!(pac::Tim11, 11, TIM11);
        tim_marker!(pac::Tim12, 12, TIM12);
        tim_marker!(pac::Tim13, 13, TIM13);
        tim_marker!(pac::Tim14, 14, TIM14);
        tim_marker!(pac::Tim15, 15, TIM15);
        tim_marker!(pac::Tim16, 16, TIM16);
        tim_marker!(pac::Tim17, 17, TIM17);
        tim_marker!(pac::Tim18, 18, TIM18);
        tim_marker!(pac::Tim19, 19, TIM19);
        tim_marker!(pac::Tim20, 20, TIM20);
        tim_marker!(pac::Tim21, 21, TIM21);
        tim_marker!(pac::Tim22, 22, TIM22);
        tim_marker!(pac::Tim23, 23, TIM23);
    }
}

pub trait ValidTimAndPin<Pin: TimPin, Tim: TimInstance>: Sealed {}

#[macro_use]
mod macros {
    macro_rules! pin_and_tim {
        ($Px:ident, $FunSel:path, $ID:expr) => {
            impl TimPin for Pin<$Px>
            where
                $Px: PinId,
            {
                const PIN_ID: DynPinId = $Px::ID;
                const FUN_SEL: FunctionSelect = $FunSel;
                const TIM_ID: TimId = TimId::new_unchecked($ID);
            }
        };
    }
}

#[cfg(feature = "vor1x")]
pub mod pins_vor1x;
#[cfg(feature = "vor4x")]
pub mod pins_vor4x;

//==================================================================================================
// Timers
//==================================================================================================

/// Hardware timers
pub struct CountdownTimer {
    id: TimId,
    regs: regs::MmioTimer<'static>,
    curr_freq: Hertz,
    ref_clk: Hertz,
    rst_val: u32,
    last_cnt: u32,
}

impl CountdownTimer {
    /// Create a countdown timer structure for a given TIM peripheral.
    ///
    /// This does not enable the timer. You can use the [Self::load], [Self::start],
    /// [Self::enable_interrupt] and [Self::enable] API to set up and configure the countdown
    /// timer.
    #[cfg(feature = "vor1x")]
    pub fn new<Tim: TimInstance>(_tim: Tim, sys_clk: Hertz) -> Self {
        enable_tim_clk(Tim::ID);
        assert_tim_reset_for_cycles(Tim::ID, 2);
        CountdownTimer {
            id: Tim::ID,
            regs: regs::Timer::new_mmio(Tim::ID),
            ref_clk: sys_clk,
            rst_val: 0,
            curr_freq: 0.Hz(),
            last_cnt: 0,
        }
    }

    /// Create a countdown timer structure for a given TIM peripheral.
    ///
    /// This does not enable the timer. You can use the [Self::load], [Self::start],
    /// [Self::enable_interrupt] and [Self::enable] API to set up and configure the countdown
    /// timer.
    #[cfg(feature = "vor4x")]
    pub fn new<Tim: TimInstance>(_tim: Tim, clks: &crate::clock::Clocks) -> Self {
        enable_tim_clk(Tim::ID);
        assert_tim_reset_for_cycles(Tim::ID, 2);
        CountdownTimer {
            id: Tim::ID,
            regs: regs::Timer::new_mmio(Tim::ID),
            ref_clk: clks.apb1(),
            rst_val: 0,
            curr_freq: 0.Hz(),
            last_cnt: 0,
        }
    }

    #[inline]
    pub fn perid(&self) -> u32 {
        self.regs.read_perid()
    }

    #[inline(always)]
    pub fn enable(&mut self) {
        self.regs
            .write_enable_control(regs::EnableControl::new_enable());
    }
    #[inline(always)]
    pub fn disable(&mut self) {
        self.regs
            .write_enable_control(regs::EnableControl::new_disable());
    }

    #[cfg(feature = "vor1x")]
    pub fn enable_interrupt(&mut self, irq_cfg: InterruptConfig) {
        if irq_cfg.route {
            let irqsel = unsafe { pac::Irqsel::steal() };
            enable_peripheral_clock(PeripheralSelect::Irqsel);
            irqsel
                .tim(self.id.value() as usize)
                .write(|w| unsafe { w.bits(irq_cfg.id as u32) });
        }
        if irq_cfg.enable_in_nvic {
            unsafe { enable_nvic_interrupt(irq_cfg.id) };
        }
        self.regs.modify_control(|mut value| {
            value.set_irq_enable(true);
            value
        });
    }

    #[cfg(feature = "vor4x")]
    #[inline(always)]
    pub fn enable_interrupt(&mut self, enable_in_nvic: bool) {
        if enable_in_nvic {
            unsafe { enable_nvic_interrupt(self.id.interrupt_id()) };
        }
        self.regs.modify_control(|mut value| {
            value.set_irq_enable(true);
            value
        });
    }

    /// This function only clears the interrupt enable bit.
    ///
    /// It does not mask the interrupt in the NVIC or un-route the IRQ.
    #[inline(always)]
    pub fn disable_interrupt(&mut self) {
        self.regs.modify_control(|mut value| {
            value.set_irq_enable(false);
            value
        });
    }

    /// Calls [Self::load] to configure the specified frequency and then calls [Self::enable].
    pub fn start(&mut self, frequency: impl Into<Hertz>) {
        self.load(frequency);
        self.enable();
    }

    /// Return `Ok` if the timer has wrapped. Peripheral will automatically clear the
    /// flag and restart the time if configured correctly
    pub fn wait(&mut self) -> nb::Result<(), Infallible> {
        let cnt = self.counter();
        if (cnt > self.last_cnt) || cnt == 0 {
            self.last_cnt = self.rst_val;
            Ok(())
        } else {
            self.last_cnt = cnt;
            Err(nb::Error::WouldBlock)
        }
    }

    /// Load the count down timer with a timeout but do not start it.
    pub fn load(&mut self, timeout: impl Into<Hertz>) {
        self.disable();
        self.curr_freq = timeout.into();
        self.rst_val = self.ref_clk.raw() / self.curr_freq.raw();
        self.set_reload(self.rst_val);
        self.set_count(self.rst_val);
    }

    #[inline(always)]
    pub fn set_reload(&mut self, val: u32) {
        self.regs.write_reset_value(val);
    }

    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.regs.write_count_value(val);
    }

    #[inline(always)]
    pub fn counter(&self) -> u32 {
        self.regs.read_count_value()
    }

    /// Disable the counter, setting both enable and active bit to 0
    #[inline]
    pub fn auto_disable(&mut self, enable: bool) {
        self.regs.modify_control(|mut value| {
            value.set_auto_disable(enable);
            value
        });
    }

    /// This option only applies when the Auto-Disable functionality is 0.
    ///
    /// The active bit is changed to 0 when count reaches 0, but the counter stays
    /// enabled. When Auto-Disable is 1, Auto-Deactivate is implied
    #[inline]
    pub fn auto_deactivate(&mut self, enable: bool) {
        self.regs.modify_control(|mut value| {
            value.set_auto_deactivate(enable);
            value
        });
    }

    /// Configure the cascade parameters
    pub fn cascade_control(&mut self, ctrl: CascadeControl) {
        self.regs.write_cascade_control(
            regs::CascadeControl::builder()
                .with_trigger2(ctrl.trigger_mode_2)
                .with_inv2(ctrl.inv_src_2)
                .with_en2(ctrl.enable_stop_src_2)
                .with_trigger1(ctrl.trigger_mode_1)
                .with_trigger0(ctrl.trigger_mode_0)
                .with_dual_cascade_op(ctrl.dual_operation)
                .with_inv1(ctrl.inv_src_1)
                .with_en1(ctrl.enable_src_1)
                .with_inv0(ctrl.inv_src_0)
                .with_en0(ctrl.enable_src_0)
                .build(),
        );
    }

    pub fn cascade_source(
        &mut self,
        cascade_index: CascadeSelect,
        src: regs::CascadeSource,
    ) -> Result<(), regs::InvalidCascadeSourceId> {
        // Safety: Index range safe by enum values.
        unsafe {
            self.regs
                .write_cascade_unchecked(cascade_index as usize, regs::CascadeSourceReg::new(src)?);
        }
        Ok(())
    }

    pub fn curr_freq(&self) -> Hertz {
        self.curr_freq
    }

    /// Disables the TIM and the dedicated TIM clock.
    pub fn stop_with_clock_disable(mut self) {
        self.disable();
        unsafe { pac::Sysconfig::steal() }
            .tim_clk_enable()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.id.value())) });
    }
}

//==================================================================================================
// Delay implementations
//==================================================================================================
//
impl embedded_hal::delay::DelayNs for CountdownTimer {
    fn delay_ns(&mut self, ns: u32) {
        let ticks = (u64::from(ns)) * (u64::from(self.ref_clk.raw())) / 1_000_000_000;

        let full_cycles = ticks >> 32;
        let mut last_count;
        let mut new_count;
        if full_cycles > 0 {
            self.set_reload(u32::MAX);
            self.set_count(u32::MAX);
            self.enable();

            for _ in 0..full_cycles {
                // Always ensure that both values are the same at the start.
                new_count = self.counter();
                last_count = new_count;
                loop {
                    new_count = self.counter();
                    if new_count == 0 {
                        // Wait till timer has wrapped.
                        while self.counter() == 0 {
                            cortex_m::asm::nop()
                        }
                        break;
                    }
                    // Timer has definitely wrapped.
                    if new_count > last_count {
                        break;
                    }
                    last_count = new_count;
                }
            }
        }
        let ticks = (ticks & u32::MAX as u64) as u32;
        self.disable();
        if ticks > 1 {
            self.set_reload(ticks);
            self.set_count(ticks);
            self.enable();
            last_count = ticks;

            loop {
                new_count = self.counter();
                if new_count == 0 || (new_count > last_count) {
                    break;
                }
                last_count = new_count;
            }
        }

        self.disable();
    }
}

#[inline(always)]
pub fn enable_tim_clk(id: TimId) {
    unsafe { pac::Sysconfig::steal() }
        .tim_clk_enable()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << id.value())) });
}

#[inline(always)]
pub fn disable_tim_clk(id: TimId) {
    unsafe { pac::Sysconfig::steal() }
        .tim_clk_enable()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << (id.value()))) });
}

/// Clear the reset bit of the TIM, holding it in reset
///
/// # Safety
///
/// Only the bit related to the corresponding TIM peripheral is modified
#[inline]
pub fn assert_tim_reset(id: TimId) {
    unsafe { pac::Peripherals::steal() }
        .sysconfig
        .tim_reset()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << id.value())) });
}

#[inline]
pub fn deassert_tim_reset(tim: TimId) {
    unsafe { pac::Peripherals::steal() }
        .sysconfig
        .tim_reset()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << tim.value())) });
}

pub fn assert_tim_reset_for_cycles(tim: TimId, cycles: u32) {
    assert_tim_reset(tim);
    cortex_m::asm::delay(cycles);
    deassert_tim_reset(tim);
}
