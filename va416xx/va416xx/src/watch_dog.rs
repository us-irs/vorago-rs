#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wdogload: Wdogload,
    wdogvalue: Wdogvalue,
    wdogcontrol: Wdogcontrol,
    wdogintclr: Wdogintclr,
    wdogris: Wdogris,
    wdogmis: Wdogmis,
    _reserved6: [u8; 0xa8],
    wdoglock: Wdoglock,
    _reserved7: [u8; 0x0e3c],
    wdogitcr: Wdogitcr,
    wdogitop: Wdogitop,
    _reserved9: [u8; 0xd8],
    wdogperiphid0: Wdogperiphid0,
    wdogperiphid1: Wdogperiphid1,
    wdogperiphid2: Wdogperiphid2,
    wdogperiphid3: Wdogperiphid3,
    wdogpcellid0: Wdogpcellid0,
    wdogpcellid1: Wdogpcellid1,
    wdogpcellid2: Wdogpcellid2,
    wdogpcellid3: Wdogpcellid3,
}
impl RegisterBlock {
    #[doc = "0x00 - Counter Start Value"]
    #[inline(always)]
    pub const fn wdogload(&self) -> &Wdogload {
        &self.wdogload
    }
    #[doc = "0x04 - Down Counter Value"]
    #[inline(always)]
    pub const fn wdogvalue(&self) -> &Wdogvalue {
        &self.wdogvalue
    }
    #[doc = "0x08 - Enable for block reset and interrupt"]
    #[inline(always)]
    pub const fn wdogcontrol(&self) -> &Wdogcontrol {
        &self.wdogcontrol
    }
    #[doc = "0x0c - A write of any value clears the WDT module interrupt, and reloads the counter from the value in the WDOGLOAD Register"]
    #[inline(always)]
    pub const fn wdogintclr(&self) -> &Wdogintclr {
        &self.wdogintclr
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn wdogris(&self) -> &Wdogris {
        &self.wdogris
    }
    #[doc = "0x14 - Interrupt status"]
    #[inline(always)]
    pub const fn wdogmis(&self) -> &Wdogmis {
        &self.wdogmis
    }
    #[doc = "0xc0 - Lock"]
    #[inline(always)]
    pub const fn wdoglock(&self) -> &Wdoglock {
        &self.wdoglock
    }
    #[doc = "0xf00 - Integration test control"]
    #[inline(always)]
    pub const fn wdogitcr(&self) -> &Wdogitcr {
        &self.wdogitcr
    }
    #[doc = "0xf04 - Integration test output set"]
    #[inline(always)]
    pub const fn wdogitop(&self) -> &Wdogitop {
        &self.wdogitop
    }
    #[doc = "0xfe0 - Peripheral ID"]
    #[inline(always)]
    pub const fn wdogperiphid0(&self) -> &Wdogperiphid0 {
        &self.wdogperiphid0
    }
    #[doc = "0xfe4 - Peripheral ID"]
    #[inline(always)]
    pub const fn wdogperiphid1(&self) -> &Wdogperiphid1 {
        &self.wdogperiphid1
    }
    #[doc = "0xfe8 - Peripheral ID"]
    #[inline(always)]
    pub const fn wdogperiphid2(&self) -> &Wdogperiphid2 {
        &self.wdogperiphid2
    }
    #[doc = "0xfec - Peripheral ID"]
    #[inline(always)]
    pub const fn wdogperiphid3(&self) -> &Wdogperiphid3 {
        &self.wdogperiphid3
    }
    #[doc = "0xff0 - PrimeCell ID"]
    #[inline(always)]
    pub const fn wdogpcellid0(&self) -> &Wdogpcellid0 {
        &self.wdogpcellid0
    }
    #[doc = "0xff4 - PrimeCell ID"]
    #[inline(always)]
    pub const fn wdogpcellid1(&self) -> &Wdogpcellid1 {
        &self.wdogpcellid1
    }
    #[doc = "0xff8 - PrimeCell ID"]
    #[inline(always)]
    pub const fn wdogpcellid2(&self) -> &Wdogpcellid2 {
        &self.wdogpcellid2
    }
    #[doc = "0xffc - PrimeCell ID"]
    #[inline(always)]
    pub const fn wdogpcellid3(&self) -> &Wdogpcellid3 {
        &self.wdogpcellid3
    }
}
#[doc = "WDOGLOAD (rw) register accessor: Counter Start Value\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogload`] module"]
#[doc(alias = "WDOGLOAD")]
pub type Wdogload = crate::Reg<wdogload::WdogloadSpec>;
#[doc = "Counter Start Value"]
pub mod wdogload;
#[doc = "WDOGVALUE (r) register accessor: Down Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogvalue::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogvalue`] module"]
#[doc(alias = "WDOGVALUE")]
pub type Wdogvalue = crate::Reg<wdogvalue::WdogvalueSpec>;
#[doc = "Down Counter Value"]
pub mod wdogvalue;
#[doc = "WDOGCONTROL (rw) register accessor: Enable for block reset and interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogcontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogcontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogcontrol`] module"]
#[doc(alias = "WDOGCONTROL")]
pub type Wdogcontrol = crate::Reg<wdogcontrol::WdogcontrolSpec>;
#[doc = "Enable for block reset and interrupt"]
pub mod wdogcontrol;
#[doc = "WDOGINTCLR (rw) register accessor: A write of any value clears the WDT module interrupt, and reloads the counter from the value in the WDOGLOAD Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogintclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogintclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogintclr`] module"]
#[doc(alias = "WDOGINTCLR")]
pub type Wdogintclr = crate::Reg<wdogintclr::WdogintclrSpec>;
#[doc = "A write of any value clears the WDT module interrupt, and reloads the counter from the value in the WDOGLOAD Register"]
pub mod wdogintclr;
#[doc = "WDOGRIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogris`] module"]
#[doc(alias = "WDOGRIS")]
pub type Wdogris = crate::Reg<wdogris::WdogrisSpec>;
#[doc = "Raw interrupt status"]
pub mod wdogris;
#[doc = "WDOGMIS (r) register accessor: Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogmis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogmis`] module"]
#[doc(alias = "WDOGMIS")]
pub type Wdogmis = crate::Reg<wdogmis::WdogmisSpec>;
#[doc = "Interrupt status"]
pub mod wdogmis;
#[doc = "WDOGLOCK (rw) register accessor: Lock\n\nYou can [`read`](crate::Reg::read) this register and get [`wdoglock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdoglock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdoglock`] module"]
#[doc(alias = "WDOGLOCK")]
pub type Wdoglock = crate::Reg<wdoglock::WdoglockSpec>;
#[doc = "Lock"]
pub mod wdoglock;
#[doc = "WDOGITCR (rw) register accessor: Integration test control\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogitcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogitcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogitcr`] module"]
#[doc(alias = "WDOGITCR")]
pub type Wdogitcr = crate::Reg<wdogitcr::WdogitcrSpec>;
#[doc = "Integration test control"]
pub mod wdogitcr;
#[doc = "WDOGITOP (rw) register accessor: Integration test output set\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogitop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogitop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogitop`] module"]
#[doc(alias = "WDOGITOP")]
pub type Wdogitop = crate::Reg<wdogitop::WdogitopSpec>;
#[doc = "Integration test output set"]
pub mod wdogitop;
#[doc = "WDOGPERIPHID0 (r) register accessor: Peripheral ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogperiphid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogperiphid0`] module"]
#[doc(alias = "WDOGPERIPHID0")]
pub type Wdogperiphid0 = crate::Reg<wdogperiphid0::Wdogperiphid0Spec>;
#[doc = "Peripheral ID"]
pub mod wdogperiphid0;
#[doc = "WDOGPERIPHID1 (r) register accessor: Peripheral ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogperiphid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogperiphid1`] module"]
#[doc(alias = "WDOGPERIPHID1")]
pub type Wdogperiphid1 = crate::Reg<wdogperiphid1::Wdogperiphid1Spec>;
#[doc = "Peripheral ID"]
pub mod wdogperiphid1;
#[doc = "WDOGPERIPHID2 (r) register accessor: Peripheral ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogperiphid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogperiphid2`] module"]
#[doc(alias = "WDOGPERIPHID2")]
pub type Wdogperiphid2 = crate::Reg<wdogperiphid2::Wdogperiphid2Spec>;
#[doc = "Peripheral ID"]
pub mod wdogperiphid2;
#[doc = "WDOGPERIPHID3 (r) register accessor: Peripheral ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogperiphid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogperiphid3`] module"]
#[doc(alias = "WDOGPERIPHID3")]
pub type Wdogperiphid3 = crate::Reg<wdogperiphid3::Wdogperiphid3Spec>;
#[doc = "Peripheral ID"]
pub mod wdogperiphid3;
#[doc = "WDOGPCELLID0 (r) register accessor: PrimeCell ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogpcellid0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogpcellid0`] module"]
#[doc(alias = "WDOGPCELLID0")]
pub type Wdogpcellid0 = crate::Reg<wdogpcellid0::Wdogpcellid0Spec>;
#[doc = "PrimeCell ID"]
pub mod wdogpcellid0;
#[doc = "WDOGPCELLID1 (r) register accessor: PrimeCell ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogpcellid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogpcellid1`] module"]
#[doc(alias = "WDOGPCELLID1")]
pub type Wdogpcellid1 = crate::Reg<wdogpcellid1::Wdogpcellid1Spec>;
#[doc = "PrimeCell ID"]
pub mod wdogpcellid1;
#[doc = "WDOGPCELLID2 (r) register accessor: PrimeCell ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogpcellid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogpcellid2`] module"]
#[doc(alias = "WDOGPCELLID2")]
pub type Wdogpcellid2 = crate::Reg<wdogpcellid2::Wdogpcellid2Spec>;
#[doc = "PrimeCell ID"]
pub mod wdogpcellid2;
#[doc = "WDOGPCELLID3 (r) register accessor: PrimeCell ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogpcellid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdogpcellid3`] module"]
#[doc(alias = "WDOGPCELLID3")]
pub type Wdogpcellid3 = crate::Reg<wdogpcellid3::Wdogpcellid3Spec>;
#[doc = "PrimeCell ID"]
pub mod wdogpcellid3;
