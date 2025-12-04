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

macro_rules! define_tim_pin_traits {
    ([$(($tim_name:ident, $index:expr)),* $(,)?]) => {
        $(
            pub trait $tim_name: AnyPin {
                const PIN_ID: DynPinId;
                const FUNC_SEL: FunctionSelect;
                const TIM_ID: TimId = TimId::new_unchecked($index);
            }
        )*
    };
}

define_tim_pin_traits!([
    (Tim0Pin, 0),
    (Tim1Pin, 1),
    (Tim2Pin, 2),
    (Tim3Pin, 3),
    (Tim4Pin, 4),
    (Tim5Pin, 5),
    (Tim6Pin, 6),
    (Tim7Pin, 7),
    (Tim8Pin, 8),
    (Tim9Pin, 9),
    (Tim10Pin, 10),
    (Tim11Pin, 11),
    (Tim12Pin, 12),
    (Tim13Pin, 13),
    (Tim14Pin, 14),
    (Tim15Pin, 15),
    (Tim16Pin, 16),
    (Tim17Pin, 17),
    (Tim18Pin, 18),
    (Tim19Pin, 19),
    (Tim20Pin, 20),
    (Tim21Pin, 21),
    (Tim22Pin, 22),
    (Tim23Pin, 23),
]);

pub trait TimInstance: Sealed {
    const ID: TimId;

    #[cfg(feature = "vor4x")]
    const IRQ: va416xx::Interrupt;

    #[cfg(feature = "vor4x")]
    fn clock(clocks: &crate::clock::Clocks) -> Hertz {
        if Self::ID.value() < 16 {
            clocks.apb1()
        } else {
            clocks.apb2()
        }
    }
}

pub trait Tim0Instance: TimInstance {}
pub trait Tim1Instance: TimInstance {}
pub trait Tim2Instance: TimInstance {}
pub trait Tim3Instance: TimInstance {}
pub trait Tim4Instance: TimInstance {}
pub trait Tim5Instance: TimInstance {}
pub trait Tim6Instance: TimInstance {}
pub trait Tim7Instance: TimInstance {}
pub trait Tim8Instance: TimInstance {}
pub trait Tim9Instance: TimInstance {}
pub trait Tim10Instance: TimInstance {}
pub trait Tim11Instance: TimInstance {}
pub trait Tim12Instance: TimInstance {}
pub trait Tim13Instance: TimInstance {}
pub trait Tim14Instance: TimInstance {}
pub trait Tim15Instance: TimInstance {}
pub trait Tim16Instance: TimInstance {}
pub trait Tim17Instance: TimInstance {}
pub trait Tim18Instance: TimInstance {}
pub trait Tim19Instance: TimInstance {}
pub trait Tim20Instance: TimInstance {}
pub trait Tim21Instance: TimInstance {}
pub trait Tim22Instance: TimInstance {}
pub trait Tim23Instance: TimInstance {}

macro_rules! tim_marker {
    ($TIMX:path, $Trait:ident, $ID:expr) => {
        impl Sealed for $TIMX {}

        impl TimInstance for $TIMX {
            const ID: TimId = TimId::new_unchecked($ID);
        }

        impl $Trait for $TIMX {}
    };
    ($TIMX:path, $Trait:ident, $ID:expr, $IrqId:ident) => {
        impl Sealed for $TIMX {}

        impl TimInstance for $TIMX {
            const ID: TimId = TimId::new_unchecked($ID);
            const IRQ: va416xx::Interrupt = va416xx::Interrupt::$IrqId;
        }

        impl $Trait for $TIMX {}
    };
}

cfg_if::cfg_if! {
    if #[cfg(feature = "vor1x")] {
        tim_marker!(pac::Tim0, Tim0Instance, 0);
        tim_marker!(pac::Tim1, Tim1Instance, 1);
        tim_marker!(pac::Tim2, Tim2Instance, 2);
        tim_marker!(pac::Tim3, Tim3Instance, 3);
        tim_marker!(pac::Tim4, Tim4Instance, 4);
        tim_marker!(pac::Tim5, Tim5Instance, 5);
        tim_marker!(pac::Tim6, Tim6Instance, 6);
        tim_marker!(pac::Tim7, Tim7Instance, 7);
        tim_marker!(pac::Tim8, Tim8Instance, 8);
        tim_marker!(pac::Tim9, Tim9Instance, 9);
        tim_marker!(pac::Tim10, Tim10Instance, 10);
        tim_marker!(pac::Tim11, Tim11Instance, 11);
        tim_marker!(pac::Tim12, Tim12Instance, 12);
        tim_marker!(pac::Tim13, Tim13Instance, 13);
        tim_marker!(pac::Tim14, Tim14Instance, 14);
        tim_marker!(pac::Tim15, Tim15Instance, 15);
        tim_marker!(pac::Tim16, Tim16Instance, 16);
        tim_marker!(pac::Tim17, Tim17Instance, 17);
        tim_marker!(pac::Tim18, Tim18Instance, 18);
        tim_marker!(pac::Tim19, Tim19Instance, 19);
        tim_marker!(pac::Tim20, Tim20Instance, 20);
        tim_marker!(pac::Tim21, Tim21Instance, 21);
        tim_marker!(pac::Tim22, Tim22Instance, 22);
        tim_marker!(pac::Tim23, Tim23Instance, 23);
    } else if #[cfg(feature = "vor4x")] {
        tim_marker!(pac::Tim0, Tim0Instance, 0, TIM0);
        tim_marker!(pac::Tim1, Tim1Instance, 1, TIM1);
        tim_marker!(pac::Tim2, Tim2Instance, 2, TIM2);
        tim_marker!(pac::Tim3, Tim3Instance, 3, TIM3);
        tim_marker!(pac::Tim4, Tim4Instance, 4, TIM4);
        tim_marker!(pac::Tim5, Tim5Instance, 5, TIM5);
        tim_marker!(pac::Tim6, Tim6Instance, 6, TIM6);
        tim_marker!(pac::Tim7, Tim7Instance, 7, TIM7);
        tim_marker!(pac::Tim8, Tim8Instance, 8, TIM8);
        tim_marker!(pac::Tim9, Tim9Instance, 9, TIM9);
        tim_marker!(pac::Tim10, Tim10Instance, 10, TIM10);
        tim_marker!(pac::Tim11, Tim11Instance, 11, TIM11);
        tim_marker!(pac::Tim12, Tim12Instance, 12, TIM12);
        tim_marker!(pac::Tim13, Tim13Instance, 13, TIM13);
        tim_marker!(pac::Tim14, Tim14Instance, 14, TIM14);
        tim_marker!(pac::Tim15, Tim15Instance, 15, TIM15);
        tim_marker!(pac::Tim16, Tim16Instance, 16, TIM16);
        tim_marker!(pac::Tim17, Tim17Instance, 17, TIM17);
        tim_marker!(pac::Tim18, Tim18Instance, 18, TIM18);
        tim_marker!(pac::Tim19, Tim19Instance, 19, TIM19);
        tim_marker!(pac::Tim20, Tim20Instance, 20, TIM20);
        tim_marker!(pac::Tim21, Tim21Instance, 21, TIM21);
        tim_marker!(pac::Tim22, Tim22Instance, 22, TIM22);
        tim_marker!(pac::Tim23, Tim23Instance, 23, TIM23);
    }
}

pub trait ValidTimAndPin0<Pin: Tim0Pin, Tim: Tim0Instance>: Sealed {}
pub trait ValidTimAndPin1<Pin: Tim1Pin, Tim: Tim1Instance>: Sealed {}
pub trait ValidTimAndPin2<Pin: Tim2Pin, Tim: Tim2Instance>: Sealed {}
pub trait ValidTimAndPin3<Pin: Tim3Pin, Tim: Tim3Instance>: Sealed {}
pub trait ValidTimAndPin4<Pin: Tim4Pin, Tim: Tim4Instance>: Sealed {}
pub trait ValidTimAndPin5<Pin: Tim5Pin, Tim: Tim5Instance>: Sealed {}
pub trait ValidTimAndPin6<Pin: Tim6Pin, Tim: Tim6Instance>: Sealed {}
pub trait ValidTimAndPin7<Pin: Tim7Pin, Tim: Tim7Instance>: Sealed {}
pub trait ValidTimAndPin8<Pin: Tim8Pin, Tim: Tim8Instance>: Sealed {}
pub trait ValidTimAndPin9<Pin: Tim9Pin, Tim: Tim9Instance>: Sealed {}
pub trait ValidTimAndPin10<Pin: Tim10Pin, Tim: Tim10Instance>: Sealed {}
pub trait ValidTimAndPin11<Pin: Tim11Pin, Tim: Tim11Instance>: Sealed {}
pub trait ValidTimAndPin12<Pin: Tim12Pin, Tim: Tim12Instance>: Sealed {}
pub trait ValidTimAndPin13<Pin: Tim13Pin, Tim: Tim13Instance>: Sealed {}
pub trait ValidTimAndPin14<Pin: Tim14Pin, Tim: Tim14Instance>: Sealed {}
pub trait ValidTimAndPin15<Pin: Tim15Pin, Tim: Tim15Instance>: Sealed {}
pub trait ValidTimAndPin16<Pin: Tim16Pin, Tim: Tim16Instance>: Sealed {}
pub trait ValidTimAndPin17<Pin: Tim17Pin, Tim: Tim17Instance>: Sealed {}
pub trait ValidTimAndPin18<Pin: Tim18Pin, Tim: Tim18Instance>: Sealed {}
pub trait ValidTimAndPin19<Pin: Tim19Pin, Tim: Tim19Instance>: Sealed {}
pub trait ValidTimAndPin20<Pin: Tim20Pin, Tim: Tim20Instance>: Sealed {}
pub trait ValidTimAndPin21<Pin: Tim21Pin, Tim: Tim21Instance>: Sealed {}
pub trait ValidTimAndPin22<Pin: Tim22Pin, Tim: Tim22Instance>: Sealed {}
pub trait ValidTimAndPin23<Pin: Tim23Pin, Tim: Tim23Instance>: Sealed {}

#[macro_use]
mod macros {
    macro_rules! pin_and_tim {
        ($Px:ident, $FunSel:path, $ID:expr) => {
            paste::paste! {
                impl [<Tim $ID Pin>] for crate::pins::Pin<$Px>
                where
                    $Px: crate::pins::PinId,
                {
                    const PIN_ID: crate::pins::DynPinId = $Px::ID;
                    const FUNC_SEL: crate::FunctionSelect = $FunSel;
                    const TIM_ID: crate::timer::TimId = crate::timer::TimId::new_unchecked($ID);
                }
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
