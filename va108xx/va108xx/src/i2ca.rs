#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    clkscale: Clkscale,
    words: Words,
    address: Address,
    data: Data,
    cmd: Cmd,
    status: Status,
    state: State,
    txcount: Txcount,
    rxcount: Rxcount,
    irq_enb: IrqEnb,
    irq_raw: IrqRaw,
    irq_end: IrqEnd,
    irq_clr: IrqClr,
    rxfifoirqtrg: Rxfifoirqtrg,
    txfifoirqtrg: Txfifoirqtrg,
    fifo_clr: FifoClr,
    tmconfig: Tmconfig,
    clktolimit: Clktolimit,
    _reserved19: [u8; 0xb4],
    s0_ctrl: S0Ctrl,
    s0_maxwords: S0Maxwords,
    s0_address: S0Address,
    s0_addressmask: S0Addressmask,
    s0_data: S0Data,
    s0_lastaddress: S0Lastaddress,
    s0_status: S0Status,
    s0_state: S0State,
    s0_txcount: S0Txcount,
    s0_rxcount: S0Rxcount,
    s0_irq_enb: S0IrqEnb,
    s0_irq_raw: S0IrqRaw,
    s0_irq_end: S0IrqEnd,
    s0_irq_clr: S0IrqClr,
    s0_rxfifoirqtrg: S0Rxfifoirqtrg,
    s0_txfifoirqtrg: S0Txfifoirqtrg,
    s0_fifo_clr: S0FifoClr,
    s0_addressb: S0Addressb,
    s0_addressmaskb: S0Addressmaskb,
    _reserved38: [u8; 0x0eb0],
    perid: Perid,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Clock Scale divide value"]
    #[inline(always)]
    pub const fn clkscale(&self) -> &Clkscale {
        &self.clkscale
    }
    #[doc = "0x08 - Word Count value"]
    #[inline(always)]
    pub const fn words(&self) -> &Words {
        &self.words
    }
    #[doc = "0x0c - I2C Address value"]
    #[inline(always)]
    pub const fn address(&self) -> &Address {
        &self.address
    }
    #[doc = "0x10 - Data Input/Output"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x14 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x18 - I2C Controller Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1c - Internal STATE of I2C Master Controller"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x20 - TX Count Register"]
    #[inline(always)]
    pub const fn txcount(&self) -> &Txcount {
        &self.txcount
    }
    #[doc = "0x24 - RX Count Register"]
    #[inline(always)]
    pub const fn rxcount(&self) -> &Rxcount {
        &self.rxcount
    }
    #[doc = "0x28 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn irq_enb(&self) -> &IrqEnb {
        &self.irq_enb
    }
    #[doc = "0x2c - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn irq_raw(&self) -> &IrqRaw {
        &self.irq_raw
    }
    #[doc = "0x30 - Enabled Interrupt Status Register"]
    #[inline(always)]
    pub const fn irq_end(&self) -> &IrqEnd {
        &self.irq_end
    }
    #[doc = "0x34 - Clear Interrupt Status Register"]
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
    #[doc = "0x40 - Clear FIFO Register"]
    #[inline(always)]
    pub const fn fifo_clr(&self) -> &FifoClr {
        &self.fifo_clr
    }
    #[doc = "0x44 - Timing Config Register"]
    #[inline(always)]
    pub const fn tmconfig(&self) -> &Tmconfig {
        &self.tmconfig
    }
    #[doc = "0x48 - Clock Low Timeout Limit Register"]
    #[inline(always)]
    pub const fn clktolimit(&self) -> &Clktolimit {
        &self.clktolimit
    }
    #[doc = "0x100 - Slave Control Register"]
    #[inline(always)]
    pub const fn s0_ctrl(&self) -> &S0Ctrl {
        &self.s0_ctrl
    }
    #[doc = "0x104 - Slave MaxWords Register"]
    #[inline(always)]
    pub const fn s0_maxwords(&self) -> &S0Maxwords {
        &self.s0_maxwords
    }
    #[doc = "0x108 - Slave I2C Address Value"]
    #[inline(always)]
    pub const fn s0_address(&self) -> &S0Address {
        &self.s0_address
    }
    #[doc = "0x10c - Slave I2C Address Mask value"]
    #[inline(always)]
    pub const fn s0_addressmask(&self) -> &S0Addressmask {
        &self.s0_addressmask
    }
    #[doc = "0x110 - Slave Data Input/Output"]
    #[inline(always)]
    pub const fn s0_data(&self) -> &S0Data {
        &self.s0_data
    }
    #[doc = "0x114 - Slave I2C Last Address value"]
    #[inline(always)]
    pub const fn s0_lastaddress(&self) -> &S0Lastaddress {
        &self.s0_lastaddress
    }
    #[doc = "0x118 - Slave I2C Controller Status Register"]
    #[inline(always)]
    pub const fn s0_status(&self) -> &S0Status {
        &self.s0_status
    }
    #[doc = "0x11c - Internal STATE of I2C Slave Controller"]
    #[inline(always)]
    pub const fn s0_state(&self) -> &S0State {
        &self.s0_state
    }
    #[doc = "0x120 - Slave TX Count Register"]
    #[inline(always)]
    pub const fn s0_txcount(&self) -> &S0Txcount {
        &self.s0_txcount
    }
    #[doc = "0x124 - Slave RX Count Register"]
    #[inline(always)]
    pub const fn s0_rxcount(&self) -> &S0Rxcount {
        &self.s0_rxcount
    }
    #[doc = "0x128 - Slave Interrupt Enable Register"]
    #[inline(always)]
    pub const fn s0_irq_enb(&self) -> &S0IrqEnb {
        &self.s0_irq_enb
    }
    #[doc = "0x12c - Slave Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn s0_irq_raw(&self) -> &S0IrqRaw {
        &self.s0_irq_raw
    }
    #[doc = "0x130 - Slave Enabled Interrupt Status Register"]
    #[inline(always)]
    pub const fn s0_irq_end(&self) -> &S0IrqEnd {
        &self.s0_irq_end
    }
    #[doc = "0x134 - Slave Clear Interrupt Status Register"]
    #[inline(always)]
    pub const fn s0_irq_clr(&self) -> &S0IrqClr {
        &self.s0_irq_clr
    }
    #[doc = "0x138 - Slave Rx FIFO IRQ Trigger Level"]
    #[inline(always)]
    pub const fn s0_rxfifoirqtrg(&self) -> &S0Rxfifoirqtrg {
        &self.s0_rxfifoirqtrg
    }
    #[doc = "0x13c - Slave Tx FIFO IRQ Trigger Level"]
    #[inline(always)]
    pub const fn s0_txfifoirqtrg(&self) -> &S0Txfifoirqtrg {
        &self.s0_txfifoirqtrg
    }
    #[doc = "0x140 - Slave Clear FIFO Register"]
    #[inline(always)]
    pub const fn s0_fifo_clr(&self) -> &S0FifoClr {
        &self.s0_fifo_clr
    }
    #[doc = "0x144 - Slave I2C Address B Value"]
    #[inline(always)]
    pub const fn s0_addressb(&self) -> &S0Addressb {
        &self.s0_addressb
    }
    #[doc = "0x148 - Slave I2C Address B Mask value"]
    #[inline(always)]
    pub const fn s0_addressmaskb(&self) -> &S0Addressmaskb {
        &self.s0_addressmaskb
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
#[doc = "CLKSCALE (rw) register accessor: Clock Scale divide value\n\nYou can [`read`](crate::Reg::read) this register and get [`clkscale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkscale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkscale`] module"]
#[doc(alias = "CLKSCALE")]
pub type Clkscale = crate::Reg<clkscale::ClkscaleSpec>;
#[doc = "Clock Scale divide value"]
pub mod clkscale;
#[doc = "WORDS (rw) register accessor: Word Count value\n\nYou can [`read`](crate::Reg::read) this register and get [`words::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`words::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@words`] module"]
#[doc(alias = "WORDS")]
pub type Words = crate::Reg<words::WordsSpec>;
#[doc = "Word Count value"]
pub mod words;
#[doc = "ADDRESS (rw) register accessor: I2C Address value\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`] module"]
#[doc(alias = "ADDRESS")]
pub type Address = crate::Reg<address::AddressSpec>;
#[doc = "I2C Address value"]
pub mod address;
#[doc = "DATA (rw) register accessor: Data Input/Output\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data Input/Output"]
pub mod data;
#[doc = "CMD (rw) register accessor: Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: I2C Controller Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "I2C Controller Status Register"]
pub mod status;
#[doc = "STATE (r) register accessor: Internal STATE of I2C Master Controller\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "Internal STATE of I2C Master Controller"]
pub mod state;
#[doc = "TXCOUNT (r) register accessor: TX Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txcount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcount`] module"]
#[doc(alias = "TXCOUNT")]
pub type Txcount = crate::Reg<txcount::TxcountSpec>;
#[doc = "TX Count Register"]
pub mod txcount;
#[doc = "RXCOUNT (r) register accessor: RX Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcount`] module"]
#[doc(alias = "RXCOUNT")]
pub type Rxcount = crate::Reg<rxcount::RxcountSpec>;
#[doc = "RX Count Register"]
pub mod rxcount;
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
#[doc = "TMCONFIG (rw) register accessor: Timing Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmconfig`] module"]
#[doc(alias = "TMCONFIG")]
pub type Tmconfig = crate::Reg<tmconfig::TmconfigSpec>;
#[doc = "Timing Config Register"]
pub mod tmconfig;
#[doc = "CLKTOLIMIT (rw) register accessor: Clock Low Timeout Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clktolimit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clktolimit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clktolimit`] module"]
#[doc(alias = "CLKTOLIMIT")]
pub type Clktolimit = crate::Reg<clktolimit::ClktolimitSpec>;
#[doc = "Clock Low Timeout Limit Register"]
pub mod clktolimit;
#[doc = "S0_CTRL (rw) register accessor: Slave Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_ctrl`] module"]
#[doc(alias = "S0_CTRL")]
pub type S0Ctrl = crate::Reg<s0_ctrl::S0CtrlSpec>;
#[doc = "Slave Control Register"]
pub mod s0_ctrl;
#[doc = "S0_MAXWORDS (rw) register accessor: Slave MaxWords Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_maxwords::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_maxwords::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_maxwords`] module"]
#[doc(alias = "S0_MAXWORDS")]
pub type S0Maxwords = crate::Reg<s0_maxwords::S0MaxwordsSpec>;
#[doc = "Slave MaxWords Register"]
pub mod s0_maxwords;
#[doc = "S0_ADDRESS (rw) register accessor: Slave I2C Address Value\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_address`] module"]
#[doc(alias = "S0_ADDRESS")]
pub type S0Address = crate::Reg<s0_address::S0AddressSpec>;
#[doc = "Slave I2C Address Value"]
pub mod s0_address;
#[doc = "S0_ADDRESSMASK (rw) register accessor: Slave I2C Address Mask value\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_addressmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_addressmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_addressmask`] module"]
#[doc(alias = "S0_ADDRESSMASK")]
pub type S0Addressmask = crate::Reg<s0_addressmask::S0AddressmaskSpec>;
#[doc = "Slave I2C Address Mask value"]
pub mod s0_addressmask;
#[doc = "S0_DATA (rw) register accessor: Slave Data Input/Output\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_data`] module"]
#[doc(alias = "S0_DATA")]
pub type S0Data = crate::Reg<s0_data::S0DataSpec>;
#[doc = "Slave Data Input/Output"]
pub mod s0_data;
#[doc = "S0_LASTADDRESS (r) register accessor: Slave I2C Last Address value\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_lastaddress::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_lastaddress`] module"]
#[doc(alias = "S0_LASTADDRESS")]
pub type S0Lastaddress = crate::Reg<s0_lastaddress::S0LastaddressSpec>;
#[doc = "Slave I2C Last Address value"]
pub mod s0_lastaddress;
#[doc = "S0_STATUS (r) register accessor: Slave I2C Controller Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_status`] module"]
#[doc(alias = "S0_STATUS")]
pub type S0Status = crate::Reg<s0_status::S0StatusSpec>;
#[doc = "Slave I2C Controller Status Register"]
pub mod s0_status;
#[doc = "S0_STATE (r) register accessor: Internal STATE of I2C Slave Controller\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_state`] module"]
#[doc(alias = "S0_STATE")]
pub type S0State = crate::Reg<s0_state::S0StateSpec>;
#[doc = "Internal STATE of I2C Slave Controller"]
pub mod s0_state;
#[doc = "S0_TXCOUNT (r) register accessor: Slave TX Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_txcount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_txcount`] module"]
#[doc(alias = "S0_TXCOUNT")]
pub type S0Txcount = crate::Reg<s0_txcount::S0TxcountSpec>;
#[doc = "Slave TX Count Register"]
pub mod s0_txcount;
#[doc = "S0_RXCOUNT (r) register accessor: Slave RX Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_rxcount::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_rxcount`] module"]
#[doc(alias = "S0_RXCOUNT")]
pub type S0Rxcount = crate::Reg<s0_rxcount::S0RxcountSpec>;
#[doc = "Slave RX Count Register"]
pub mod s0_rxcount;
#[doc = "S0_IRQ_ENB (rw) register accessor: Slave Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_irq_enb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_irq_enb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_irq_enb`] module"]
#[doc(alias = "S0_IRQ_ENB")]
pub type S0IrqEnb = crate::Reg<s0_irq_enb::S0IrqEnbSpec>;
#[doc = "Slave Interrupt Enable Register"]
pub mod s0_irq_enb;
pub use s0_irq_enb as s0_irq_raw;
pub use s0_irq_enb as s0_irq_end;
pub use s0_irq_enb as s0_irq_clr;
pub use S0IrqEnb as S0IrqRaw;
pub use S0IrqEnb as S0IrqEnd;
pub use S0IrqEnb as S0IrqClr;
#[doc = "S0_RXFIFOIRQTRG (rw) register accessor: Slave Rx FIFO IRQ Trigger Level\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_rxfifoirqtrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_rxfifoirqtrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_rxfifoirqtrg`] module"]
#[doc(alias = "S0_RXFIFOIRQTRG")]
pub type S0Rxfifoirqtrg = crate::Reg<s0_rxfifoirqtrg::S0RxfifoirqtrgSpec>;
#[doc = "Slave Rx FIFO IRQ Trigger Level"]
pub mod s0_rxfifoirqtrg;
#[doc = "S0_TXFIFOIRQTRG (rw) register accessor: Slave Tx FIFO IRQ Trigger Level\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_txfifoirqtrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_txfifoirqtrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_txfifoirqtrg`] module"]
#[doc(alias = "S0_TXFIFOIRQTRG")]
pub type S0Txfifoirqtrg = crate::Reg<s0_txfifoirqtrg::S0TxfifoirqtrgSpec>;
#[doc = "Slave Tx FIFO IRQ Trigger Level"]
pub mod s0_txfifoirqtrg;
#[doc = "S0_FIFO_CLR (w) register accessor: Slave Clear FIFO Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_fifo_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_fifo_clr`] module"]
#[doc(alias = "S0_FIFO_CLR")]
pub type S0FifoClr = crate::Reg<s0_fifo_clr::S0FifoClrSpec>;
#[doc = "Slave Clear FIFO Register"]
pub mod s0_fifo_clr;
#[doc = "S0_ADDRESSB (rw) register accessor: Slave I2C Address B Value\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_addressb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_addressb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_addressb`] module"]
#[doc(alias = "S0_ADDRESSB")]
pub type S0Addressb = crate::Reg<s0_addressb::S0AddressbSpec>;
#[doc = "Slave I2C Address B Value"]
pub mod s0_addressb;
#[doc = "S0_ADDRESSMASKB (rw) register accessor: Slave I2C Address B Mask value\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_addressmaskb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_addressmaskb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0_addressmaskb`] module"]
#[doc(alias = "S0_ADDRESSMASKB")]
pub type S0Addressmaskb = crate::Reg<s0_addressmaskb::S0AddressmaskbSpec>;
#[doc = "Slave I2C Address B Mask value"]
pub mod s0_addressmaskb;
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`] module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
