use crate::sysconfig::reset_peripheral_for_cycles;

pub use crate::gpio::{DynPinId, Port};

use crate::PeripheralSelect;
use crate::sealed::Sealed;
#[cfg(feature = "vor1x")]
use va108xx as pac;
#[cfg(feature = "vor4x")]
use va416xx as pac;

/// Trait implemented by data structures associated with pin identification.
pub trait PinId {
    const ID: crate::gpio::ll::DynPinId;
}

pub trait AnyPin: Sealed {
    const ID: DynPinId;
}

/// Primary Pin structure for the physical pins exposed by Vorago MCUs.
///
/// This pin structure is only used for resource management and does not do anything on its
/// own.
pub struct Pin<Id: PinId> {
    phantom: core::marker::PhantomData<Id>,
}

impl<Id: PinId + Sealed> AnyPin for Pin<Id> {
    const ID: DynPinId = Id::ID;
}

impl<I: PinId> Pin<I> {
    #[allow(clippy::new_without_default)]
    #[doc(hidden)]
    pub const fn __new() -> Self {
        Self {
            phantom: core::marker::PhantomData,
        }
    }

    /// Create a new pin instance.
    ///
    /// # Safety
    ///
    /// This circumvents ownership rules of the HAL and allows creating multiple instances
    /// of the same pin.
    pub const unsafe fn steal() -> Self {
        Self::__new()
    }
}

macro_rules! pin_id {
    ($Id:ident, $Port:path, $num:literal) => {
        // Need paste macro to use ident in doc attribute
        paste::paste! {
            #[doc = "Pin ID representing pin " $Id]
            #[derive(Debug)]
            pub enum $Id {}

            impl $crate::sealed::Sealed for $Id {}
            impl PinId for $Id {
                const ID: DynPinId = DynPinId::new_unchecked($Port, $num);
            }
        }
    };
}

impl<I: PinId + Sealed> Sealed for Pin<I> {}

pin_id!(Pa0, Port::A, 0);
pin_id!(Pa1, Port::A, 1);
pin_id!(Pa2, Port::A, 2);
pin_id!(Pa3, Port::A, 3);
pin_id!(Pa4, Port::A, 4);
pin_id!(Pa5, Port::A, 5);
pin_id!(Pa6, Port::A, 6);
pin_id!(Pa7, Port::A, 7);
pin_id!(Pa8, Port::A, 8);
pin_id!(Pa9, Port::A, 9);
pin_id!(Pa10, Port::A, 10);
pin_id!(Pa11, Port::A, 11);
pin_id!(Pa12, Port::A, 12);
pin_id!(Pa13, Port::A, 13);
pin_id!(Pa14, Port::A, 14);
pin_id!(Pa15, Port::A, 15);
#[cfg(feature = "vor1x")]
pin_id!(Pa16, Port::A, 16);
#[cfg(feature = "vor1x")]
pin_id!(Pa17, Port::A, 17);
#[cfg(feature = "vor1x")]
pin_id!(Pa18, Port::A, 18);
#[cfg(feature = "vor1x")]
pin_id!(Pa19, Port::A, 19);
#[cfg(feature = "vor1x")]
pin_id!(Pa20, Port::A, 20);
#[cfg(feature = "vor1x")]
pin_id!(Pa21, Port::A, 21);
#[cfg(feature = "vor1x")]
pin_id!(Pa22, Port::A, 22);
#[cfg(feature = "vor1x")]
pin_id!(Pa23, Port::A, 23);
#[cfg(feature = "vor1x")]
pin_id!(Pa24, Port::A, 24);
#[cfg(feature = "vor1x")]
pin_id!(Pa25, Port::A, 25);
#[cfg(feature = "vor1x")]
pin_id!(Pa26, Port::A, 26);
#[cfg(feature = "vor1x")]
#[cfg(feature = "vor1x")]
pin_id!(Pa27, Port::A, 27);
#[cfg(feature = "vor1x")]
pin_id!(Pa28, Port::A, 28);
#[cfg(feature = "vor1x")]
pin_id!(Pa29, Port::A, 29);
#[cfg(feature = "vor1x")]
pin_id!(Pa30, Port::A, 30);
#[cfg(feature = "vor1x")]
pin_id!(Pa31, Port::A, 31);

pin_id!(Pb0, Port::B, 0);
pin_id!(Pb1, Port::B, 1);
pin_id!(Pb2, Port::B, 2);
pin_id!(Pb3, Port::B, 3);
pin_id!(Pb4, Port::B, 4);
#[cfg(not(feature = "va41628"))]
pin_id!(Pb5, Port::B, 5);
#[cfg(not(feature = "va41628"))]
pin_id!(Pb6, Port::B, 6);
#[cfg(not(feature = "va41628"))]
pin_id!(Pb7, Port::B, 7);
#[cfg(not(feature = "va41628"))]
pin_id!(Pb8, Port::B, 8);
#[cfg(not(feature = "va41628"))]
pin_id!(Pb9, Port::B, 9);
#[cfg(not(feature = "va41628"))]
pin_id!(Pb10, Port::B, 10);
#[cfg(not(feature = "va41628"))]
pin_id!(Pb11, Port::B, 11);
pin_id!(Pb12, Port::B, 12);
pin_id!(Pb13, Port::B, 13);
pin_id!(Pb14, Port::B, 14);
pin_id!(Pb15, Port::B, 15);
#[cfg(feature = "vor1x")]
pin_id!(Pb16, Port::B, 16);
#[cfg(feature = "vor1x")]
pin_id!(Pb17, Port::B, 17);
#[cfg(feature = "vor1x")]
pin_id!(Pb18, Port::B, 18);
#[cfg(feature = "vor1x")]
pin_id!(Pb19, Port::B, 19);
#[cfg(feature = "vor1x")]
pin_id!(Pb20, Port::B, 20);
#[cfg(feature = "vor1x")]
pin_id!(Pb21, Port::B, 21);
#[cfg(feature = "vor1x")]
pin_id!(Pb22, Port::B, 22);
#[cfg(feature = "vor1x")]
pin_id!(Pb23, Port::B, 23);

cfg_if::cfg_if! {
    if #[cfg(feature = "vor4x")] {
        pin_id!(Pc0, Port::C, 0);
        pin_id!(Pc1, Port::C, 1);
        pin_id!(Pc2, Port::C, 2);
        pin_id!(Pc3, Port::C, 3);
        pin_id!(Pc4, Port::C, 4);
        pin_id!(Pc5, Port::C, 5);
        pin_id!(Pc6, Port::C, 6);
        pin_id!(Pc7, Port::C, 7);
        pin_id!(Pc8, Port::C, 8);
        pin_id!(Pc9, Port::C, 9);
        pin_id!(Pc10, Port::C, 10);
        pin_id!(Pc11, Port::C, 11);
        pin_id!(Pc12, Port::C, 12);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pc13, Port::C, 13);
        pin_id!(Pc14, Port::C, 14);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pc15, Port::C, 15);

        #[cfg(not(feature = "va41628"))]
        pin_id!(Pd0, Port::D, 0);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pd1, Port::D, 1);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pd2, Port::D, 2);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pd3, Port::D, 3);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pd4, Port::D, 4);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pd5, Port::D, 5);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pd6, Port::D, 6);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pd7, Port::D, 7);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pd8, Port::D, 8);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pd9, Port::D, 9);
        pin_id!(Pd10, Port::D, 10);
        pin_id!(Pd11, Port::D, 11);
        pin_id!(Pd12, Port::D, 12);
        pin_id!(Pd13, Port::D, 13);
        pin_id!(Pd14, Port::D, 14);
        pin_id!(Pd15, Port::D, 15);

        pin_id!(Pe0, Port::E, 0);
        pin_id!(Pe1, Port::E, 1);
        pin_id!(Pe2, Port::E, 2);
        pin_id!(Pe3, Port::E, 3);
        pin_id!(Pe4, Port::E, 4);
        pin_id!(Pe5, Port::E, 5);
        pin_id!(Pe6, Port::E, 6);
        pin_id!(Pe7, Port::E, 7);
        pin_id!(Pe8, Port::E, 8);
        pin_id!(Pe9, Port::E, 9);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pe10, Port::E, 10);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pe11, Port::E, 11);
        pin_id!(Pe12, Port::E, 12);
        pin_id!(Pe13, Port::E, 13);
        pin_id!(Pe14, Port::E, 14);
        pin_id!(Pe15, Port::E, 15);

        pin_id!(Pf0, Port::F, 0);
        pin_id!(Pf1, Port::F, 1);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pf2, Port::F, 2);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pf3, Port::F, 3);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pf4, Port::F, 4);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pf5, Port::F, 5);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pf6, Port::F, 6);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pf7, Port::F, 7);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pf8, Port::F, 8);
        pin_id!(Pf9, Port::F, 9);
        #[cfg(not(feature = "va41628"))]
        pin_id!(Pf10, Port::F, 10);
        pin_id!(Pf11, Port::F, 11);
        pin_id!(Pf12, Port::F, 12);
        pin_id!(Pf13, Port::F, 13);
        pin_id!(Pf14, Port::F, 14);
        pin_id!(Pf15, Port::F, 15);

        pin_id!(Pg0, Port::G, 0);
        pin_id!(Pg1, Port::G, 1);
        pin_id!(Pg2, Port::G, 2);
        pin_id!(Pg3, Port::G, 3);
        pin_id!(Pg4, Port::G, 4);
        pin_id!(Pg5, Port::G, 5);
        pin_id!(Pg6, Port::G, 6);
        pin_id!(Pg7, Port::G, 7);
    }
}

/// Resource management singleton for GPIO PORT A.
pub struct PinsA {
    pub pa0: Pin<Pa0>,
    pub pa1: Pin<Pa1>,
    pub pa2: Pin<Pa2>,
    pub pa3: Pin<Pa3>,
    pub pa4: Pin<Pa4>,
    pub pa5: Pin<Pa5>,
    pub pa6: Pin<Pa6>,
    pub pa7: Pin<Pa7>,
    pub pa8: Pin<Pa8>,
    pub pa9: Pin<Pa9>,
    pub pa10: Pin<Pa10>,
    pub pa11: Pin<Pa11>,
    pub pa12: Pin<Pa12>,
    pub pa13: Pin<Pa13>,
    pub pa14: Pin<Pa14>,
    pub pa15: Pin<Pa15>,
    #[cfg(feature = "vor1x")]
    pub pa16: Pin<Pa16>,
    #[cfg(feature = "vor1x")]
    pub pa17: Pin<Pa17>,
    #[cfg(feature = "vor1x")]
    pub pa18: Pin<Pa18>,
    #[cfg(feature = "vor1x")]
    pub pa19: Pin<Pa19>,
    #[cfg(feature = "vor1x")]
    pub pa20: Pin<Pa20>,
    #[cfg(feature = "vor1x")]
    pub pa21: Pin<Pa21>,
    #[cfg(feature = "vor1x")]
    pub pa22: Pin<Pa22>,
    #[cfg(feature = "vor1x")]
    pub pa23: Pin<Pa23>,
    #[cfg(feature = "vor1x")]
    pub pa24: Pin<Pa24>,
    #[cfg(feature = "vor1x")]
    pub pa25: Pin<Pa25>,
    #[cfg(feature = "vor1x")]
    pub pa26: Pin<Pa26>,
    #[cfg(feature = "vor1x")]
    pub pa27: Pin<Pa27>,
    #[cfg(feature = "vor1x")]
    pub pa28: Pin<Pa28>,
    #[cfg(feature = "vor1x")]
    pub pa29: Pin<Pa29>,
    #[cfg(feature = "vor1x")]
    pub pa30: Pin<Pa30>,
    #[cfg(feature = "vor1x")]
    pub pa31: Pin<Pa31>,
}

impl PinsA {
    pub fn new(_port_a: pac::Porta) -> Self {
        let syscfg = unsafe { pac::Sysconfig::steal() };
        reset_peripheral_for_cycles(PeripheralSelect::PortA, 2);
        syscfg.peripheral_clk_enable().modify(|_, w| {
            w.porta().set_bit();
            #[cfg(feature = "vor1x")]
            w.gpio().set_bit();
            w.ioconfig().set_bit()
        });
        Self {
            pa0: Pin::__new(),
            pa1: Pin::__new(),
            pa2: Pin::__new(),
            pa3: Pin::__new(),
            pa4: Pin::__new(),
            pa5: Pin::__new(),
            pa6: Pin::__new(),
            pa7: Pin::__new(),
            pa8: Pin::__new(),
            pa9: Pin::__new(),
            pa10: Pin::__new(),
            pa11: Pin::__new(),
            pa12: Pin::__new(),
            pa13: Pin::__new(),
            pa14: Pin::__new(),
            pa15: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa16: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa17: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa18: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa19: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa20: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa21: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa22: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa23: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa24: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa25: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa26: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa27: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa28: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa29: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa30: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pa31: Pin::__new(),
        }
    }
}

/// Resource management singleton for GPIO PORT B.
pub struct PinsB {
    pub pb0: Pin<Pb0>,
    pub pb1: Pin<Pb1>,
    pub pb2: Pin<Pb2>,
    pub pb3: Pin<Pb3>,
    pub pb4: Pin<Pb4>,
    #[cfg(not(feature = "va41628"))]
    pub pb5: Pin<Pb5>,
    #[cfg(not(feature = "va41628"))]
    pub pb6: Pin<Pb6>,
    #[cfg(not(feature = "va41628"))]
    pub pb7: Pin<Pb7>,
    #[cfg(not(feature = "va41628"))]
    pub pb8: Pin<Pb8>,
    #[cfg(not(feature = "va41628"))]
    pub pb9: Pin<Pb9>,
    #[cfg(not(feature = "va41628"))]
    pub pb10: Pin<Pb10>,
    #[cfg(not(feature = "va41628"))]
    pub pb11: Pin<Pb11>,
    pub pb12: Pin<Pb12>,
    pub pb13: Pin<Pb13>,
    pub pb14: Pin<Pb14>,
    pub pb15: Pin<Pb15>,
    #[cfg(feature = "vor1x")]
    pub pb16: Pin<Pb16>,
    #[cfg(feature = "vor1x")]
    pub pb17: Pin<Pb17>,
    #[cfg(feature = "vor1x")]
    pub pb18: Pin<Pb18>,
    #[cfg(feature = "vor1x")]
    pub pb19: Pin<Pb19>,
    #[cfg(feature = "vor1x")]
    pub pb20: Pin<Pb20>,
    #[cfg(feature = "vor1x")]
    pub pb21: Pin<Pb21>,
    #[cfg(feature = "vor1x")]
    pub pb22: Pin<Pb22>,
    #[cfg(feature = "vor1x")]
    pub pb23: Pin<Pb23>,
}

impl PinsB {
    pub fn new(_port_b: pac::Portb) -> Self {
        let syscfg = unsafe { pac::Sysconfig::steal() };
        reset_peripheral_for_cycles(PeripheralSelect::PortB, 2);
        syscfg.peripheral_clk_enable().modify(|_, w| {
            w.portb().set_bit();
            #[cfg(feature = "vor1x")]
            w.gpio().set_bit();
            w.ioconfig().set_bit()
        });
        Self {
            pb0: Pin::__new(),
            pb1: Pin::__new(),
            pb2: Pin::__new(),
            pb3: Pin::__new(),
            pb4: Pin::__new(),
            #[cfg(not(feature = "va41628"))]
            pb5: Pin::__new(),
            #[cfg(not(feature = "va41628"))]
            pb6: Pin::__new(),
            #[cfg(not(feature = "va41628"))]
            pb7: Pin::__new(),
            #[cfg(not(feature = "va41628"))]
            pb8: Pin::__new(),
            #[cfg(not(feature = "va41628"))]
            pb9: Pin::__new(),
            #[cfg(not(feature = "va41628"))]
            pb10: Pin::__new(),
            #[cfg(not(feature = "va41628"))]
            pb11: Pin::__new(),
            pb12: Pin::__new(),
            pb13: Pin::__new(),
            pb14: Pin::__new(),
            pb15: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pb16: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pb17: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pb18: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pb19: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pb20: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pb21: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pb22: Pin::__new(),
            #[cfg(feature = "vor1x")]
            pb23: Pin::__new(),
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "vor4x")] {
        /// Resource management singleton for GPIO PORT C.
        pub struct PinsC {
            pub pc0: Pin<Pc0>,
            pub pc1: Pin<Pc1>,
            pub pc2: Pin<Pc2>,
            pub pc3: Pin<Pc3>,
            pub pc4: Pin<Pc4>,
            pub pc5: Pin<Pc5>,
            pub pc6: Pin<Pc6>,
            pub pc7: Pin<Pc7>,
            pub pc8: Pin<Pc8>,
            pub pc9: Pin<Pc9>,
            pub pc10: Pin<Pc10>,
            pub pc11: Pin<Pc11>,
            pub pc12: Pin<Pc12>,
            #[cfg(not(feature = "va41628"))]
            pub pc13: Pin<Pc13>,
            pub pc14: Pin<Pc14>,
            #[cfg(not(feature = "va41628"))]
            pub pc15: Pin<Pc15>,
        }

        impl PinsC {
            pub fn new(_port_c: pac::Portc) -> Self {
                let syscfg = unsafe { pac::Sysconfig::steal() };
                reset_peripheral_for_cycles(PeripheralSelect::PortC, 2);
                syscfg.peripheral_clk_enable().modify(|_, w| {
                    w.portc().set_bit();
                    w.ioconfig().set_bit()
                });
                Self {
                    pc0: Pin::__new(),
                    pc1: Pin::__new(),
                    pc2: Pin::__new(),
                    pc3: Pin::__new(),
                    pc4: Pin::__new(),
                    pc5: Pin::__new(),
                    pc6: Pin::__new(),
                    pc7: Pin::__new(),
                    pc8: Pin::__new(),
                    pc9: Pin::__new(),
                    pc10: Pin::__new(),
                    pc11: Pin::__new(),
                    pc12: Pin::__new(),
            #[cfg(not(feature = "va41628"))]
                    pc13: Pin::__new(),
                    pc14: Pin::__new(),
            #[cfg(not(feature = "va41628"))]
                    pc15: Pin::__new(),
                }
            }
        }

        /// Resource management singleton for GPIO PORT D.
        pub struct PinsD {
            #[cfg(not(feature = "va41628"))]
            pub pd0: Pin<Pd0>,
            #[cfg(not(feature = "va41628"))]
            pub pd1: Pin<Pd1>,
            #[cfg(not(feature = "va41628"))]
            pub pd2: Pin<Pd2>,
            #[cfg(not(feature = "va41628"))]
            pub pd3: Pin<Pd3>,
            #[cfg(not(feature = "va41628"))]
            pub pd4: Pin<Pd4>,
            #[cfg(not(feature = "va41628"))]
            pub pd5: Pin<Pd5>,
            #[cfg(not(feature = "va41628"))]
            pub pd6: Pin<Pd6>,
            #[cfg(not(feature = "va41628"))]
            pub pd7: Pin<Pd7>,
            #[cfg(not(feature = "va41628"))]
            pub pd8: Pin<Pd8>,
            #[cfg(not(feature = "va41628"))]
            pub pd9: Pin<Pd9>,
            pub pd10: Pin<Pd10>,
            pub pd11: Pin<Pd11>,
            pub pd12: Pin<Pd12>,
            pub pd13: Pin<Pd13>,
            pub pd14: Pin<Pd14>,
            pub pd15: Pin<Pd15>,
        }

        impl PinsD {
            pub fn new(_port_d: pac::Portd) -> Self {
                let syscfg = unsafe { pac::Sysconfig::steal() };
                reset_peripheral_for_cycles(PeripheralSelect::PortD, 2);
                syscfg.peripheral_clk_enable().modify(|_, w| {
                    w.portd().set_bit();
                    w.ioconfig().set_bit()
                });
                Self {
                    #[cfg(not(feature = "va41628"))]
                    pd0: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pd1: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pd2: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pd3: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pd4: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pd5: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pd6: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pd7: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pd8: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pd9: Pin::__new(),
                    pd10: Pin::__new(),
                    pd11: Pin::__new(),
                    pd12: Pin::__new(),
                    pd13: Pin::__new(),
                    pd14: Pin::__new(),
                    pd15: Pin::__new(),
                }
            }
        }

        /// Resource management singleton for GPIO PORT E.
        pub struct PinsE {
            pub pe0: Pin<Pe0>,
            pub pe1: Pin<Pe1>,
            pub pe2: Pin<Pe2>,
            pub pe3: Pin<Pe3>,
            pub pe4: Pin<Pe4>,
            pub pe5: Pin<Pe5>,
            pub pe6: Pin<Pe6>,
            pub pe7: Pin<Pe7>,
            pub pe8: Pin<Pe8>,
            pub pe9: Pin<Pe9>,
            #[cfg(not(feature = "va41628"))]
            pub pe10: Pin<Pe10>,
            #[cfg(not(feature = "va41628"))]
            pub pe11: Pin<Pe11>,
            pub pe12: Pin<Pe12>,
            pub pe13: Pin<Pe13>,
            pub pe14: Pin<Pe14>,
            pub pe15: Pin<Pe15>,
        }

        impl PinsE {
            pub fn new(_port_e: pac::Porte) -> Self {
                let syscfg = unsafe { pac::Sysconfig::steal() };
                reset_peripheral_for_cycles(PeripheralSelect::PortE, 2);
                syscfg.peripheral_clk_enable().modify(|_, w| {
                    w.porte().set_bit();
                    w.ioconfig().set_bit()
                });
                Self {
                    pe0: Pin::__new(),
                    pe1: Pin::__new(),
                    pe2: Pin::__new(),
                    pe3: Pin::__new(),
                    pe4: Pin::__new(),
                    pe5: Pin::__new(),
                    pe6: Pin::__new(),
                    pe7: Pin::__new(),
                    pe8: Pin::__new(),
                    pe9: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pe10: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pe11: Pin::__new(),
                    pe12: Pin::__new(),
                    pe13: Pin::__new(),
                    pe14: Pin::__new(),
                    pe15: Pin::__new(),
                }
            }
        }

        /// Resource management singleton for GPIO PORT F.
        pub struct PinsF {
            pub pf0: Pin<Pf0>,
            pub pf1: Pin<Pf1>,
            #[cfg(not(feature = "va41628"))]
            pub pf2: Pin<Pf2>,
            #[cfg(not(feature = "va41628"))]
            pub pf3: Pin<Pf3>,
            #[cfg(not(feature = "va41628"))]
            pub pf4: Pin<Pf4>,
            #[cfg(not(feature = "va41628"))]
            pub pf5: Pin<Pf5>,
            #[cfg(not(feature = "va41628"))]
            pub pf6: Pin<Pf6>,
            #[cfg(not(feature = "va41628"))]
            pub pf7: Pin<Pf7>,
            #[cfg(not(feature = "va41628"))]
            pub pf8: Pin<Pf8>,
            pub pf9: Pin<Pf9>,
            #[cfg(not(feature = "va41628"))]
            pub pf10: Pin<Pf10>,
            pub pf11: Pin<Pf11>,
            pub pf12: Pin<Pf12>,
            pub pf13: Pin<Pf13>,
            pub pf14: Pin<Pf14>,
            pub pf15: Pin<Pf15>,
        }

        impl PinsF {
            pub fn new(_port_f: pac::Portf) -> Self {
                let syscfg = unsafe { pac::Sysconfig::steal() };
                reset_peripheral_for_cycles(PeripheralSelect::PortF, 2);
                syscfg.peripheral_clk_enable().modify(|_, w| {
                    w.portf().set_bit();
                    w.ioconfig().set_bit()
                });
                Self {
                    pf0: Pin::__new(),
                    pf1: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pf2: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pf3: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pf4: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pf5: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pf6: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pf7: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pf8: Pin::__new(),
                    pf9: Pin::__new(),
                    #[cfg(not(feature = "va41628"))]
                    pf10: Pin::__new(),
                    pf11: Pin::__new(),
                    pf12: Pin::__new(),
                    pf13: Pin::__new(),
                    pf14: Pin::__new(),
                    pf15: Pin::__new(),
                }
            }
        }

        /// Resource management singleton for GPIO PORT G.
        pub struct PinsG {
            pub pg0: Pin<Pg0>,
            pub pg1: Pin<Pg1>,
            pub pg2: Pin<Pg2>,
            pub pg3: Pin<Pg3>,
            pub pg4: Pin<Pg4>,
            pub pg5: Pin<Pg5>,
            pub pg6: Pin<Pg6>,
            pub pg7: Pin<Pg7>,
        }

        impl PinsG {
            pub fn new(_port_g: pac::Portg) -> Self {
                let syscfg = unsafe { pac::Sysconfig::steal() };
                reset_peripheral_for_cycles(PeripheralSelect::PortG, 2);
                syscfg.peripheral_clk_enable().modify(|_, w| {
                    w.portg().set_bit();
                    w.ioconfig().set_bit()
                });
                Self {
                    pg0: Pin::__new(),
                    pg1: Pin::__new(),
                    pg2: Pin::__new(),
                    pg3: Pin::__new(),
                    pg4: Pin::__new(),
                    pg5: Pin::__new(),
                    pg6: Pin::__new(),
                    pg7: Pin::__new(),
                }
            }
        }
    }
}
