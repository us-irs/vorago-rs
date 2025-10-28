#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    data: Data,
    enable: Enable,
    ctrl: Ctrl,
    clkscale: Clkscale,
    rxstatus: Rxstatus,
    txstatus: Txstatus,
    fifo_clr: FifoClr,
    txbreak: Txbreak,
    addr9: Addr9,
    addr9mask: Addr9mask,
    irq_enb: IrqEnb,
    irq_raw: IrqRaw,
    irq_end: IrqEnd,
    irq_clr: IrqClr,
    rxfifoirqtrg: Rxfifoirqtrg,
    txfifoirqtrg: Txfifoirqtrg,
    rxfifortstrg: Rxfifortstrg,
    state: State,
    _reserved18: [u8; 0x0fb4],
    perid: Perid,
}
impl RegisterBlock {
    #[doc = "0x00 - Data In/Out Register"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x04 - Enable Register"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x08 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - Clock Scale Register"]
    #[inline(always)]
    pub const fn clkscale(&self) -> &Clkscale {
        &self.clkscale
    }
    #[doc = "0x10 - Status Register"]
    #[inline(always)]
    pub const fn rxstatus(&self) -> &Rxstatus {
        &self.rxstatus
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn txstatus(&self) -> &Txstatus {
        &self.txstatus
    }
    #[doc = "0x18 - Clear FIFO Register"]
    #[inline(always)]
    pub const fn fifo_clr(&self) -> &FifoClr {
        &self.fifo_clr
    }
    #[doc = "0x1c - Break Transmit Register"]
    #[inline(always)]
    pub const fn txbreak(&self) -> &Txbreak {
        &self.txbreak
    }
    #[doc = "0x20 - Address9 Register"]
    #[inline(always)]
    pub const fn addr9(&self) -> &Addr9 {
        &self.addr9
    }
    #[doc = "0x24 - Address9 Mask Register"]
    #[inline(always)]
    pub const fn addr9mask(&self) -> &Addr9mask {
        &self.addr9mask
    }
    #[doc = "0x28 - IRQ Enable Register"]
    #[inline(always)]
    pub const fn irq_enb(&self) -> &IrqEnb {
        &self.irq_enb
    }
    #[doc = "0x2c - IRQ Raw Status Register"]
    #[inline(always)]
    pub const fn irq_raw(&self) -> &IrqRaw {
        &self.irq_raw
    }
    #[doc = "0x30 - IRQ Enabled Status Register"]
    #[inline(always)]
    pub const fn irq_end(&self) -> &IrqEnd {
        &self.irq_end
    }
    #[doc = "0x34 - IRQ Clear Status Register"]
    #[inline(always)]
    pub const fn irq_clr(&self) -> &IrqClr {
        &self.irq_clr
    }
    #[doc = "0x38 - Rx FIFO IRQ Trigger Level"]
    #[inline(always)]
    pub const fn rxfifoirqtrg(&self) -> &Rxfifoirqtrg {
        &self.rxfifoirqtrg
    }
    #[doc = "0x3c - Tx FIFO IRQ Trigger Level"]
    #[inline(always)]
    pub const fn txfifoirqtrg(&self) -> &Txfifoirqtrg {
        &self.txfifoirqtrg
    }
    #[doc = "0x40 - Rx FIFO RTS Trigger Level"]
    #[inline(always)]
    pub const fn rxfifortstrg(&self) -> &Rxfifortstrg {
        &self.rxfifortstrg
    }
    #[doc = "0x44 - Internal STATE of UART Controller"]
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
#[doc = "DATA (rw) register accessor: Data In/Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data In/Out Register"]
pub mod data;
#[doc = "ENABLE (rw) register accessor: Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable Register"]
pub mod enable;
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CLKSCALE (rw) register accessor: Clock Scale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkscale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkscale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkscale`] module"]
#[doc(alias = "CLKSCALE")]
pub type Clkscale = crate::Reg<clkscale::ClkscaleSpec>;
#[doc = "Clock Scale Register"]
pub mod clkscale;
#[doc = "RXSTATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxstatus`] module"]
#[doc(alias = "RXSTATUS")]
pub type Rxstatus = crate::Reg<rxstatus::RxstatusSpec>;
#[doc = "Status Register"]
pub mod rxstatus;
#[doc = "TXSTATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txstatus`] module"]
#[doc(alias = "TXSTATUS")]
pub type Txstatus = crate::Reg<txstatus::TxstatusSpec>;
#[doc = "Status Register"]
pub mod txstatus;
#[doc = "FIFO_CLR (w) register accessor: Clear FIFO Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_clr`] module"]
#[doc(alias = "FIFO_CLR")]
pub type FifoClr = crate::Reg<fifo_clr::FifoClrSpec>;
#[doc = "Clear FIFO Register"]
pub mod fifo_clr;
#[doc = "TXBREAK (w) register accessor: Break Transmit Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbreak::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbreak`] module"]
#[doc(alias = "TXBREAK")]
pub type Txbreak = crate::Reg<txbreak::TxbreakSpec>;
#[doc = "Break Transmit Register"]
pub mod txbreak;
#[doc = "ADDR9 (rw) register accessor: Address9 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr9`] module"]
#[doc(alias = "ADDR9")]
pub type Addr9 = crate::Reg<addr9::Addr9Spec>;
#[doc = "Address9 Register"]
pub mod addr9;
#[doc = "ADDR9MASK (rw) register accessor: Address9 Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr9mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr9mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr9mask`] module"]
#[doc(alias = "ADDR9MASK")]
pub type Addr9mask = crate::Reg<addr9mask::Addr9maskSpec>;
#[doc = "Address9 Mask Register"]
pub mod addr9mask;
#[doc = "IRQ_ENB (rw) register accessor: IRQ Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_enb`] module"]
#[doc(alias = "IRQ_ENB")]
pub type IrqEnb = crate::Reg<irq_enb::IrqEnbSpec>;
#[doc = "IRQ Enable Register"]
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
#[doc = "RXFIFORTSTRG (rw) register accessor: Rx FIFO RTS Trigger Level\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifortstrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifortstrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifortstrg`] module"]
#[doc(alias = "RXFIFORTSTRG")]
pub type Rxfifortstrg = crate::Reg<rxfifortstrg::RxfifortstrgSpec>;
#[doc = "Rx FIFO RTS Trigger Level"]
pub mod rxfifortstrg;
#[doc = "STATE (r) register accessor: Internal STATE of UART Controller\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "Internal STATE of UART Controller"]
pub mod state;
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`] module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
