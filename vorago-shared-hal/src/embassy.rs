use core::cell::{Cell, RefCell};

use crate::{
    enable_nvic_interrupt,
    timer::{
        TimId, TimInstance, assert_tim_reset_for_cycles, enable_tim_clk,
        regs::{EnableControl, MmioTimer},
    },
};
use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::TICK_HZ;
use embassy_time_driver::{Driver, time_driver_impl};
use embassy_time_queue_utils::Queue;
use once_cell::sync::OnceCell;
use portable_atomic::{AtomicU32, Ordering};

#[cfg(feature = "vor1x")]
use crate::time::Hertz;
#[cfg(feature = "vor1x")]
use crate::{PeripheralSelect, enable_peripheral_clock};

time_driver_impl!(
    static TIME_DRIVER: TimerDriver = TimerDriver {
        periods: AtomicU32::new(0),
        alarms: Mutex::new(AlarmState::new()),
        queue: Mutex::new(RefCell::new(Queue::new())),
});

/// Expose the time driver so the user can specify the IRQ handlers themselves.
pub fn time_driver() -> &'static TimerDriver {
    &TIME_DRIVER
}

struct AlarmState {
    timestamp: Cell<u64>,
}

impl AlarmState {
    const fn new() -> Self {
        Self {
            timestamp: Cell::new(u64::MAX),
        }
    }
}

unsafe impl Send for AlarmState {}

static SCALE: OnceCell<u64> = OnceCell::new();
static TIMEKEEPER_TIM: OnceCell<TimId> = OnceCell::new();
static ALARM_TIM: OnceCell<TimId> = OnceCell::new();

pub struct TimerDriver {
    periods: AtomicU32,
    /// Timestamp at which to fire alarm. u64::MAX if no alarm is scheduled.
    alarms: Mutex<AlarmState>,
    queue: Mutex<RefCell<Queue>>,
}

impl TimerDriver {
    #[cfg(feature = "vor1x")]
    #[doc(hidden)]
    pub fn __init<TimekeeperTim: TimInstance, AlarmTim: TimInstance>(
        &self,
        sysclk: Hertz,
        _timekeeper_tim: TimekeeperTim,
        _alarm_tim: AlarmTim,
        timekeeper_irq: va108xx::Interrupt,
        alarm_irq: va108xx::Interrupt,
    ) {
        if ALARM_TIM.get().is_some() || TIMEKEEPER_TIM.get().is_some() {
            return;
        }
        ALARM_TIM.set(AlarmTim::ID).ok();
        TIMEKEEPER_TIM.set(TimekeeperTim::ID).ok();
        enable_peripheral_clock(PeripheralSelect::Irqsel);
        enable_tim_clk(TimekeeperTim::ID);
        assert_tim_reset_for_cycles(TimekeeperTim::ID, 2);

        let mut timekeeper_reg_block = unsafe { TimekeeperTim::ID.steal_regs() };
        let mut alarm_tim_reg_block = unsafe { AlarmTim::ID.steal_regs() };
        // Initiate scale value here. This is required to convert timer ticks back to a timestamp.
        SCALE.set((sysclk.raw() / TICK_HZ as u32) as u64).unwrap();
        timekeeper_reg_block.write_reset_value(u32::MAX);
        // Decrementing counter.
        timekeeper_reg_block.write_count_value(u32::MAX);
        let irqsel = unsafe { va108xx::Irqsel::steal() };
        // Switch on. Timekeeping should always be done.
        irqsel
            .tim(TimekeeperTim::ID.value() as usize)
            .write(|w| unsafe { w.bits(timekeeper_irq as u32) });
        unsafe {
            enable_nvic_interrupt(timekeeper_irq);
        }
        timekeeper_reg_block.modify_control(|mut value| {
            value.set_irq_enable(true);
            value
        });
        timekeeper_reg_block.write_enable_control(EnableControl::new_enable());

        enable_tim_clk(AlarmTim::ID);
        assert_tim_reset_for_cycles(AlarmTim::ID, 2);

        // Explicitely disable alarm timer until needed.
        alarm_tim_reg_block.modify_control(|mut value| {
            value.set_irq_enable(false);
            value.set_enable(false);
            value
        });
        // Enable general interrupts. The IRQ enable of the peripheral remains cleared.
        unsafe {
            enable_nvic_interrupt(alarm_irq);
        }
        irqsel
            .tim(AlarmTim::ID.value() as usize)
            .write(|w| unsafe { w.bits(alarm_irq as u32) });
    }

    #[cfg(feature = "vor4x")]
    #[doc(hidden)]
    pub fn __init<TimekeeperTim: TimInstance, AlarmTim: TimInstance>(
        &self,
        _timekeeper_tim: TimekeeperTim,
        _alarm_tim: AlarmTim,
        clocks: &crate::clock::Clocks,
    ) {
        if ALARM_TIM.get().is_some() || TIMEKEEPER_TIM.get().is_some() {
            return;
        }
        ALARM_TIM.set(AlarmTim::ID).ok();
        TIMEKEEPER_TIM.set(TimekeeperTim::ID).ok();
        let mut timekeeper_regs = unsafe { TimekeeperTim::ID.steal_regs() };
        let mut alarm_regs = unsafe { AlarmTim::ID.steal_regs() };

        enable_tim_clk(TimekeeperTim::ID);
        assert_tim_reset_for_cycles(TimekeeperTim::ID, 2);

        // Initiate scale value here. This is required to convert timer ticks back to a timestamp.

        SCALE
            .set((TimekeeperTim::clock(clocks).raw() / TICK_HZ as u32) as u64)
            .unwrap();
        timekeeper_regs.write_reset_value(u32::MAX);
        // Decrementing counter.
        timekeeper_regs.write_count_value(u32::MAX);
        // Switch on. Timekeeping should always be done.
        unsafe {
            enable_nvic_interrupt(TimekeeperTim::IRQ);
        }
        timekeeper_regs.modify_control(|mut value| {
            value.set_irq_enable(true);
            value
        });
        timekeeper_regs.write_enable_control(EnableControl::new_enable());

        enable_tim_clk(AlarmTim::ID);
        assert_tim_reset_for_cycles(AlarmTim::ID, 2);
        // Explicitely disable alarm timer until needed.
        alarm_regs.modify_control(|mut value| {
            value.set_irq_enable(false);
            value.set_enable(false);
            value
        });
        // Enable general interrupts. The IRQ enable of the peripheral remains cleared.
        unsafe {
            enable_nvic_interrupt(AlarmTim::IRQ);
        }
    }

    fn timekeeper_tim() -> MmioTimer<'static> {
        TIMEKEEPER_TIM
            .get()
            .map(|tim| unsafe { tim.steal_regs() })
            .unwrap()
    }
    fn alarm_tim() -> MmioTimer<'static> {
        ALARM_TIM
            .get()
            .map(|tim| unsafe { tim.steal_regs() })
            .unwrap()
    }

    /// Should be called inside the IRQ of the timekeeper timer.
    ///
    /// # Safety
    ///
    /// This function has to be called once by the TIM IRQ used for the timekeeping.
    pub unsafe fn on_interrupt_timekeeping(&self) {
        self.next_period();
    }

    /// Should be called inside the IRQ of the alarm timer.
    ///
    /// # Safety
    ///
    ///This function has to be called once by the TIM IRQ used for the timekeeping.
    pub unsafe fn on_interrupt_alarm(&self) {
        critical_section::with(|cs| {
            if self.alarms.borrow(cs).timestamp.get() <= self.now() {
                self.trigger_alarm(cs)
            }
        })
    }

    fn next_period(&self) {
        let period = self.periods.fetch_add(1, Ordering::AcqRel) + 1;
        let t = (period as u64) << 32;
        critical_section::with(|cs| {
            let alarm = &self.alarms.borrow(cs);
            let at = alarm.timestamp.get();
            if at < t {
                self.trigger_alarm(cs);
            } else {
                let mut alarm_tim = Self::alarm_tim();

                let remaining_ticks = (at - t).checked_mul(*SCALE.get().unwrap());
                if remaining_ticks.is_some_and(|v| v <= u32::MAX as u64) {
                    alarm_tim.write_enable_control(EnableControl::new_disable());
                    alarm_tim.write_count_value(remaining_ticks.unwrap() as u32);
                    alarm_tim.modify_control(|mut value| {
                        value.set_irq_enable(true);
                        value
                    });
                    alarm_tim.write_enable_control(EnableControl::new_enable());
                }
            }
        })
    }

    fn trigger_alarm(&self, cs: CriticalSection) {
        Self::alarm_tim().modify_control(|mut value| {
            value.set_irq_enable(false);
            value.set_enable(false);
            value
        });

        let alarm = &self.alarms.borrow(cs);
        // Setting the maximum value disables the alarm.
        alarm.timestamp.set(u64::MAX);

        // Call after clearing alarm, so the callback can set another alarm.
        let mut next = self
            .queue
            .borrow(cs)
            .borrow_mut()
            .next_expiration(self.now());
        while !self.set_alarm(cs, next) {
            next = self
                .queue
                .borrow(cs)
                .borrow_mut()
                .next_expiration(self.now());
        }
    }

    fn set_alarm(&self, cs: CriticalSection, timestamp: u64) -> bool {
        if SCALE.get().is_none() {
            return false;
        }
        let mut alarm_tim = Self::alarm_tim();
        alarm_tim.modify_control(|mut value| {
            value.set_irq_enable(false);
            value.set_enable(false);
            value
        });

        let alarm = self.alarms.borrow(cs);
        alarm.timestamp.set(timestamp);

        let t = self.now();
        if timestamp <= t {
            alarm.timestamp.set(u64::MAX);
            return false;
        }

        // If it hasn't triggered yet, setup the relevant reset value, regardless of whether
        // the interrupts are enabled or not. When they are enabled at a later point, the
        // right value is already set.

        // If the timestamp is in the next few ticks, add a bit of buffer to be sure the alarm
        // is not missed.
        //
        // This means that an alarm can be delayed for up to 2 ticks (from t+1 to t+3), but this is allowed
        // by the Alarm trait contract. What's not allowed is triggering alarms *before* their scheduled time,
        // and we don't do that here.
        let safe_timestamp = timestamp.max(t + 3);
        let timer_ticks = (safe_timestamp - t).checked_mul(*SCALE.get().unwrap());
        alarm_tim.write_reset_value(u32::MAX);
        if timer_ticks.is_some_and(|v| v <= u32::MAX as u64) {
            alarm_tim.write_count_value(timer_ticks.unwrap() as u32);
            alarm_tim.modify_control(|mut value| {
                value.set_irq_enable(true);
                value.set_enable(true);
                value
            });
        }
        // If it's too far in the future, don't enable timer yet.
        // It will be enabled later by `next_period`.

        true
    }
}

impl Driver for TimerDriver {
    fn now(&self) -> u64 {
        if SCALE.get().is_none() {
            return 0;
        }
        let mut period1: u32;
        let mut period2: u32;
        let mut counter_val: u32;

        loop {
            // Acquire ensures that we get the latest value of `periods` and
            // no instructions can be reordered before the load.
            period1 = self.periods.load(Ordering::Acquire);

            counter_val = u32::MAX - Self::timekeeper_tim().read_count_value();

            // Double read to protect against race conditions when the counter is overflowing.
            period2 = self.periods.load(Ordering::Relaxed);
            if period1 == period2 {
                let now = (((period1 as u64) << 32) | counter_val as u64) / *SCALE.get().unwrap();
                return now;
            }
        }
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();

            if queue.schedule_wake(at, waker) {
                let mut next = queue.next_expiration(self.now());
                while !self.set_alarm(cs, next) {
                    next = queue.next_expiration(self.now());
                }
            }
        })
    }
}
