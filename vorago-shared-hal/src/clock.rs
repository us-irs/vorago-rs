use crate::time::Hertz;

pub const HBO_FREQ: Hertz = Hertz::from_raw(20_000_000);

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed.
/// The [self] module documentation gives some more information on how to retrieve an instance
/// of this structure.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Clocks {
    sysclk: Hertz,
    apb1: Hertz,
    apb2: Hertz,
    #[cfg(not(feature = "va41628"))]
    adc_clk: Hertz,
}

impl Clocks {
    #[doc(hidden)]
    pub fn __new(final_sysclk: Hertz, #[cfg(not(feature = "va41628"))] adc_clk: Hertz) -> Self {
        Self {
            sysclk: final_sysclk,
            apb1: final_sysclk / 2,
            apb2: final_sysclk / 4,
            #[cfg(not(feature = "va41628"))]
            adc_clk,
        }
    }

    /// Returns the frequency of the HBO clock
    pub const fn hbo(&self) -> Hertz {
        HBO_FREQ
    }

    /// Returns the frequency of the APB0 which is equal to the system clock.
    pub const fn apb0(&self) -> Hertz {
        self.sysclk()
    }

    /// Returns system clock divied by 2.
    pub const fn apb1(&self) -> Hertz {
        self.apb1
    }

    /// Returns system clock divied by 4.
    pub const fn apb2(&self) -> Hertz {
        self.apb2
    }

    /// Returns the system (core) frequency
    pub const fn sysclk(&self) -> Hertz {
        self.sysclk
    }

    /// Returns the ADC clock frequency which has a separate divider.
    #[cfg(not(feature = "va41628"))]
    pub const fn adc_clk(&self) -> Hertz {
        self.adc_clk
    }
}
