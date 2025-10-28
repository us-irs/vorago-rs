use core::marker::PhantomData;

use arbitrary_int::{prelude::*, u7};

#[cfg(feature = "vor1x")]
const BASE_ADDR: usize = 0x4002_0000;
#[cfg(feature = "vor4x")]
const BASE_ADDR: usize = 0x4001_8000;

#[bitbybit::bitenum(u3)]
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusSelect {
    /// Pulse when timer reaches 0.
    OneCyclePulse = 0b000,
    OutputActiveBit = 0b001,
    /// Creates a divide by two output clock of the timer.
    ToggleOnEachCycle = 0b010,
    /// 1 when count value >= PWM A value, 0 otherwise
    PwmaOutput = 0b011,
    /// 1 when count value < PWM A value and >= PWM B, 0 when counter value >= PWM A value or < PWM
    /// B value
    PwmbOutput = 0b100,
    EnabledBit = 0b101,
    /// 1 when counter value <= PWM A value and 0 otherwise.
    PwmaActiveBit = 0b110,
}

#[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
pub struct Control {
    /// The counter is requested to stop on the next normal count cycle.
    #[bit(9, rw)]
    request_stop: bool,
    #[bit(8, rw)]
    status_invert: bool,
    #[bits(5..=7, rw)]
    status_sel: Option<StatusSelect>,
    #[bit(4, rw)]
    irq_enable: bool,
    /// Only applies if the Auto-Disable bit is 0. The ACTIVE bit goes to 0 when the count reaches
    /// 0, but the timer remains enabled.
    #[bit(3, rw)]
    auto_deactivate: bool,
    /// Counter is fully disabled when count reaches 0, which means that both the ENABLE
    /// and ACTIVE bits go to 0.
    #[bit(2, rw)]
    auto_disable: bool,
    #[bit(1, r)]
    active: bool,
    #[bit(0, rw)]
    enable: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EnableControl(arbitrary_int::UInt<u32, 1>);

impl EnableControl {
    pub fn new_disable() -> Self {
        EnableControl(arbitrary_int::UInt::<u32, 1>::from_u32(0))
    }

    pub fn new_enable() -> Self {
        EnableControl(arbitrary_int::UInt::<u32, 1>::from_u32(1))
    }

    pub fn enabled(&self) -> bool {
        self.0.value() != 0
    }
}

#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CascadeInvert {
    #[default]
    ActiveHigh = 0,
    ActiveLow = 1,
}

/// When two cascade sources are selected, configure the required operation.
#[bitbybit::bitenum(u1, exhaustive = true)]
#[derive(Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DualCascadeOp {
    #[default]
    LogicalAnd = 0,
    LogicalOr = 1,
}

#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_fields(feature = "defmt"))]
pub struct CascadeControl {
    /// The counter is automatically disabled if the corresponding Cascade 2 level-sensitive input
    /// souce is active when the count reaches 0. If the counter is not 0, the cascade control is
    /// ignored.
    #[bit(10, rw)]
    trigger2: bool,
    #[bit(9, rw)]
    inv2: CascadeInvert,
    /// Enable Cascade 2 signal active as a requirement to stop counting. This mode is similar
    /// to the REQ_STOP control bit, but signalled by a Cascade source.
    #[bit(8, rw)]
    en2: bool,
    /// Same as the trigger field for Cascade 0.
    #[bit(7, rw)]
    trigger1: bool,
    /// Enable trigger mode for Cascade 0. In trigger mode, couting will start with the selected
    /// cascade signal active, but once the counter is active, cascade control will be ignored.
    #[bit(6, rw)]
    trigger0: bool,
    /// Specify required operation if both Cascade 0 and Cascade 1 are active.
    /// 0 is a logical AND of both cascade signals, 1 is a logical OR.
    #[bit(4, rw)]
    dual_cascade_op: DualCascadeOp,
    /// Inversion bit for Cascade 1
    #[bit(3, rw)]
    inv1: CascadeInvert,
    /// Enable Cascade 1 signal active as a requirement for counting.
    #[bit(2, rw)]
    en1: bool,
    /// Inversion bit for Cascade 0.
    #[bit(1, rw)]
    inv0: CascadeInvert,
    /// Enable Cascade 0 signal active as a requirement for counting.
    #[bit(0, rw)]
    en0: bool,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidCascadeSourceId;

#[cfg(feature = "vor1x")]
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CascadeSource {
    PortA(u8),
    PortB(u8),
    Tim(u8),
    RamSbe = 96,
    RamMbe = 97,
    RomSbe = 98,
    RomMbe = 99,
    Txev = 100,
    ClockDivider(u8),
}

#[cfg(feature = "vor4x")]
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CascadeSource {
    PortA(u8),
    PortB(u8),
    PortC(u8),
    PortD(u8),
    PortE(u8),
    Tim(u8),
    TxEv,
    AdcIrq,
    RomSbe,
    RomMbe,
    Ram0Sbe,
    Ram0Mbe,
    Ram1Sbe,
    Ram1Mbe,
    WdogIrq,
}

impl CascadeSource {
    #[cfg(feature = "vor1x")]
    pub fn id(&self) -> Result<u7, InvalidCascadeSourceId> {
        let port_check = |base: u8, id: u8, len: u8| -> Result<u7, InvalidCascadeSourceId> {
            if id > len - 1 {
                return Err(InvalidCascadeSourceId);
            }
            Ok(u7::new(base + id))
        };
        match self {
            CascadeSource::PortA(id) => port_check(0, *id, 32),
            CascadeSource::PortB(id) => port_check(32, *id, 32),
            CascadeSource::Tim(id) => port_check(64, *id, 24),
            CascadeSource::RamSbe => Ok(u7::new(96)),
            CascadeSource::RamMbe => Ok(u7::new(97)),
            CascadeSource::RomSbe => Ok(u7::new(98)),
            CascadeSource::RomMbe => Ok(u7::new(99)),
            CascadeSource::Txev => Ok(u7::new(100)),
            CascadeSource::ClockDivider(id) => port_check(120, *id, 8),
        }
    }

    #[cfg(feature = "vor4x")]
    fn id(&self) -> Result<u7, InvalidCascadeSourceId> {
        let port_check = |base: u8, id: u8| -> Result<u7, InvalidCascadeSourceId> {
            if id > 15 {
                return Err(InvalidCascadeSourceId);
            }
            Ok(u7::new(base + id))
        };
        match self {
            CascadeSource::PortA(id) => port_check(0, *id),
            CascadeSource::PortB(id) => port_check(16, *id),
            CascadeSource::PortC(id) => port_check(32, *id),
            CascadeSource::PortD(id) => port_check(48, *id),
            CascadeSource::PortE(id) => port_check(64, *id),
            CascadeSource::Tim(id) => {
                if *id > 23 {
                    return Err(InvalidCascadeSourceId);
                }
                Ok(u7::new(80 + id))
            }
            CascadeSource::TxEv => Ok(u7::new(104)),
            CascadeSource::AdcIrq => Ok(u7::new(105)),
            CascadeSource::RomSbe => Ok(u7::new(106)),
            CascadeSource::RomMbe => Ok(u7::new(106)),
            CascadeSource::Ram0Sbe => Ok(u7::new(108)),
            CascadeSource::Ram0Mbe => Ok(u7::new(109)),
            CascadeSource::Ram1Sbe => Ok(u7::new(110)),
            CascadeSource::Ram1Mbe => Ok(u7::new(111)),
            CascadeSource::WdogIrq => Ok(u7::new(112)),
        }
    }

    #[cfg(feature = "vor1x")]
    pub fn from_raw(raw: u32) -> Result<Self, InvalidCascadeSourceId> {
        let id = u7::new((raw & 0x7F) as u8);
        if id.value() > 127 {
            return Err(InvalidCascadeSourceId);
        }
        let id = id.as_u8();
        if id < 32 {
            return Ok(CascadeSource::PortA(id));
        } else if (32..56).contains(&id) {
            return Ok(CascadeSource::PortB(id - 32));
        } else if (64..88).contains(&id) {
            return Ok(CascadeSource::Tim(id - 64));
        } else if id > 120 {
            return Ok(CascadeSource::ClockDivider(id - 120));
        }
        match id {
            96 => Ok(CascadeSource::RamSbe),
            97 => Ok(CascadeSource::RamMbe),
            98 => Ok(CascadeSource::RomSbe),
            99 => Ok(CascadeSource::RomMbe),
            100 => Ok(CascadeSource::Txev),
            _ => Err(InvalidCascadeSourceId),
        }
    }
    #[cfg(feature = "vor4x")]
    pub fn from_raw(raw: u32) -> Result<Self, InvalidCascadeSourceId> {
        use crate::NUM_PORT_DEFAULT;

        let id = u7::new((raw & 0x7F) as u8);
        if id.value() > 127 {
            return Err(InvalidCascadeSourceId);
        }
        let id = id.as_u8();
        if id < 16 {
            return Ok(CascadeSource::PortA(id));
        } else if (16..16 + NUM_PORT_DEFAULT as u8).contains(&id) {
            return Ok(CascadeSource::PortB(id - 16));
        } else if (32..32 + NUM_PORT_DEFAULT as u8).contains(&id) {
            return Ok(CascadeSource::PortC(id - 32));
        } else if (48..48 + NUM_PORT_DEFAULT as u8).contains(&id) {
            return Ok(CascadeSource::PortD(id - 48));
        } else if (64..64 + NUM_PORT_DEFAULT as u8).contains(&id) {
            return Ok(CascadeSource::PortE(id - 64));
        } else if (80..104).contains(&id) {
            return Ok(CascadeSource::Tim(id - 80));
        }
        match id {
            104 => Ok(CascadeSource::TxEv),
            105 => Ok(CascadeSource::AdcIrq),
            106 => Ok(CascadeSource::RomSbe),
            107 => Ok(CascadeSource::RomMbe),
            108 => Ok(CascadeSource::Ram0Sbe),
            109 => Ok(CascadeSource::Ram0Mbe),
            110 => Ok(CascadeSource::Ram1Sbe),
            111 => Ok(CascadeSource::Ram1Mbe),
            112 => Ok(CascadeSource::WdogIrq),
            _ => Err(InvalidCascadeSourceId),
        }
    }
}

#[bitbybit::bitfield(u32)]
pub struct CascadeSourceReg {
    #[bits(0..=6, rw)]
    raw: u7,
}

impl CascadeSourceReg {
    pub fn new(source: CascadeSource) -> Result<Self, InvalidCascadeSourceId> {
        let id = source.id()?;
        Ok(Self::new_with_raw_value(id.as_u32()))
    }

    pub fn as_cascade_source(&self) -> Result<CascadeSource, InvalidCascadeSourceId> {
        CascadeSource::from_raw(self.raw().as_u32())
    }
}

#[derive(derive_mmio::Mmio)]
#[mmio(no_ctors)]
#[repr(C)]
pub struct Timer {
    control: Control,
    reset_value: u32,
    count_value: u32,
    enable_control: EnableControl,
    cascade_control: CascadeControl,
    /// CASCADE0 and CASCADE1 are used to control the counting and activation of the counter.
    /// CASCADE2 is used to request stopping of the timer.
    cascade: [CascadeSourceReg; 3],
    /// PWM A compare value.
    pwma_value: u32,
    /// PWM B compare value.
    pwmb_value: u32,
    #[cfg(feature = "vor1x")]
    _reserved: [u32; 0x3f5],
    #[cfg(feature = "vor4x")]
    _reserved: [u32; 0xf5],
    /// Vorago 1x: 0x0111_07E1. Vorago 4x: 0x0211_07E9
    perid: u32,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "vor1x")] {
        static_assertions::const_assert_eq!(core::mem::size_of::<Timer>(), 0x1000);
    } else if #[cfg(feature = "vor4x")] {
        static_assertions::const_assert_eq!(core::mem::size_of::<Timer>(), 0x400);
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidTimerIndex(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TimId(u8);

impl TimId {
    pub const fn new(index: usize) -> Result<Self, InvalidTimerIndex> {
        if index > 23 {
            return Err(InvalidTimerIndex(index));
        }
        Ok(TimId(index as u8))
    }

    pub const fn new_unchecked(index: usize) -> Self {
        if index > 23 {
            panic!("invalid timer index");
        }
        TimId(index as u8)
    }

    /// Unsafely steal the TIM peripheral block for the TIM ID.
    ///
    /// # Safety
    ///
    /// Circumvents ownership and safety guarantees by the HAL.
    pub const unsafe fn steal_regs(&self) -> MmioTimer<'static> {
        Timer::new_mmio(*self)
    }

    pub const fn value(&self) -> u8 {
        self.0
    }

    #[cfg(feature = "vor4x")]
    pub const fn interrupt_id(&self) -> va416xx::Interrupt {
        match self.value() {
            0 => va416xx::Interrupt::TIM0,
            1 => va416xx::Interrupt::TIM1,
            2 => va416xx::Interrupt::TIM2,
            3 => va416xx::Interrupt::TIM3,
            4 => va416xx::Interrupt::TIM4,
            5 => va416xx::Interrupt::TIM5,
            6 => va416xx::Interrupt::TIM6,
            7 => va416xx::Interrupt::TIM7,
            8 => va416xx::Interrupt::TIM8,
            9 => va416xx::Interrupt::TIM9,
            10 => va416xx::Interrupt::TIM10,
            11 => va416xx::Interrupt::TIM11,
            12 => va416xx::Interrupt::TIM12,
            13 => va416xx::Interrupt::TIM13,
            14 => va416xx::Interrupt::TIM14,
            15 => va416xx::Interrupt::TIM15,
            16 => va416xx::Interrupt::TIM16,
            17 => va416xx::Interrupt::TIM17,
            18 => va416xx::Interrupt::TIM18,
            19 => va416xx::Interrupt::TIM19,
            20 => va416xx::Interrupt::TIM20,
            21 => va416xx::Interrupt::TIM21,
            22 => va416xx::Interrupt::TIM22,
            23 => va416xx::Interrupt::TIM23,
            _ => unreachable!(),
        }
    }
}

impl Timer {
    const fn new_mmio_at(base: usize) -> MmioTimer<'static> {
        MmioTimer {
            ptr: base as *mut _,
            phantom: PhantomData,
        }
    }

    pub const fn new_mmio(id: TimId) -> MmioTimer<'static> {
        if cfg!(feature = "vor1x") {
            Timer::new_mmio_at(BASE_ADDR + 0x1000 * id.value() as usize)
        } else {
            Timer::new_mmio_at(BASE_ADDR + 0x400 * id.value() as usize)
        }
    }
    pub fn new_mmio_with_raw_index(
        timer_index: usize,
    ) -> Result<MmioTimer<'static>, InvalidTimerIndex> {
        if timer_index > 23 {
            return Err(InvalidTimerIndex(timer_index));
        }
        if cfg!(feature = "vor1x") {
            Ok(Timer::new_mmio_at(BASE_ADDR + 0x1000 * timer_index))
        } else {
            Ok(Timer::new_mmio_at(BASE_ADDR + 0x400 * timer_index))
        }
    }
}
