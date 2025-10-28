#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    fifo_data: FifoData,
    status: Status,
    irq_enb: IrqEnb,
    irq_raw: IrqRaw,
    irq_end: IrqEnd,
    irq_clr: IrqClr,
    rxfifoirqtrg: Rxfifoirqtrg,
    fifo_clr: FifoClr,
    _reserved9: [u8; 0x0fd8],
    perid: Perid,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - FIFO data"]
    #[inline(always)]
    pub const fn fifo_data(&self) -> &FifoData {
        &self.fifo_data
    }
    #[doc = "0x08 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x0c - Interrupt Enable"]
    #[inline(always)]
    pub const fn irq_enb(&self) -> &IrqEnb {
        &self.irq_enb
    }
    #[doc = "0x10 - Raw Interrupt Status"]
    #[inline(always)]
    pub const fn irq_raw(&self) -> &IrqRaw {
        &self.irq_raw
    }
    #[doc = "0x14 - Enabled Interrupt Status"]
    #[inline(always)]
    pub const fn irq_end(&self) -> &IrqEnd {
        &self.irq_end
    }
    #[doc = "0x18 - Clear Interrupt"]
    #[inline(always)]
    pub const fn irq_clr(&self) -> &IrqClr {
        &self.irq_clr
    }
    #[doc = "0x1c - Receive FIFO Interrupt Trigger Value"]
    #[inline(always)]
    pub const fn rxfifoirqtrg(&self) -> &Rxfifoirqtrg {
        &self.rxfifoirqtrg
    }
    #[doc = "0x20 - FIFO Clear"]
    #[inline(always)]
    pub const fn fifo_clr(&self) -> &FifoClr {
        &self.fifo_clr
    }
    #[doc = "0xffc - Peripheral ID Register"]
    #[inline(always)]
    pub const fn perid(&self) -> &Perid {
        &self.perid
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "FIFO_DATA (r) register accessor: FIFO data\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_data`] module"]
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
#[doc = "RXFIFOIRQTRG (rw) register accessor: Receive FIFO Interrupt Trigger Value\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifoirqtrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifoirqtrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifoirqtrg`] module"]
#[doc(alias = "RXFIFOIRQTRG")]
pub type Rxfifoirqtrg = crate::Reg<rxfifoirqtrg::RxfifoirqtrgSpec>;
#[doc = "Receive FIFO Interrupt Trigger Value"]
pub mod rxfifoirqtrg;
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
