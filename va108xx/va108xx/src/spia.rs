#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl0: Ctrl0,
    ctrl1: Ctrl1,
    data: Data,
    status: Status,
    clkprescale: Clkprescale,
    irq_enb: IrqEnb,
    irq_raw: IrqRaw,
    irq_end: IrqEnd,
    irq_clr: IrqClr,
    rxfifoirqtrg: Rxfifoirqtrg,
    txfifoirqtrg: Txfifoirqtrg,
    fifo_clr: FifoClr,
    state: State,
    _reserved13: [u8; 0x0fc8],
    perid: Perid,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register 0"]
    #[inline(always)]
    pub const fn ctrl0(&self) -> &Ctrl0 {
        &self.ctrl0
    }
    #[doc = "0x04 - Control Register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x08 - Data Input/Output"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x0c - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - Clock Pre Scale divide value"]
    #[inline(always)]
    pub const fn clkprescale(&self) -> &Clkprescale {
        &self.clkprescale
    }
    #[doc = "0x14 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn irq_enb(&self) -> &IrqEnb {
        &self.irq_enb
    }
    #[doc = "0x18 - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn irq_raw(&self) -> &IrqRaw {
        &self.irq_raw
    }
    #[doc = "0x1c - Enabled Interrupt Status Register"]
    #[inline(always)]
    pub const fn irq_end(&self) -> &IrqEnd {
        &self.irq_end
    }
    #[doc = "0x20 - Clear Interrupt Status Register"]
    #[inline(always)]
    pub const fn irq_clr(&self) -> &IrqClr {
        &self.irq_clr
    }
    #[doc = "0x24 - Rx FIFO IRQ Trigger Level"]
    #[inline(always)]
    pub const fn rxfifoirqtrg(&self) -> &Rxfifoirqtrg {
        &self.rxfifoirqtrg
    }
    #[doc = "0x28 - Tx FIFO IRQ Trigger Level"]
    #[inline(always)]
    pub const fn txfifoirqtrg(&self) -> &Txfifoirqtrg {
        &self.txfifoirqtrg
    }
    #[doc = "0x2c - Clear FIFO Register"]
    #[inline(always)]
    pub const fn fifo_clr(&self) -> &FifoClr {
        &self.fifo_clr
    }
    #[doc = "0x30 - Internal STATE of SPI Controller"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0xffc - Peripheral ID Register"]
    #[inline(always)]
    pub const fn perid(&self) -> &Perid {
        &self.perid
    }
}
#[doc = "CTRL0 (rw) register accessor: Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl0`] module"]
#[doc(alias = "CTRL0")]
pub type Ctrl0 = crate::Reg<ctrl0::Ctrl0Spec>;
#[doc = "Control Register 0"]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "Control Register 1"]
pub mod ctrl1;
#[doc = "DATA (rw) register accessor: Data Input/Output\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data Input/Output"]
pub mod data;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CLKPRESCALE (rw) register accessor: Clock Pre Scale divide value\n\nYou can [`read`](crate::Reg::read) this register and get [`clkprescale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkprescale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkprescale`] module"]
#[doc(alias = "CLKPRESCALE")]
pub type Clkprescale = crate::Reg<clkprescale::ClkprescaleSpec>;
#[doc = "Clock Pre Scale divide value"]
pub mod clkprescale;
#[doc = "IRQ_ENB (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_enb`] module"]
#[doc(alias = "IRQ_ENB")]
pub type IrqEnb = crate::Reg<irq_enb::IrqEnbSpec>;
#[doc = "Interrupt Enable Register"]
pub mod irq_enb;
pub use irq_enb as irq_raw;
pub use irq_enb as irq_end;
pub use irq_enb as irq_clr;
pub use IrqEnb as IrqRaw;
pub use IrqEnb as IrqEnd;
pub use IrqEnb as IrqClr;
#[doc = "RXFIFOIRQTRG (rw) register accessor: Rx FIFO IRQ Trigger Level\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifoirqtrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifoirqtrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifoirqtrg`] module"]
#[doc(alias = "RXFIFOIRQTRG")]
pub type Rxfifoirqtrg = crate::Reg<rxfifoirqtrg::RxfifoirqtrgSpec>;
#[doc = "Rx FIFO IRQ Trigger Level"]
pub mod rxfifoirqtrg;
#[doc = "TXFIFOIRQTRG (rw) register accessor: Tx FIFO IRQ Trigger Level\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifoirqtrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifoirqtrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifoirqtrg`] module"]
#[doc(alias = "TXFIFOIRQTRG")]
pub type Txfifoirqtrg = crate::Reg<txfifoirqtrg::TxfifoirqtrgSpec>;
#[doc = "Tx FIFO IRQ Trigger Level"]
pub mod txfifoirqtrg;
#[doc = "FIFO_CLR (w) register accessor: Clear FIFO Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_clr`] module"]
#[doc(alias = "FIFO_CLR")]
pub type FifoClr = crate::Reg<fifo_clr::FifoClrSpec>;
#[doc = "Clear FIFO Register"]
pub mod fifo_clr;
#[doc = "STATE (r) register accessor: Internal STATE of SPI Controller\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "Internal STATE of SPI Controller"]
pub mod state;
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`] module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
