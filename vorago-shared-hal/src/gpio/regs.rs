use crate::Port;

cfg_if::cfg_if! {
    if #[cfg(feature = "vor1x")] {
        /// PORT A base address.
        pub const GPIO_0_BASE: usize = 0x5000_0000;
        /// PORT B base address.
        pub const GPIO_1_BASE: usize = 0x5000_1000;
    } else if #[cfg(feature = "vor4x")] {
        /// PORT A base address.
        pub const GPIO_0_BASE: usize = 0x4001_2000;
        /// PORT B base address.
        pub const GPIO_1_BASE: usize = 0x4001_2400;
        /// PORT C base address.
        pub const GPIO_2_BASE: usize = 0x4001_2800;
        /// PORT D base address.
        pub const GPIO_3_BASE: usize = 0x4001_2C00;
        /// PORT E base address.
        pub const GPIO_4_BASE: usize = 0x4001_3000;
        /// PORT F base address.
        pub const GPIO_5_BASE: usize = 0x4001_3400;
        /// PORT G base address.
        pub const GPIO_6_BASE: usize = 0x4001_3800;
    }
}

#[derive(derive_mmio::Mmio)]
#[mmio(no_ctors)]
#[repr(C)]
pub struct Gpio {
    #[mmio(PureRead)]
    data_in: u32,
    #[mmio(PureRead)]
    data_in_raw: u32,
    data_out: u32,
    data_out_raw: u32,
    #[mmio(Write)]
    set_out: u32,
    #[mmio(Write)]
    clr_out: u32,
    #[mmio(Write)]
    tog_out: u32,
    data_mask: u32,
    /// Direction bits. 1 for output, 0 for input.
    dir: u32,
    pulse: u32,
    pulsebase: u32,
    delay1: u32,
    delay2: u32,
    irq_sen: u32,
    irq_edge: u32,
    irq_evt: u32,
    irq_enable: u32,
    /// Raw interrupt status. This register is not latched and may not indicated edge sensitive
    /// events.
    #[mmio(PureRead)]
    irq_raw: u32,
    /// Read-only register which shows enabled and active interrupts. Called IRQ_end by Vorago.
    #[mmio(PureRead)]
    irq_status: u32,
    #[mmio(PureRead)]
    edge_status: u32,

    #[cfg(feature = "vor1x")]
    _reserved: [u32; 0x3eb],
    #[cfg(feature = "vor4x")]
    _reserved: [u32; 0xeb],

    /// Peripheral ID. Vorago 1x reset value: 0x0040_07e1. Vorago 4x reset value: 0x0210_07E9.
    perid: u32,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "vor1x")] {
        static_assertions::const_assert_eq!(core::mem::size_of::<Gpio>(), 0x1000);
    } else if #[cfg(feature = "vor4x")] {
        static_assertions::const_assert_eq!(core::mem::size_of::<Gpio>(), 0x400);
    }
}

impl Gpio {
    const fn new_mmio_at(base: usize) -> MmioGpio<'static> {
        MmioGpio {
            ptr: base as *mut _,
            phantom: core::marker::PhantomData,
        }
    }

    pub const fn new_mmio(port: Port) -> MmioGpio<'static> {
        match port {
            Port::A => Self::new_mmio_at(GPIO_0_BASE),
            Port::B => Self::new_mmio_at(GPIO_1_BASE),
            #[cfg(feature = "vor4x")]
            Port::C => Self::new_mmio_at(GPIO_2_BASE),
            #[cfg(feature = "vor4x")]
            Port::D => Self::new_mmio_at(GPIO_3_BASE),
            #[cfg(feature = "vor4x")]
            Port::E => Self::new_mmio_at(GPIO_4_BASE),
            #[cfg(feature = "vor4x")]
            Port::F => Self::new_mmio_at(GPIO_5_BASE),
            #[cfg(feature = "vor4x")]
            Port::G => Self::new_mmio_at(GPIO_6_BASE),
        }
    }
}

impl MmioGpio<'_> {
    pub fn port(&self) -> Port {
        match unsafe { self.ptr() } as usize {
            GPIO_0_BASE => Port::A,
            GPIO_1_BASE => Port::B,
            #[cfg(feature = "vor4x")]
            GPIO_2_BASE => Port::C,
            #[cfg(feature = "vor4x")]
            GPIO_3_BASE => Port::D,
            #[cfg(feature = "vor4x")]
            GPIO_4_BASE => Port::E,
            #[cfg(feature = "vor4x")]
            GPIO_5_BASE => Port::F,
            #[cfg(feature = "vor4x")]
            GPIO_6_BASE => Port::G,
            // Constructors were disabled, so this should really not happen.
            _ => panic!("unexpected base address of GPIO register block"),
        }
    }
}
