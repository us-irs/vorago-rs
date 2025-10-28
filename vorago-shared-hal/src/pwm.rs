use core::convert::Infallible;
use core::marker::PhantomData;

use crate::gpio::IoPeriphPin;
use crate::timer::enable_tim_clk;
use crate::timer::regs::{EnableControl, StatusSelect};
use crate::{PeripheralSelect, enable_peripheral_clock};

use crate::time::Hertz;
use crate::timer::{self, TimId, TimInstance, TimPin};

const DUTY_MAX: u16 = u16::MAX;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwmA {}
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwmB {}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("pin tim ID {pin_tim:?} and timer tim id {tim_id:?} do not match")]
pub struct TimMissmatchError {
    pin_tim: TimId,
    tim_id: TimId,
}

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

impl<Mode> PwmPin<Mode> {
    /// Create a new PWM pin
    pub fn new<Pin: TimPin, Tim: TimInstance>(
        _pin: Pin,
        _tim: Tim,
        #[cfg(feature = "vor1x")] sys_clk: Hertz,
        #[cfg(feature = "vor4x")] clks: &crate::clock::Clocks,
        initial_frequency: Hertz,
    ) -> Result<Self, TimMissmatchError> {
        if Pin::TIM_ID != Tim::ID {
            return Err(TimMissmatchError {
                pin_tim: Pin::TIM_ID,
                tim_id: Tim::ID,
            });
        }
        IoPeriphPin::new(Pin::PIN_ID, Pin::FUN_SEL, None);
        let mut pin = PwmPin {
            tim_id: Tim::ID,
            regs: timer::regs::Timer::new_mmio(Tim::ID),
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
        enable_tim_clk(Tim::ID);
        pin.enable_pwm_a();
        pin.set_period(initial_frequency);
        Ok(pin)
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
