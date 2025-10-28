#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    imr: Imr,
    isr: Isr,
    icr: Icr,
    config: Config,
    valid: Valid,
    ehr_data0: EhrData0,
    ehr_data1: EhrData1,
    ehr_data2: EhrData2,
    ehr_data3: EhrData3,
    ehr_data4: EhrData4,
    ehr_data5: EhrData5,
    rnd_source_enable: RndSourceEnable,
    sample_cnt1: SampleCnt1,
    autocorr_statistic: AutocorrStatistic,
    debug_control: DebugControl,
    _reserved15: [u8; 0x04],
    sw_reset: SwReset,
    _reserved16: [u8; 0x74],
    busy: Busy,
    rst_bits_counter: RstBitsCounter,
    _reserved18: [u8; 0x20],
    bist_cntr0: BistCntr0,
    bist_cntr1: BistCntr1,
    bist_cntr2: BistCntr2,
}
impl RegisterBlock {
    #[doc = "0x100 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x104 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x108 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x10c - Configuration Register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x110 - Valid Register"]
    #[inline(always)]
    pub const fn valid(&self) -> &Valid {
        &self.valid
    }
    #[doc = "0x114 - Entropy Holding Register Data Register"]
    #[inline(always)]
    pub const fn ehr_data0(&self) -> &EhrData0 {
        &self.ehr_data0
    }
    #[doc = "0x118 - Entropy Holding Register Data Register"]
    #[inline(always)]
    pub const fn ehr_data1(&self) -> &EhrData1 {
        &self.ehr_data1
    }
    #[doc = "0x11c - Entropy Holding Register Data Register"]
    #[inline(always)]
    pub const fn ehr_data2(&self) -> &EhrData2 {
        &self.ehr_data2
    }
    #[doc = "0x120 - Entropy Holding Register Data Register"]
    #[inline(always)]
    pub const fn ehr_data3(&self) -> &EhrData3 {
        &self.ehr_data3
    }
    #[doc = "0x124 - Entropy Holding Register Data Register"]
    #[inline(always)]
    pub const fn ehr_data4(&self) -> &EhrData4 {
        &self.ehr_data4
    }
    #[doc = "0x128 - Entropy Holding Register Data Register"]
    #[inline(always)]
    pub const fn ehr_data5(&self) -> &EhrData5 {
        &self.ehr_data5
    }
    #[doc = "0x12c - Random Source Enable Register"]
    #[inline(always)]
    pub const fn rnd_source_enable(&self) -> &RndSourceEnable {
        &self.rnd_source_enable
    }
    #[doc = "0x130 - Section TBD"]
    #[inline(always)]
    pub const fn sample_cnt1(&self) -> &SampleCnt1 {
        &self.sample_cnt1
    }
    #[doc = "0x134 - Auto-correlator Statistic Register"]
    #[inline(always)]
    pub const fn autocorr_statistic(&self) -> &AutocorrStatistic {
        &self.autocorr_statistic
    }
    #[doc = "0x138 - Section TBD"]
    #[inline(always)]
    pub const fn debug_control(&self) -> &DebugControl {
        &self.debug_control
    }
    #[doc = "0x140 - Reset Register"]
    #[inline(always)]
    pub const fn sw_reset(&self) -> &SwReset {
        &self.sw_reset
    }
    #[doc = "0x1b8 - Busy Register"]
    #[inline(always)]
    pub const fn busy(&self) -> &Busy {
        &self.busy
    }
    #[doc = "0x1bc - Reset Bits Counter Register"]
    #[inline(always)]
    pub const fn rst_bits_counter(&self) -> &RstBitsCounter {
        &self.rst_bits_counter
    }
    #[doc = "0x1e0 - BIST Counter Register"]
    #[inline(always)]
    pub const fn bist_cntr0(&self) -> &BistCntr0 {
        &self.bist_cntr0
    }
    #[doc = "0x1e4 - BIST Counter Register"]
    #[inline(always)]
    pub const fn bist_cntr1(&self) -> &BistCntr1 {
        &self.bist_cntr1
    }
    #[doc = "0x1e8 - BIST Counter Register"]
    #[inline(always)]
    pub const fn bist_cntr2(&self) -> &BistCntr2 {
        &self.bist_cntr2
    }
}
#[doc = "IMR (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "CONFIG (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration Register"]
pub mod config;
#[doc = "VALID (r) register accessor: Valid Register\n\nYou can [`read`](crate::Reg::read) this register and get [`valid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@valid`] module"]
#[doc(alias = "VALID")]
pub type Valid = crate::Reg<valid::ValidSpec>;
#[doc = "Valid Register"]
pub mod valid;
#[doc = "EHR_DATA0 (r) register accessor: Entropy Holding Register Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehr_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehr_data0`] module"]
#[doc(alias = "EHR_DATA0")]
pub type EhrData0 = crate::Reg<ehr_data0::EhrData0Spec>;
#[doc = "Entropy Holding Register Data Register"]
pub mod ehr_data0;
pub use ehr_data0 as ehr_data1;
pub use ehr_data0 as ehr_data2;
pub use ehr_data0 as ehr_data3;
pub use ehr_data0 as ehr_data4;
pub use ehr_data0 as ehr_data5;
pub use EhrData0 as EhrData1;
pub use EhrData0 as EhrData2;
pub use EhrData0 as EhrData3;
pub use EhrData0 as EhrData4;
pub use EhrData0 as EhrData5;
#[doc = "RND_SOURCE_ENABLE (rw) register accessor: Random Source Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_source_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_source_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rnd_source_enable`] module"]
#[doc(alias = "RND_SOURCE_ENABLE")]
pub type RndSourceEnable = crate::Reg<rnd_source_enable::RndSourceEnableSpec>;
#[doc = "Random Source Enable Register"]
pub mod rnd_source_enable;
#[doc = "SAMPLE_CNT1 (rw) register accessor: Section TBD\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample_cnt1`] module"]
#[doc(alias = "SAMPLE_CNT1")]
pub type SampleCnt1 = crate::Reg<sample_cnt1::SampleCnt1Spec>;
#[doc = "Section TBD"]
pub mod sample_cnt1;
#[doc = "AUTOCORR_STATISTIC (rw) register accessor: Auto-correlator Statistic Register\n\nYou can [`read`](crate::Reg::read) this register and get [`autocorr_statistic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocorr_statistic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocorr_statistic`] module"]
#[doc(alias = "AUTOCORR_STATISTIC")]
pub type AutocorrStatistic = crate::Reg<autocorr_statistic::AutocorrStatisticSpec>;
#[doc = "Auto-correlator Statistic Register"]
pub mod autocorr_statistic;
#[doc = "DEBUG_CONTROL (rw) register accessor: Section TBD\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_control`] module"]
#[doc(alias = "DEBUG_CONTROL")]
pub type DebugControl = crate::Reg<debug_control::DebugControlSpec>;
#[doc = "Section TBD"]
pub mod debug_control;
#[doc = "SW_RESET (rw) register accessor: Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_reset`] module"]
#[doc(alias = "SW_RESET")]
pub type SwReset = crate::Reg<sw_reset::SwResetSpec>;
#[doc = "Reset Register"]
pub mod sw_reset;
#[doc = "BUSY (r) register accessor: Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busy`] module"]
#[doc(alias = "BUSY")]
pub type Busy = crate::Reg<busy::BusySpec>;
#[doc = "Busy Register"]
pub mod busy;
#[doc = "RST_BITS_COUNTER (rw) register accessor: Reset Bits Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_bits_counter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_bits_counter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_bits_counter`] module"]
#[doc(alias = "RST_BITS_COUNTER")]
pub type RstBitsCounter = crate::Reg<rst_bits_counter::RstBitsCounterSpec>;
#[doc = "Reset Bits Counter Register"]
pub mod rst_bits_counter;
#[doc = "BIST_CNTR0 (r) register accessor: BIST Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bist_cntr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bist_cntr0`] module"]
#[doc(alias = "BIST_CNTR0")]
pub type BistCntr0 = crate::Reg<bist_cntr0::BistCntr0Spec>;
#[doc = "BIST Counter Register"]
pub mod bist_cntr0;
pub use bist_cntr0 as bist_cntr1;
pub use bist_cntr0 as bist_cntr2;
pub use BistCntr0 as BistCntr1;
pub use BistCntr0 as BistCntr2;
