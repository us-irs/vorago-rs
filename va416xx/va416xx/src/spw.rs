#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    sts: Sts,
    defaddr: Defaddr,
    clkdiv: Clkdiv,
    dkey: Dkey,
    tc: Tc,
    tdr: Tdr,
    _reserved7: [u8; 0x04],
    dmactrl0: Dmactrl0,
    dmamaxlen0: Dmamaxlen0,
    dmatxdesc0: Dmatxdesc0,
    dmarxdesc0: Dmarxdesc0,
    dmaaddr0: Dmaaddr0,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Status/Interrupt Source Register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x08 - Node Address Register"]
    #[inline(always)]
    pub const fn defaddr(&self) -> &Defaddr {
        &self.defaddr
    }
    #[doc = "0x0c - Clock Divisor Register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x10 - Destination Key"]
    #[inline(always)]
    pub const fn dkey(&self) -> &Dkey {
        &self.dkey
    }
    #[doc = "0x14 - Time Code Register"]
    #[inline(always)]
    pub const fn tc(&self) -> &Tc {
        &self.tc
    }
    #[doc = "0x18 - Timer and Disconnect Register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &Tdr {
        &self.tdr
    }
    #[doc = "0x20 - DMA Control Register"]
    #[inline(always)]
    pub const fn dmactrl0(&self) -> &Dmactrl0 {
        &self.dmactrl0
    }
    #[doc = "0x24 - DMA RX Maximum Length Register"]
    #[inline(always)]
    pub const fn dmamaxlen0(&self) -> &Dmamaxlen0 {
        &self.dmamaxlen0
    }
    #[doc = "0x28 - DMA Transmitter Descriptor Table Address Register"]
    #[inline(always)]
    pub const fn dmatxdesc0(&self) -> &Dmatxdesc0 {
        &self.dmatxdesc0
    }
    #[doc = "0x2c - DMA Receiver Table Destination Register"]
    #[inline(always)]
    pub const fn dmarxdesc0(&self) -> &Dmarxdesc0 {
        &self.dmarxdesc0
    }
    #[doc = "0x30 - DMA Receiver Table Address Register"]
    #[inline(always)]
    pub const fn dmaaddr0(&self) -> &Dmaaddr0 {
        &self.dmaaddr0
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STS (rw) register accessor: Status/Interrupt Source Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`] module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "Status/Interrupt Source Register"]
pub mod sts;
#[doc = "DEFADDR (rw) register accessor: Node Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`defaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`defaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@defaddr`] module"]
#[doc(alias = "DEFADDR")]
pub type Defaddr = crate::Reg<defaddr::DefaddrSpec>;
#[doc = "Node Address Register"]
pub mod defaddr;
#[doc = "CLKDIV (rw) register accessor: Clock Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock Divisor Register"]
pub mod clkdiv;
#[doc = "DKEY (rw) register accessor: Destination Key\n\nYou can [`read`](crate::Reg::read) this register and get [`dkey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dkey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dkey`] module"]
#[doc(alias = "DKEY")]
pub type Dkey = crate::Reg<dkey::DkeySpec>;
#[doc = "Destination Key"]
pub mod dkey;
#[doc = "TC (rw) register accessor: Time Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc`] module"]
#[doc(alias = "TC")]
pub type Tc = crate::Reg<tc::TcSpec>;
#[doc = "Time Code Register"]
pub mod tc;
#[doc = "TDR (r) register accessor: Timer and Disconnect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`] module"]
#[doc(alias = "TDR")]
pub type Tdr = crate::Reg<tdr::TdrSpec>;
#[doc = "Timer and Disconnect Register"]
pub mod tdr;
#[doc = "DMACTRL0 (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactrl0`] module"]
#[doc(alias = "DMACTRL0")]
pub type Dmactrl0 = crate::Reg<dmactrl0::Dmactrl0Spec>;
#[doc = "DMA Control Register"]
pub mod dmactrl0;
#[doc = "DMAMAXLEN0 (rw) register accessor: DMA RX Maximum Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmamaxlen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamaxlen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamaxlen0`] module"]
#[doc(alias = "DMAMAXLEN0")]
pub type Dmamaxlen0 = crate::Reg<dmamaxlen0::Dmamaxlen0Spec>;
#[doc = "DMA RX Maximum Length Register"]
pub mod dmamaxlen0;
#[doc = "DMATXDESC0 (rw) register accessor: DMA Transmitter Descriptor Table Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatxdesc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatxdesc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatxdesc0`] module"]
#[doc(alias = "DMATXDESC0")]
pub type Dmatxdesc0 = crate::Reg<dmatxdesc0::Dmatxdesc0Spec>;
#[doc = "DMA Transmitter Descriptor Table Address Register"]
pub mod dmatxdesc0;
#[doc = "DMARXDESC0 (rw) register accessor: DMA Receiver Table Destination Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarxdesc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarxdesc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarxdesc0`] module"]
#[doc(alias = "DMARXDESC0")]
pub type Dmarxdesc0 = crate::Reg<dmarxdesc0::Dmarxdesc0Spec>;
#[doc = "DMA Receiver Table Destination Register"]
pub mod dmarxdesc0;
#[doc = "DMAADDR0 (rw) register accessor: DMA Receiver Table Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaaddr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaaddr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaaddr0`] module"]
#[doc(alias = "DMAADDR0")]
pub type Dmaaddr0 = crate::Reg<dmaaddr0::Dmaaddr0Spec>;
#[doc = "DMA Receiver Table Address Register"]
pub mod dmaaddr0;
