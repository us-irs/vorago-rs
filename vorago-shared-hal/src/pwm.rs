use core::convert::Infallible;
use core::marker::PhantomData;

use crate::gpio::{DynPinId, IoPeriphPin};
use crate::timer::enable_tim_clk;
use crate::timer::regs::{EnableControl, StatusSelect};
use crate::{FunctionSelect, PeripheralSelect, enable_peripheral_clock};

use crate::time::Hertz;
use crate::timer::{self, TimId};
use crate::timer::{
    Tim0Instance, Tim0Pin, Tim1Instance, Tim1Pin, Tim2Instance, Tim2Pin, Tim3Instance, Tim3Pin,
    Tim4Instance, Tim4Pin, Tim5Instance, Tim5Pin, Tim6Instance, Tim6Pin, Tim7Instance, Tim7Pin,
    Tim8Instance, Tim8Pin, Tim9Instance, Tim9Pin, Tim10Instance, Tim10Pin, Tim11Instance, Tim11Pin,
    Tim12Instance, Tim12Pin, Tim13Instance, Tim13Pin, Tim14Instance, Tim14Pin, Tim15Instance,
    Tim15Pin, Tim16Instance, Tim16Pin, Tim17Instance, Tim17Pin, Tim18Instance, Tim18Pin,
    Tim19Instance, Tim19Pin, Tim20Instance, Tim20Pin, Tim21Instance, Tim21Pin, Tim22Instance,
    Tim22Pin, Tim23Instance, Tim23Pin,
};

const DUTY_MAX: u16 = u16::MAX;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwmA {}
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwmB {}

//==================================================================================================
// PWM pin
//==================================================================================================

/// Reduced version where type information is deleted
pub struct PwmPin<Mode = PwmA> {
    tim_id: TimId,
    regs: timer::regs::MmioTimer<'static>,
    ref_clk: Hertz,
    /// For PWMB, this is the upper limit
    current_duty: u16,
    /// For PWMA, this value will not be used
    current_lower_limit: u16,
    current_period: Hertz,
    current_rst_val: u32,
    mode: PhantomData<Mode>,
}

macro_rules! impl_pwm_new_methods {
    (
        $(
            ($tim_name:ident, $pin_trait:ident, $instance_trait:ident)
        ),*
        $(,)?
    ) => {
        paste::paste! {
            $(
                #[doc = concat!("Create a new PWM pin using ", stringify!($tim_name))]
                pub fn [<new_with_ $tim_name:lower>] <Pin: $pin_trait, Tim: $instance_trait>(
                    _pin: Pin,
                    _tim: Tim,
                    #[cfg(feature = "vor1x")] sys_clk: Hertz,
                    #[cfg(feature = "vor4x")] clks: &crate::clock::Clocks,
                    initial_frequency: Hertz,
                ) -> Self {
                    Self::new(
                        Pin::PIN_ID,
                        Pin::FUNC_SEL,
                        Tim::ID,
                        #[cfg(feature = "vor1x")]
                        sys_clk,
                        #[cfg(feature = "vor4x")]
                        clks,
                        initial_frequency,
                    )
                }
            )*
        }
    };
}

impl<Mode> PwmPin<Mode> {
    impl_pwm_new_methods! {
        (Tim0,  Tim0Pin,  Tim0Instance),
        (Tim1,  Tim1Pin,  Tim1Instance),
        (Tim2,  Tim2Pin,  Tim2Instance),
        (Tim3,  Tim3Pin,  Tim3Instance),
        (Tim4,  Tim4Pin,  Tim4Instance),
        (Tim5,  Tim5Pin,  Tim5Instance),
        (Tim6,  Tim6Pin,  Tim6Instance),
        (Tim7,  Tim7Pin,  Tim7Instance),
        (Tim8,  Tim8Pin,  Tim8Instance),
        (Tim9,  Tim9Pin,  Tim9Instance),
        (Tim10, Tim10Pin, Tim10Instance),
        (Tim11, Tim11Pin, Tim11Instance),
        (Tim12, Tim12Pin, Tim12Instance),
        (Tim13, Tim13Pin, Tim13Instance),
        (Tim14, Tim14Pin, Tim14Instance),
        (Tim15, Tim15Pin, Tim15Instance),
        (Tim16, Tim16Pin, Tim16Instance),
        (Tim17, Tim17Pin, Tim17Instance),
        (Tim18, Tim18Pin, Tim18Instance),
        (Tim19, Tim19Pin, Tim19Instance),
        (Tim20, Tim20Pin, Tim20Instance),
        (Tim21, Tim21Pin, Tim21Instance),
        (Tim22, Tim22Pin, Tim22Instance),
        (Tim23, Tim23Pin, Tim23Instance),
    }

    fn new(
        pin_id: DynPinId,
        func_sel: FunctionSelect,
        tim_id: TimId,
        #[cfg(feature = "vor1x")] sys_clk: Hertz,
        #[cfg(feature = "vor4x")] clks: &crate::clock::Clocks,
        initial_frequency: Hertz,
    ) -> Self {
        IoPeriphPin::new(pin_id, func_sel, None);
        let mut pin = PwmPin {
            tim_id,
            regs: timer::regs::Timer::new_mmio(tim_id),
            current_duty: 0,
            current_lower_limit: 0,
            current_period: initial_frequency,
            current_rst_val: 0,
            #[cfg(feature = "vor1x")]
            ref_clk: sys_clk,
            #[cfg(feature = "vor4x")]
            ref_clk: clks.apb1(),
            mode: PhantomData,
        };
        // For Vorago 4x, the presence of the pin structure ensures that its respective peripheral
        // clock was already enabled.
        #[cfg(feature = "vor1x")]
        enable_peripheral_clock(PeripheralSelect::Gpio);
        enable_peripheral_clock(PeripheralSelect::IoConfig);
        enable_tim_clk(tim_id);
        pin.enable_pwm_a();
        pin.set_period(initial_frequency);
        pin
    }

    #[inline]
    fn enable_pwm_a(&mut self) {
        self.regs.modify_control(|mut value| {
            value.set_status_sel(StatusSelect::PwmaOutput);
            value
        });
    }

    #[inline]
    fn enable_pwm_b(&mut self) {
        self.regs.modify_control(|mut value| {
            value.set_status_sel(StatusSelect::PwmbOutput);
            value
        });
    }

    #[inline]
    pub fn get_period(&self) -> Hertz {
        self.current_period
    }

    #[inline]
    pub fn set_period(&mut self, period: impl Into<Hertz>) {
        self.current_period = period.into();
        // Avoid division by 0
        if self.current_period.raw() == 0 {
            return;
        }
        self.current_rst_val = self.ref_clk.raw() / self.current_period.raw();
        self.regs.write_reset_value(self.current_rst_val);
    }

    #[inline]
    pub fn disable(&mut self) {
        self.regs.write_enable_control(EnableControl::new_disable());
    }

    #[inline]
    pub fn enable(&mut self) {
        self.regs.write_enable_control(EnableControl::new_enable());
    }

    #[inline]
    pub fn period(&self) -> Hertz {
        self.current_period
    }

    #[inline(always)]
    pub fn duty(&self) -> u16 {
        self.current_duty
    }
}

impl From<PwmPin<PwmA>> for PwmPin<PwmB> {
    fn from(other: PwmPin<PwmA>) -> Self {
        let mut pwmb = Self {
            mode: PhantomData,
            regs: other.regs,
            tim_id: other.tim_id,
            ref_clk: other.ref_clk,
            current_duty: other.current_duty,
            current_lower_limit: other.current_lower_limit,
            current_period: other.current_period,
            current_rst_val: other.current_rst_val,
        };
        pwmb.enable_pwm_b();
        pwmb
    }
}

impl From<PwmPin<PwmB>> for PwmPin<PwmA> {
    fn from(other: PwmPin<PwmB>) -> Self {
        let mut pwmb = Self {
            mode: PhantomData,
            tim_id: other.tim_id,
            regs: other.regs,
            ref_clk: other.ref_clk,
            current_duty: other.current_duty,
            current_lower_limit: other.current_lower_limit,
            current_period: other.current_period,
            current_rst_val: other.current_rst_val,
        };
        pwmb.enable_pwm_a();
        pwmb
    }
}

//==================================================================================================
// PWMB implementations
//==================================================================================================

impl PwmPin<PwmB> {
    #[inline(always)]
    pub fn pwmb_lower_limit(&self) -> u16 {
        self.current_lower_limit
    }

    #[inline(always)]
    pub fn pwmb_upper_limit(&self) -> u16 {
        self.current_duty
    }

    /// Set the lower limit for PWMB
    ///
    /// The PWM signal will be 1 as long as the current RST counter is larger than
    /// the lower limit. For example, with a lower limit of 0.5 and and an upper limit
    /// of 0.7, Only a fixed period between 0.5 * period and 0.7 * period will be in a high
    /// state
    #[inline(always)]
    pub fn set_pwmb_lower_limit(&mut self, duty: u16) {
        self.current_lower_limit = duty;
        let pwmb_val: u64 =
            (self.current_rst_val as u64 * self.current_lower_limit as u64) / DUTY_MAX as u64;
        self.regs.write_pwmb_value(pwmb_val as u32);
    }

    /// Set the higher limit for PWMB
    ///
    /// The PWM signal will be 1 as long as the current RST counter is smaller than
    /// the higher limit. For example, with a lower limit of 0.5 and and an upper limit
    /// of 0.7, Only a fixed period between 0.5 * period and 0.7 * period will be in a high
    /// state
    pub fn set_pwmb_upper_limit(&mut self, duty: u16) {
        self.current_duty = duty;
        let pwma_val: u64 =
            (self.current_rst_val as u64 * self.current_duty as u64) / DUTY_MAX as u64;
        self.regs.write_pwma_value(pwma_val as u32);
    }
}

//==================================================================================================
// Embedded HAL implementation: PWMA only
//==================================================================================================

impl embedded_hal::pwm::ErrorType for PwmPin {
    type Error = Infallible;
}

impl embedded_hal::pwm::SetDutyCycle for PwmPin {
    #[inline]
    fn max_duty_cycle(&self) -> u16 {
        DUTY_MAX
    }

    #[inline]
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.current_duty = duty;
        let pwma_val: u64 = (self.current_rst_val as u64
            * (DUTY_MAX as u64 - self.current_duty as u64))
            / DUTY_MAX as u64;
        self.regs.write_pwma_value(pwma_val as u32);
        Ok(())
    }
}

/// Get the corresponding u16 duty cycle from a percent value ranging between 0.0 and 1.0.
///
/// Please note that this might load a lot of floating point code because this processor does not
/// have a FPU
pub fn get_duty_from_percent(percent: f32) -> u16 {
    if percent > 1.0 {
        DUTY_MAX
    } else if percent <= 0.0 {
        0
    } else {
        (percent * DUTY_MAX as f32) as u16
    }
}
