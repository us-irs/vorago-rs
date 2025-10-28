#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl0: Ctrl0,
    ctrl1: Ctrl1,
    fifo_data: FifoData,
    status: Status,
    irq_enb: IrqEnb,
    irq_raw: IrqRaw,
    irq_end: IrqEnd,
    irq_clr: IrqClr,
    txfifoirqtrg: Txfifoirqtrg,
    fifo_clr: FifoClr,
    _reserved10: [u8; 0x07d4],
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
    #[doc = "0x08 - FIFO data"]
    #[inline(always)]
    pub const fn fifo_data(&self) -> &FifoData {
        &self.fifo_data
    }
    #[doc = "0x0c - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - Interrupt Enable"]
    #[inline(always)]
    pub const fn irq_enb(&self) -> &IrqEnb {
        &self.irq_enb
    }
    #[doc = "0x14 - Raw Interrupt Status"]
    #[inline(always)]
    pub const fn irq_raw(&self) -> &IrqRaw {
        &self.irq_raw
    }
    #[doc = "0x18 - Enabled Interrupt Status"]
    #[inline(always)]
    pub const fn irq_end(&self) -> &IrqEnd {
        &self.irq_end
    }
    #[doc = "0x1c - Clear Interrupt"]
    #[inline(always)]
    pub const fn irq_clr(&self) -> &IrqClr {
        &self.irq_clr
    }
    #[doc = "0x20 - Receive FIFO Interrupt Trigger Value"]
    #[inline(always)]
    pub const fn txfifoirqtrg(&self) -> &Txfifoirqtrg {
        &self.txfifoirqtrg
    }
    #[doc = "0x24 - FIFO Clear"]
    #[inline(always)]
    pub const fn fifo_clr(&self) -> &FifoClr {
        &self.fifo_clr
    }
    #[doc = "0x7fc - Peripheral ID Register"]
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
#[doc = "FIFO_DATA (rw) register accessor: FIFO data\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_data`] module"]
#[doc(alias = "FIFO_DATA")]
pub type FifoData = crate::Reg<fifo_data::FifoDataSpec>;
#[doc = "FIFO data"]
pub mod fifo_data;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "IRQ_ENB (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_enb`] module"]
#[doc(alias = "IRQ_ENB")]
pub type IrqEnb = crate::Reg<irq_enb::IrqEnbSpec>;
#[doc = "Interrupt Enable"]
pub mod irq_enb;
#[doc = "IRQ_RAW (r) register accessor: Raw Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_raw`] module"]
#[doc(alias = "IRQ_RAW")]
pub type IrqRaw = crate::Reg<irq_raw::IrqRawSpec>;
#[doc = "Raw Interrupt Status"]
pub mod irq_raw;
#[doc = "IRQ_END (r) register accessor: Enabled Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_end::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_end`] module"]
#[doc(alias = "IRQ_END")]
pub type IrqEnd = crate::Reg<irq_end::IrqEndSpec>;
#[doc = "Enabled Interrupt Status"]
pub mod irq_end;
#[doc = "IRQ_CLR (w) register accessor: Clear Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_clr`] module"]
#[doc(alias = "IRQ_CLR")]
pub type IrqClr = crate::Reg<irq_clr::IrqClrSpec>;
#[doc = "Clear Interrupt"]
pub mod irq_clr;
#[doc = "TXFIFOIRQTRG (rw) register accessor: Receive FIFO Interrupt Trigger Value\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifoirqtrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifoirqtrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifoirqtrg`] module"]
#[doc(alias = "TXFIFOIRQTRG")]
pub type Txfifoirqtrg = crate::Reg<txfifoirqtrg::TxfifoirqtrgSpec>;
#[doc = "Receive FIFO Interrupt Trigger Value"]
pub mod txfifoirqtrg;
#[doc = "FIFO_CLR (rw) register accessor: FIFO Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_clr`] module"]
#[doc(alias = "FIFO_CLR")]
pub type FifoClr = crate::Reg<fifo_clr::FifoClrSpec>;
#[doc = "FIFO Clear"]
pub mod fifo_clr;
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`] module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
