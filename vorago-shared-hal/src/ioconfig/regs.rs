use core::marker::PhantomData;

use crate::{NUM_PORT_A, NUM_PORT_B, gpio::DynPinId};
#[cfg(feature = "vor4x")]
use crate::{NUM_PORT_DEFAULT, NUM_PORT_G};

#[cfg(feature = "vor1x")]
pub const BASE_ADDR: usize = 0x4000_2000;
#[cfg(feature = "vor4x")]
pub const BASE_ADDR: usize = 0x4001_1000;

#[bitbybit::bitenum(u3)]
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterType {
    SysClk = 0,
    DirectInput = 1,
    FilterOneCycle = 2,
    FilterTwoCycles = 3,
    FilterThreeCycles = 4,
    FilterFourCycles = 5,
}

#[derive(Debug, PartialEq, Eq)]
#[bitbybit::bitenum(u3, exhaustive = true)]
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

#[derive(Debug, PartialEq, Eq)]
#[bitbybit::bitenum(u1, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pull {
    Up = 0,
    Down = 1,
}

#[derive(Debug, Eq, PartialEq)]
#[bitbybit::bitenum(u2, exhaustive = true)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FunctionSelect {
    Sel0 = 0b00,
    Sel1 = 0b01,
    Sel2 = 0b10,
    Sel3 = 0b11,
}

#[bitbybit::bitfield(u32, debug, defmt_fields(feature = "defmt"))]
pub struct Config {
    #[bit(16, rw)]
    io_disable: bool,
    #[bits(13..=14, rw)]
    funsel: FunctionSelect,
    #[bit(12, rw)]
    pull_when_output_active: bool,
    #[bit(11, rw)]
    pull_enable: bool,
    #[bit(10, rw)]
    pull_dir: Pull,
    #[bit(9, rw)]
    invert_output: bool,
    #[bit(8, rw)]
    open_drain: bool,
    /// IEWO bit. Allows monitoring of output values.
    #[bit(7, rw)]
    input_enable_when_output: bool,
    #[bit(6, rw)]
    invert_input: bool,
    #[bits(3..=5, rw)]
    filter_clk_sel: FilterClockSelect,
    #[bits(0..=2, rw)]
    filter_type: Option<FilterType>,
}

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
    pub const fn new_mmio() -> MmioIoConfig<'static> {
        MmioIoConfig {
            ptr: BASE_ADDR as *mut _,
            phantom: PhantomData,
        }
    }
}

impl MmioIoConfig<'_> {
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

    pub fn modify_pin_config<F: FnOnce(Config) -> Config>(&mut self, id: DynPinId, f: F) {
        let config = self.read_pin_config(id);
        self.write_pin_config(id, f(config))
    }

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
