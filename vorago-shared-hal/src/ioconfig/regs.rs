use core::marker::PhantomData;

use crate::{NUM_PORT_A, NUM_PORT_B, gpio::DynPinId};
#[cfg(feature = "vor4x")]
use crate::{NUM_PORT_DEFAULT, NUM_PORT_G};

/// IOCONFIG base address.
#[cfg(feature = "vor1x")]
pub const BASE_ADDR: usize = 0x4000_2000;
/// IOCONFIG base address.
#[cfg(feature = "vor4x")]
pub const BASE_ADDR: usize = 0x4001_1000;

/// Input filter type for a pin, see [Config::filter_type].
#[bitbybit::bitenum(u3)]
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterType {
    /// Filter clocked by the system clock.
    SysClk = 0,
    /// No filtering, the input is passed through directly.
    DirectInput = 1,
    /// Filter requiring the input to be stable for one filter clock cycle.
    FilterOneCycle = 2,
    /// Filter requiring the input to be stable for two filter clock cycles.
    FilterTwoCycles = 3,
    /// Filter requiring the input to be stable for three filter clock cycles.
    FilterThreeCycles = 4,
    /// Filter requiring the input to be stable for four filter clock cycles.
    FilterFourCycles = 5,
}

/// Input filter clock source for a pin, see [Config::filter_clk_sel].
#[derive(Debug, PartialEq, Eq)]
#[bitbybit::bitenum(u3, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterClockSelect {
    /// The system clock.
    SysClk = 0,
    /// Filter clock 1.
    Clk1 = 1,
    /// Filter clock 2.
    Clk2 = 2,
    /// Filter clock 3.
    Clk3 = 3,
    /// Filter clock 4.
    Clk4 = 4,
    /// Filter clock 5.
    Clk5 = 5,
    /// Filter clock 6.
    Clk6 = 6,
    /// Filter clock 7.
    Clk7 = 7,
}

/// Pull resistor direction.
#[derive(Debug, PartialEq, Eq)]
#[bitbybit::bitenum(u1, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pull {
    /// Pull-down resistor.
    Down = 0,
    /// Pull-up resistor.
    Up = 1,
}

/// Pin alternate function select.
#[derive(Debug, Eq, PartialEq)]
#[bitbybit::bitenum(u2, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FunctionSelect {
    /// Function 0, always GPIO.
    Sel0 = 0b00,
    /// Function 1.
    Sel1 = 0b01,
    /// Function 2.
    Sel2 = 0b10,
    /// Function 3.
    Sel3 = 0b11,
}

/// Per-pin IOCONFIG register.
#[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
pub struct Config {
    /// Disable the pin entirely.
    #[bit(16, rw)]
    io_disable: bool,
    /// Alternate function select.
    #[bits(13..=14, rw)]
    funsel: FunctionSelect,
    /// Only apply the pull resistor while the pin drives an output.
    #[bit(12, rw)]
    pull_when_output_active: bool,
    /// Enable the pull resistor.
    #[bit(11, rw)]
    pull_enable: bool,
    /// Pull resistor direction.
    #[bit(10, rw)]
    pull_dir: Pull,
    /// Invert the output value.
    #[bit(9, rw)]
    invert_output: bool,
    /// Configure the pin as open-drain output.
    #[bit(8, rw)]
    open_drain: bool,
    /// IEWO bit. Allows monitoring of output values.
    #[bit(7, rw)]
    input_enable_when_output: bool,
    /// Invert the input value.
    #[bit(6, rw)]
    invert_input: bool,
    /// Input filter clock source.
    #[bits(3..=5, rw)]
    filter_clk_sel: FilterClockSelect,
    /// Input filter type.
    #[bits(0..=2, rw)]
    filter_type: Option<FilterType>,
}

/// IOCONFIG peripheral register block.
#[derive(derive_mmio::Mmio)]
#[mmio(no_ctors)]
#[repr(C)]
pub struct IoConfig {
    port_a: [Config; NUM_PORT_A],
    port_b: [Config; NUM_PORT_B],
    #[cfg(feature = "vor4x")]
    port_c: [Config; NUM_PORT_DEFAULT],
    #[cfg(feature = "vor4x")]
    port_d: [Config; NUM_PORT_DEFAULT],
    #[cfg(feature = "vor4x")]
    port_e: [Config; NUM_PORT_DEFAULT],
    #[cfg(feature = "vor4x")]
    port_f: [Config; NUM_PORT_DEFAULT],
    #[cfg(feature = "vor4x")]
    port_g: [Config; NUM_PORT_G],
    #[cfg(feature = "vor4x")]
    _reserved0: [u32; 0x8],
    #[cfg(feature = "vor4x")]
    #[mmio(PureRead)]
    clk_div_0: u32,
    #[cfg(feature = "vor4x")]
    clk_div_1: u32,
    #[cfg(feature = "vor4x")]
    clk_div_2: u32,
    #[cfg(feature = "vor4x")]
    clk_div_3: u32,
    #[cfg(feature = "vor4x")]
    clk_div_4: u32,
    #[cfg(feature = "vor4x")]
    clk_div_5: u32,
    #[cfg(feature = "vor4x")]
    clk_div_6: u32,
    #[cfg(feature = "vor4x")]
    clk_div_7: u32,
    #[cfg(feature = "vor4x")]
    _reserved1: [u32; 0x387],
    #[cfg(feature = "vor1x")]
    _reserved1: [u32; 0x3c7],
    #[mmio(PureRead)]
    /// Reset value: 0x0282_07E9 for Vorago 4x, and 0x0182_07E1 for Vorago 1x
    perid: u32,
}

static_assertions::const_assert_eq!(core::mem::size_of::<IoConfig>(), 0x1000);

impl IoConfig {
    /// Get an MMIO accessor for the IOCONFIG register block.
    pub const fn new_mmio() -> MmioIoConfig<'static> {
        MmioIoConfig {
            ptr: BASE_ADDR as *mut _,
            phantom: PhantomData,
        }
    }
}

impl MmioIoConfig<'_> {
    /// Read the IOCONFIG register for the given pin.
    pub fn read_pin_config(&self, id: DynPinId) -> Config {
        let offset = id.offset();
        match id.port() {
            crate::Port::A => unsafe { self.read_port_a_unchecked(offset) },
            crate::Port::B => unsafe { self.read_port_b_unchecked(offset) },
            #[cfg(feature = "vor4x")]
            crate::Port::C => unsafe { self.read_port_c_unchecked(offset) },
            #[cfg(feature = "vor4x")]
            crate::Port::D => unsafe { self.read_port_d_unchecked(offset) },
            #[cfg(feature = "vor4x")]
            crate::Port::E => unsafe { self.read_port_e_unchecked(offset) },
            #[cfg(feature = "vor4x")]
            crate::Port::F => unsafe { self.read_port_f_unchecked(offset) },
            #[cfg(feature = "vor4x")]
            crate::Port::G => unsafe { self.read_port_g_unchecked(offset) },
        }
    }

    /// Read-modify-write the IOCONFIG register for the given pin.
    pub fn modify_pin_config<F: FnOnce(Config) -> Config>(&mut self, id: DynPinId, f: F) {
        let config = self.read_pin_config(id);
        self.write_pin_config(id, f(config))
    }

    /// Write the IOCONFIG register for the given pin.
    pub fn write_pin_config(&mut self, id: DynPinId, config: Config) {
        let offset = id.offset();
        match id.port() {
            crate::Port::A => unsafe { self.write_port_a_unchecked(offset, config) },
            crate::Port::B => unsafe { self.write_port_b_unchecked(offset, config) },
            #[cfg(feature = "vor4x")]
            crate::Port::C => unsafe { self.write_port_c_unchecked(offset, config) },
            #[cfg(feature = "vor4x")]
            crate::Port::D => unsafe { self.write_port_d_unchecked(offset, config) },
            #[cfg(feature = "vor4x")]
            crate::Port::E => unsafe { self.write_port_e_unchecked(offset, config) },
            #[cfg(feature = "vor4x")]
            crate::Port::F => unsafe { self.write_port_f_unchecked(offset, config) },
            #[cfg(feature = "vor4x")]
            crate::Port::G => unsafe { self.write_port_g_unchecked(offset, config) },
        }
    }
}
