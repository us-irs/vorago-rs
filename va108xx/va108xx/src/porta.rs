#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_datain: [u8; 0x04],
    _reserved_1_datainraw: [u8; 0x04],
    _reserved_2_dataout: [u8; 0x04],
    _reserved_3_dataoutraw: [u8; 0x04],
    _reserved_4_setout: [u8; 0x04],
    _reserved_5_clrout: [u8; 0x04],
    _reserved_6_togout: [u8; 0x04],
    _reserved_7_datamask: [u8; 0x04],
    _reserved_8_dir: [u8; 0x04],
    _reserved_9_pulse: [u8; 0x04],
    _reserved_10_pulsebase: [u8; 0x04],
    _reserved_11_delay: [u8; 0x04],
    _reserved_12_delay: [u8; 0x04],
    irq_sen: IrqSen,
    irq_edge: IrqEdge,
    irq_evt: IrqEvt,
    irq_enb: IrqEnb,
    irq_raw: IrqRaw,
    irq_end: IrqEnd,
    edge_status: EdgeStatus,
    _reserved20: [u8; 0x0fac],
    perid: Perid,
}
impl RegisterBlock {
    #[doc = "0x00 - Data In Register by Byte"]
    #[inline(always)]
    pub const fn datainbyte(&self, n: usize) -> &Datainbyte {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - Data In Register by Byte"]
    #[inline(always)]
    pub fn datainbyte_iter(&self) -> impl Iterator<Item = &Datainbyte> {
        (0..4).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(n).cast() })
    }
    #[doc = "0x00 - Data In Register"]
    #[inline(always)]
    pub const fn datain(&self) -> &Datain {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Data In Raw Register by Byte"]
    #[inline(always)]
    pub const fn datainrawbyte(&self, n: usize) -> &Datainrawbyte {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04 - Data In Raw Register by Byte"]
    #[inline(always)]
    pub fn datainrawbyte_iter(&self) -> impl Iterator<Item = &Datainrawbyte> {
        (0..4)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).add(n).cast() })
    }
    #[doc = "0x04 - Data In Raw Register"]
    #[inline(always)]
    pub const fn datainraw(&self) -> &Datainraw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Data Out Register by Byte"]
    #[inline(always)]
    pub const fn dataoutbyte(&self, n: usize) -> &Dataoutbyte {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08 - Data Out Register by Byte"]
    #[inline(always)]
    pub fn dataoutbyte_iter(&self) -> impl Iterator<Item = &Dataoutbyte> {
        (0..4)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).add(n).cast() })
    }
    #[doc = "0x08 - Data Out Register"]
    #[inline(always)]
    pub const fn dataout(&self) -> &Dataout {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Data Out Register by Byte"]
    #[inline(always)]
    pub const fn dataoutrawbyte(&self, n: usize) -> &Dataoutrawbyte {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c - Data Out Register by Byte"]
    #[inline(always)]
    pub fn dataoutrawbyte_iter(&self) -> impl Iterator<Item = &Dataoutrawbyte> {
        (0..4)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).add(n).cast() })
    }
    #[doc = "0x0c - Data Out Register"]
    #[inline(always)]
    pub const fn dataoutraw(&self) -> &Dataoutraw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x10 - Set Out Register by Byte"]
    #[inline(always)]
    pub const fn setoutbyte(&self, n: usize) -> &Setoutbyte {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10 - Set Out Register by Byte"]
    #[inline(always)]
    pub fn setoutbyte_iter(&self) -> impl Iterator<Item = &Setoutbyte> {
        (0..4)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).add(n).cast() })
    }
    #[doc = "0x10 - Set Out Register"]
    #[inline(always)]
    pub const fn setout(&self) -> &Setout {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x14 - Clear Out Register by Byte"]
    #[inline(always)]
    pub const fn clroutbyte(&self, n: usize) -> &Clroutbyte {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14 - Clear Out Register by Byte"]
    #[inline(always)]
    pub fn clroutbyte_iter(&self) -> impl Iterator<Item = &Clroutbyte> {
        (0..4)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).add(n).cast() })
    }
    #[doc = "0x14 - Clear Out Register"]
    #[inline(always)]
    pub const fn clrout(&self) -> &Clrout {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x18 - Toggle Out Register by Byte"]
    #[inline(always)]
    pub const fn togoutbyte(&self, n: usize) -> &Togoutbyte {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18 - Toggle Out Register by Byte"]
    #[inline(always)]
    pub fn togoutbyte_iter(&self) -> impl Iterator<Item = &Togoutbyte> {
        (0..4)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).add(n).cast() })
    }
    #[doc = "0x18 - Toggle Out Register"]
    #[inline(always)]
    pub const fn togout(&self) -> &Togout {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - Data Out Register by Byte"]
    #[inline(always)]
    pub const fn datamaskbyte(&self, n: usize) -> &Datamaskbyte {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c - Data Out Register by Byte"]
    #[inline(always)]
    pub fn datamaskbyte_iter(&self) -> impl Iterator<Item = &Datamaskbyte> {
        (0..4)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).add(n).cast() })
    }
    #[doc = "0x1c - Data mask Register"]
    #[inline(always)]
    pub const fn datamask(&self) -> &Datamask {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - Direction Register by Byte"]
    #[inline(always)]
    pub const fn dirbyte(&self, n: usize) -> &Dirbyte {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20 - Direction Register by Byte"]
    #[inline(always)]
    pub fn dirbyte_iter(&self) -> impl Iterator<Item = &Dirbyte> {
        (0..4)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).add(n).cast() })
    }
    #[doc = "0x20 - Direction Register (1:Output, 0:Input)"]
    #[inline(always)]
    pub const fn dir(&self) -> &Dir {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - Pulse Mode Register by Byte"]
    #[inline(always)]
    pub const fn pulsebyte(&self, n: usize) -> &Pulsebyte {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x24 - Pulse Mode Register by Byte"]
    #[inline(always)]
    pub fn pulsebyte_iter(&self) -> impl Iterator<Item = &Pulsebyte> {
        (0..4)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).add(n).cast() })
    }
    #[doc = "0x24 - Pulse Mode Register"]
    #[inline(always)]
    pub const fn pulse(&self) -> &Pulse {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x28 - Pulse Base Mode Register by Byte"]
    #[inline(always)]
    pub const fn pulsebasebyte(&self, n: usize) -> &Pulsebasebyte {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x28 - Pulse Base Mode Register by Byte"]
    #[inline(always)]
    pub fn pulsebasebyte_iter(&self) -> impl Iterator<Item = &Pulsebasebyte> {
        (0..4)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).add(n).cast() })
    }
    #[doc = "0x28 - Pulse Base Value Register"]
    #[inline(always)]
    pub const fn pulsebase(&self) -> &Pulsebase {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - Delay1 Register by Byte"]
    #[inline(always)]
    pub const fn delay1byte(&self, n: usize) -> &Delay1byte {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c - Delay1 Register by Byte"]
    #[inline(always)]
    pub fn delay1byte_iter(&self) -> impl Iterator<Item = &Delay1byte> {
        (0..4)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).add(n).cast() })
    }
    #[doc = "0x2c - Delay1 Register"]
    #[inline(always)]
    pub const fn delay1(&self) -> &Delay1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x30 - Delay2 Register by Byte"]
    #[inline(always)]
    pub const fn delay2byte(&self, n: usize) -> &Delay2byte {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30 - Delay2 Register by Byte"]
    #[inline(always)]
    pub fn delay2byte_iter(&self) -> impl Iterator<Item = &Delay2byte> {
        (0..4)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).add(n).cast() })
    }
    #[doc = "0x30 - Delay2 Register"]
    #[inline(always)]
    pub const fn delay2(&self) -> &Delay2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x34 - Interrupt Sense Register (1:Level Sensitive, 0:Edge Sensitive)"]
    #[inline(always)]
    pub const fn irq_sen(&self) -> &IrqSen {
        &self.irq_sen
    }
    #[doc = "0x38 - Interrupt Both Edge Register (1:Both Edges, 0:Single Edge)"]
    #[inline(always)]
    pub const fn irq_edge(&self) -> &IrqEdge {
        &self.irq_edge
    }
    #[doc = "0x3c - Interrupt Event Register (1:HighLevel/L->H Edge, 0:LowLevel/H->L Edge)"]
    #[inline(always)]
    pub const fn irq_evt(&self) -> &IrqEvt {
        &self.irq_evt
    }
    #[doc = "0x40 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn irq_enb(&self) -> &IrqEnb {
        &self.irq_enb
    }
    #[doc = "0x44 - Raw Interrupt Status"]
    #[inline(always)]
    pub const fn irq_raw(&self) -> &IrqRaw {
        &self.irq_raw
    }
    #[doc = "0x48 - Masked Interrupt Status"]
    #[inline(always)]
    pub const fn irq_end(&self) -> &IrqEnd {
        &self.irq_end
    }
    #[doc = "0x4c - Edge Status Register"]
    #[inline(always)]
    pub const fn edge_status(&self) -> &EdgeStatus {
        &self.edge_status
    }
    #[doc = "0xffc - Peripheral ID Register"]
    #[inline(always)]
    pub const fn perid(&self) -> &Perid {
        &self.perid
    }
}
#[doc = "DATAIN (r) register accessor: Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`datain::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datain`] module"]
#[doc(alias = "DATAIN")]
pub type Datain = crate::Reg<datain::DatainSpec>;
#[doc = "Data In Register"]
pub mod datain;
#[doc = "DATAINBYTE (r) register accessor: Data In Register by Byte\n\nYou can [`read`](crate::Reg::read) this register and get [`datainbyte::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datainbyte`] module"]
#[doc(alias = "DATAINBYTE")]
pub type Datainbyte = crate::Reg<datainbyte::DatainbyteSpec>;
#[doc = "Data In Register by Byte"]
pub mod datainbyte;
pub use datain as datainraw;
pub use datainbyte as datainrawbyte;
pub use Datain as Datainraw;
pub use Datainbyte as Datainrawbyte;
#[doc = "DATAOUT (w) register accessor: Data Out Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataout::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataout`] module"]
#[doc(alias = "DATAOUT")]
pub type Dataout = crate::Reg<dataout::DataoutSpec>;
#[doc = "Data Out Register"]
pub mod dataout;
#[doc = "DATAOUTBYTE (w) register accessor: Data Out Register by Byte\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataoutbyte::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dataoutbyte`] module"]
#[doc(alias = "DATAOUTBYTE")]
pub type Dataoutbyte = crate::Reg<dataoutbyte::DataoutbyteSpec>;
#[doc = "Data Out Register by Byte"]
pub mod dataoutbyte;
pub use dataout as dataoutraw;
pub use dataout as setout;
pub use dataout as clrout;
pub use dataout as togout;
pub use dataoutbyte as dataoutrawbyte;
pub use dataoutbyte as setoutbyte;
pub use dataoutbyte as clroutbyte;
pub use dataoutbyte as togoutbyte;
pub use Dataout as Dataoutraw;
pub use Dataout as Setout;
pub use Dataout as Clrout;
pub use Dataout as Togout;
pub use Dataoutbyte as Dataoutrawbyte;
pub use Dataoutbyte as Setoutbyte;
pub use Dataoutbyte as Clroutbyte;
pub use Dataoutbyte as Togoutbyte;
#[doc = "DATAMASK (rw) register accessor: Data mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`datamask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datamask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datamask`] module"]
#[doc(alias = "DATAMASK")]
pub type Datamask = crate::Reg<datamask::DatamaskSpec>;
#[doc = "Data mask Register"]
pub mod datamask;
#[doc = "DATAMASKBYTE (rw) register accessor: Data Out Register by Byte\n\nYou can [`read`](crate::Reg::read) this register and get [`datamaskbyte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datamaskbyte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datamaskbyte`] module"]
#[doc(alias = "DATAMASKBYTE")]
pub type Datamaskbyte = crate::Reg<datamaskbyte::DatamaskbyteSpec>;
#[doc = "Data Out Register by Byte"]
pub mod datamaskbyte;
pub use datamask as dir;
pub use datamask as pulse;
pub use datamask as pulsebase;
pub use datamask as delay1;
pub use datamask as delay2;
pub use datamaskbyte as dirbyte;
pub use datamaskbyte as pulsebyte;
pub use datamaskbyte as pulsebasebyte;
pub use datamaskbyte as delay1byte;
pub use datamaskbyte as delay2byte;
pub use Datamask as Dir;
pub use Datamask as Pulse;
pub use Datamask as Pulsebase;
pub use Datamask as Delay1;
pub use Datamask as Delay2;
pub use Datamaskbyte as Dirbyte;
pub use Datamaskbyte as Pulsebyte;
pub use Datamaskbyte as Pulsebasebyte;
pub use Datamaskbyte as Delay1byte;
pub use Datamaskbyte as Delay2byte;
#[doc = "IRQ_SEN (rw) register accessor: Interrupt Sense Register (1:Level Sensitive, 0:Edge Sensitive)\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_sen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_sen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_sen`] module"]
#[doc(alias = "IRQ_SEN")]
pub type IrqSen = crate::Reg<irq_sen::IrqSenSpec>;
#[doc = "Interrupt Sense Register (1:Level Sensitive, 0:Edge Sensitive)"]
pub mod irq_sen;
#[doc = "IRQ_EDGE (rw) register accessor: Interrupt Both Edge Register (1:Both Edges, 0:Single Edge)\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_edge::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_edge::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_edge`] module"]
#[doc(alias = "IRQ_EDGE")]
pub type IrqEdge = crate::Reg<irq_edge::IrqEdgeSpec>;
#[doc = "Interrupt Both Edge Register (1:Both Edges, 0:Single Edge)"]
pub mod irq_edge;
#[doc = "IRQ_EVT (rw) register accessor: Interrupt Event Register (1:HighLevel/L->H Edge, 0:LowLevel/H->L Edge)\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_evt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_evt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_evt`] module"]
#[doc(alias = "IRQ_EVT")]
pub type IrqEvt = crate::Reg<irq_evt::IrqEvtSpec>;
#[doc = "Interrupt Event Register (1:HighLevel/L->H Edge, 0:LowLevel/H->L Edge)"]
pub mod irq_evt;
#[doc = "IRQ_ENB (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_enb`] module"]
#[doc(alias = "IRQ_ENB")]
pub type IrqEnb = crate::Reg<irq_enb::IrqEnbSpec>;
#[doc = "Interrupt Enable Register"]
pub mod irq_enb;
#[doc = "IRQ_RAW (r) register accessor: Raw Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_raw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_raw`] module"]
#[doc(alias = "IRQ_RAW")]
pub type IrqRaw = crate::Reg<irq_raw::IrqRawSpec>;
#[doc = "Raw Interrupt Status"]
pub mod irq_raw;
#[doc = "IRQ_END (r) register accessor: Masked Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_end::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_end`] module"]
#[doc(alias = "IRQ_END")]
pub type IrqEnd = crate::Reg<irq_end::IrqEndSpec>;
#[doc = "Masked Interrupt Status"]
pub mod irq_end;
#[doc = "EDGE_STATUS (rw) register accessor: Edge Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`edge_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edge_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edge_status`] module"]
#[doc(alias = "EDGE_STATUS")]
pub type EdgeStatus = crate::Reg<edge_status::EdgeStatusSpec>;
#[doc = "Edge Status Register"]
pub mod edge_status;
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`] module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
