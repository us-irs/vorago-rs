#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    status: Status,
    cfg: Cfg,
    ctrl_base_ptr: CtrlBasePtr,
    alt_ctrl_base_ptr: AltCtrlBasePtr,
    waitonreq_status: WaitonreqStatus,
    chnl_sw_request: ChnlSwRequest,
    chnl_useburst_set: ChnlUseburstSet,
    chnl_useburst_clr: ChnlUseburstClr,
    chnl_req_mask_set: ChnlReqMaskSet,
    chnl_req_mask_clr: ChnlReqMaskClr,
    chnl_enable_set: ChnlEnableSet,
    chnl_enable_clr: ChnlEnableClr,
    chnl_pri_alt_set: ChnlPriAltSet,
    chnl_pri_alt_clr: ChnlPriAltClr,
    chnl_priority_set: ChnlPrioritySet,
    chnl_priority_clr: ChnlPriorityClr,
    _reserved16: [u8; 0x0c],
    err_clr: ErrClr,
    _reserved17: [u8; 0x0db0],
    integration_cfg: IntegrationCfg,
    _reserved18: [u8; 0x04],
    stall_status: StallStatus,
    _reserved19: [u8; 0x04],
    dma_req_status: DmaReqStatus,
    _reserved20: [u8; 0x04],
    dma_sreq_status: DmaSreqStatus,
    _reserved21: [u8; 0x04],
    dma_done_set: DmaDoneSet,
    dma_done_clr: DmaDoneClr,
    dma_active_set: DmaActiveSet,
    dma_active_clr: DmaActiveClr,
    _reserved25: [u8; 0x18],
    err_set: ErrSet,
    _reserved26: [u8; 0x0184],
    periph_id_4: PeriphId4,
    _reserved27: [u8; 0x0c],
    periph_id_0: PeriphId0,
    periph_id_1: PeriphId1,
    periph_id_2: PeriphId2,
    periph_id_3: PeriphId3,
    primecell_id_0: PrimecellId0,
    primecell_id_1: PrimecellId1,
    primecell_id_2: PrimecellId2,
    primecell_id_3: PrimecellId3,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x04 - DMA Configuration"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x08 - Base Pointer for DMA Control Registers"]
    #[inline(always)]
    pub const fn ctrl_base_ptr(&self) -> &CtrlBasePtr {
        &self.ctrl_base_ptr
    }
    #[doc = "0x0c - DMA Channel alternate control data base pointer"]
    #[inline(always)]
    pub const fn alt_ctrl_base_ptr(&self) -> &AltCtrlBasePtr {
        &self.alt_ctrl_base_ptr
    }
    #[doc = "0x10 - DMA channel wait on request status"]
    #[inline(always)]
    pub const fn waitonreq_status(&self) -> &WaitonreqStatus {
        &self.waitonreq_status
    }
    #[doc = "0x14 - DMA channel software request"]
    #[inline(always)]
    pub const fn chnl_sw_request(&self) -> &ChnlSwRequest {
        &self.chnl_sw_request
    }
    #[doc = "0x18 - DMA channel useburst set"]
    #[inline(always)]
    pub const fn chnl_useburst_set(&self) -> &ChnlUseburstSet {
        &self.chnl_useburst_set
    }
    #[doc = "0x1c - DMA channel useburst clear"]
    #[inline(always)]
    pub const fn chnl_useburst_clr(&self) -> &ChnlUseburstClr {
        &self.chnl_useburst_clr
    }
    #[doc = "0x20 - DMA channel request mask set"]
    #[inline(always)]
    pub const fn chnl_req_mask_set(&self) -> &ChnlReqMaskSet {
        &self.chnl_req_mask_set
    }
    #[doc = "0x24 - DMA channel request mask clear"]
    #[inline(always)]
    pub const fn chnl_req_mask_clr(&self) -> &ChnlReqMaskClr {
        &self.chnl_req_mask_clr
    }
    #[doc = "0x28 - DMA channel enable set"]
    #[inline(always)]
    pub const fn chnl_enable_set(&self) -> &ChnlEnableSet {
        &self.chnl_enable_set
    }
    #[doc = "0x2c - DMA channel enable clear"]
    #[inline(always)]
    pub const fn chnl_enable_clr(&self) -> &ChnlEnableClr {
        &self.chnl_enable_clr
    }
    #[doc = "0x30 - DMA channel primary alternate set"]
    #[inline(always)]
    pub const fn chnl_pri_alt_set(&self) -> &ChnlPriAltSet {
        &self.chnl_pri_alt_set
    }
    #[doc = "0x34 - DMA channel primary alternate clear"]
    #[inline(always)]
    pub const fn chnl_pri_alt_clr(&self) -> &ChnlPriAltClr {
        &self.chnl_pri_alt_clr
    }
    #[doc = "0x38 - DMA channel priority set"]
    #[inline(always)]
    pub const fn chnl_priority_set(&self) -> &ChnlPrioritySet {
        &self.chnl_priority_set
    }
    #[doc = "0x3c - DMA channel priority clear"]
    #[inline(always)]
    pub const fn chnl_priority_clr(&self) -> &ChnlPriorityClr {
        &self.chnl_priority_clr
    }
    #[doc = "0x4c - DMA bus error clear"]
    #[inline(always)]
    pub const fn err_clr(&self) -> &ErrClr {
        &self.err_clr
    }
    #[doc = "0xe00 - DMA integration configuration"]
    #[inline(always)]
    pub const fn integration_cfg(&self) -> &IntegrationCfg {
        &self.integration_cfg
    }
    #[doc = "0xe08 - DMA stall status"]
    #[inline(always)]
    pub const fn stall_status(&self) -> &StallStatus {
        &self.stall_status
    }
    #[doc = "0xe10 - DMA Configuration"]
    #[inline(always)]
    pub const fn dma_req_status(&self) -> &DmaReqStatus {
        &self.dma_req_status
    }
    #[doc = "0xe18 - DMA single request status"]
    #[inline(always)]
    pub const fn dma_sreq_status(&self) -> &DmaSreqStatus {
        &self.dma_sreq_status
    }
    #[doc = "0xe20 - DMA done set"]
    #[inline(always)]
    pub const fn dma_done_set(&self) -> &DmaDoneSet {
        &self.dma_done_set
    }
    #[doc = "0xe24 - DMA done clear"]
    #[inline(always)]
    pub const fn dma_done_clr(&self) -> &DmaDoneClr {
        &self.dma_done_clr
    }
    #[doc = "0xe28 - DMA active set"]
    #[inline(always)]
    pub const fn dma_active_set(&self) -> &DmaActiveSet {
        &self.dma_active_set
    }
    #[doc = "0xe2c - DMA active clear"]
    #[inline(always)]
    pub const fn dma_active_clr(&self) -> &DmaActiveClr {
        &self.dma_active_clr
    }
    #[doc = "0xe48 - DMA bus error set"]
    #[inline(always)]
    pub const fn err_set(&self) -> &ErrSet {
        &self.err_set
    }
    #[doc = "0xfd0 - DMA Peripheral ID 4"]
    #[inline(always)]
    pub const fn periph_id_4(&self) -> &PeriphId4 {
        &self.periph_id_4
    }
    #[doc = "0xfe0 - DMA Peripheral ID 0"]
    #[inline(always)]
    pub const fn periph_id_0(&self) -> &PeriphId0 {
        &self.periph_id_0
    }
    #[doc = "0xfe4 - DMA Peripheral ID 1"]
    #[inline(always)]
    pub const fn periph_id_1(&self) -> &PeriphId1 {
        &self.periph_id_1
    }
    #[doc = "0xfe8 - DMA Peripheral ID 2"]
    #[inline(always)]
    pub const fn periph_id_2(&self) -> &PeriphId2 {
        &self.periph_id_2
    }
    #[doc = "0xfec - DMA Peripheral ID 3"]
    #[inline(always)]
    pub const fn periph_id_3(&self) -> &PeriphId3 {
        &self.periph_id_3
    }
    #[doc = "0xff0 - DMA PrimeCell ID 0"]
    #[inline(always)]
    pub const fn primecell_id_0(&self) -> &PrimecellId0 {
        &self.primecell_id_0
    }
    #[doc = "0xff4 - DMA PrimeCell ID 1"]
    #[inline(always)]
    pub const fn primecell_id_1(&self) -> &PrimecellId1 {
        &self.primecell_id_1
    }
    #[doc = "0xff8 - DMA PrimeCell ID 2"]
    #[inline(always)]
    pub const fn primecell_id_2(&self) -> &PrimecellId2 {
        &self.primecell_id_2
    }
    #[doc = "0xffc - DMA PrimeCell ID 3"]
    #[inline(always)]
    pub const fn primecell_id_3(&self) -> &PrimecellId3 {
        &self.primecell_id_3
    }
}
#[doc = "STATUS (r) register accessor: DMA Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "DMA Status"]
pub mod status;
#[doc = "CFG (w) register accessor: DMA Configuration\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "DMA Configuration"]
pub mod cfg;
#[doc = "CTRL_BASE_PTR (rw) register accessor: Base Pointer for DMA Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_base_ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_base_ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_base_ptr`] module"]
#[doc(alias = "CTRL_BASE_PTR")]
pub type CtrlBasePtr = crate::Reg<ctrl_base_ptr::CtrlBasePtrSpec>;
#[doc = "Base Pointer for DMA Control Registers"]
pub mod ctrl_base_ptr;
#[doc = "ALT_CTRL_BASE_PTR (rw) register accessor: DMA Channel alternate control data base pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`alt_ctrl_base_ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alt_ctrl_base_ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alt_ctrl_base_ptr`] module"]
#[doc(alias = "ALT_CTRL_BASE_PTR")]
pub type AltCtrlBasePtr = crate::Reg<alt_ctrl_base_ptr::AltCtrlBasePtrSpec>;
#[doc = "DMA Channel alternate control data base pointer"]
pub mod alt_ctrl_base_ptr;
#[doc = "WAITONREQ_STATUS (r) register accessor: DMA channel wait on request status\n\nYou can [`read`](crate::Reg::read) this register and get [`waitonreq_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@waitonreq_status`] module"]
#[doc(alias = "WAITONREQ_STATUS")]
pub type WaitonreqStatus = crate::Reg<waitonreq_status::WaitonreqStatusSpec>;
#[doc = "DMA channel wait on request status"]
pub mod waitonreq_status;
#[doc = "CHNL_SW_REQUEST (w) register accessor: DMA channel software request\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_sw_request::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl_sw_request`] module"]
#[doc(alias = "CHNL_SW_REQUEST")]
pub type ChnlSwRequest = crate::Reg<chnl_sw_request::ChnlSwRequestSpec>;
#[doc = "DMA channel software request"]
pub mod chnl_sw_request;
#[doc = "CHNL_USEBURST_SET (rw) register accessor: DMA channel useburst set\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl_useburst_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_useburst_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl_useburst_set`] module"]
#[doc(alias = "CHNL_USEBURST_SET")]
pub type ChnlUseburstSet = crate::Reg<chnl_useburst_set::ChnlUseburstSetSpec>;
#[doc = "DMA channel useburst set"]
pub mod chnl_useburst_set;
#[doc = "CHNL_USEBURST_CLR (rw) register accessor: DMA channel useburst clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl_useburst_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_useburst_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl_useburst_clr`] module"]
#[doc(alias = "CHNL_USEBURST_CLR")]
pub type ChnlUseburstClr = crate::Reg<chnl_useburst_clr::ChnlUseburstClrSpec>;
#[doc = "DMA channel useburst clear"]
pub mod chnl_useburst_clr;
#[doc = "CHNL_REQ_MASK_SET (rw) register accessor: DMA channel request mask set\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl_req_mask_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_req_mask_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl_req_mask_set`] module"]
#[doc(alias = "CHNL_REQ_MASK_SET")]
pub type ChnlReqMaskSet = crate::Reg<chnl_req_mask_set::ChnlReqMaskSetSpec>;
#[doc = "DMA channel request mask set"]
pub mod chnl_req_mask_set;
#[doc = "CHNL_REQ_MASK_CLR (rw) register accessor: DMA channel request mask clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl_req_mask_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_req_mask_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl_req_mask_clr`] module"]
#[doc(alias = "CHNL_REQ_MASK_CLR")]
pub type ChnlReqMaskClr = crate::Reg<chnl_req_mask_clr::ChnlReqMaskClrSpec>;
#[doc = "DMA channel request mask clear"]
pub mod chnl_req_mask_clr;
#[doc = "CHNL_ENABLE_SET (rw) register accessor: DMA channel enable set\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl_enable_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_enable_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl_enable_set`] module"]
#[doc(alias = "CHNL_ENABLE_SET")]
pub type ChnlEnableSet = crate::Reg<chnl_enable_set::ChnlEnableSetSpec>;
#[doc = "DMA channel enable set"]
pub mod chnl_enable_set;
#[doc = "CHNL_ENABLE_CLR (rw) register accessor: DMA channel enable clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl_enable_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_enable_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl_enable_clr`] module"]
#[doc(alias = "CHNL_ENABLE_CLR")]
pub type ChnlEnableClr = crate::Reg<chnl_enable_clr::ChnlEnableClrSpec>;
#[doc = "DMA channel enable clear"]
pub mod chnl_enable_clr;
#[doc = "CHNL_PRI_ALT_SET (rw) register accessor: DMA channel primary alternate set\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl_pri_alt_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_pri_alt_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl_pri_alt_set`] module"]
#[doc(alias = "CHNL_PRI_ALT_SET")]
pub type ChnlPriAltSet = crate::Reg<chnl_pri_alt_set::ChnlPriAltSetSpec>;
#[doc = "DMA channel primary alternate set"]
pub mod chnl_pri_alt_set;
#[doc = "CHNL_PRI_ALT_CLR (rw) register accessor: DMA channel primary alternate clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl_pri_alt_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_pri_alt_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl_pri_alt_clr`] module"]
#[doc(alias = "CHNL_PRI_ALT_CLR")]
pub type ChnlPriAltClr = crate::Reg<chnl_pri_alt_clr::ChnlPriAltClrSpec>;
#[doc = "DMA channel primary alternate clear"]
pub mod chnl_pri_alt_clr;
#[doc = "CHNL_PRIORITY_SET (rw) register accessor: DMA channel priority set\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl_priority_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_priority_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl_priority_set`] module"]
#[doc(alias = "CHNL_PRIORITY_SET")]
pub type ChnlPrioritySet = crate::Reg<chnl_priority_set::ChnlPrioritySetSpec>;
#[doc = "DMA channel priority set"]
pub mod chnl_priority_set;
#[doc = "CHNL_PRIORITY_CLR (w) register accessor: DMA channel priority clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_priority_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnl_priority_clr`] module"]
#[doc(alias = "CHNL_PRIORITY_CLR")]
pub type ChnlPriorityClr = crate::Reg<chnl_priority_clr::ChnlPriorityClrSpec>;
#[doc = "DMA channel priority clear"]
pub mod chnl_priority_clr;
#[doc = "ERR_CLR (rw) register accessor: DMA bus error clear\n\nYou can [`read`](crate::Reg::read) this register and get [`err_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_clr`] module"]
#[doc(alias = "ERR_CLR")]
pub type ErrClr = crate::Reg<err_clr::ErrClrSpec>;
#[doc = "DMA bus error clear"]
pub mod err_clr;
#[doc = "INTEGRATION_CFG (rw) register accessor: DMA integration configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`integration_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`integration_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@integration_cfg`] module"]
#[doc(alias = "INTEGRATION_CFG")]
pub type IntegrationCfg = crate::Reg<integration_cfg::IntegrationCfgSpec>;
#[doc = "DMA integration configuration"]
pub mod integration_cfg;
#[doc = "STALL_STATUS (rw) register accessor: DMA stall status\n\nYou can [`read`](crate::Reg::read) this register and get [`stall_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stall_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stall_status`] module"]
#[doc(alias = "STALL_STATUS")]
pub type StallStatus = crate::Reg<stall_status::StallStatusSpec>;
#[doc = "DMA stall status"]
pub mod stall_status;
#[doc = "DMA_REQ_STATUS (rw) register accessor: DMA Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_req_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_req_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_req_status`] module"]
#[doc(alias = "DMA_REQ_STATUS")]
pub type DmaReqStatus = crate::Reg<dma_req_status::DmaReqStatusSpec>;
#[doc = "DMA Configuration"]
pub mod dma_req_status;
#[doc = "DMA_SREQ_STATUS (rw) register accessor: DMA single request status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sreq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sreq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_sreq_status`] module"]
#[doc(alias = "DMA_SREQ_STATUS")]
pub type DmaSreqStatus = crate::Reg<dma_sreq_status::DmaSreqStatusSpec>;
#[doc = "DMA single request status"]
pub mod dma_sreq_status;
#[doc = "DMA_DONE_SET (rw) register accessor: DMA done set\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_done_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_done_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_done_set`] module"]
#[doc(alias = "DMA_DONE_SET")]
pub type DmaDoneSet = crate::Reg<dma_done_set::DmaDoneSetSpec>;
#[doc = "DMA done set"]
pub mod dma_done_set;
#[doc = "DMA_DONE_CLR (rw) register accessor: DMA done clear\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_done_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_done_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_done_clr`] module"]
#[doc(alias = "DMA_DONE_CLR")]
pub type DmaDoneClr = crate::Reg<dma_done_clr::DmaDoneClrSpec>;
#[doc = "DMA done clear"]
pub mod dma_done_clr;
#[doc = "DMA_ACTIVE_SET (rw) register accessor: DMA active set\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_active_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_active_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_active_set`] module"]
#[doc(alias = "DMA_ACTIVE_SET")]
pub type DmaActiveSet = crate::Reg<dma_active_set::DmaActiveSetSpec>;
#[doc = "DMA active set"]
pub mod dma_active_set;
#[doc = "DMA_ACTIVE_CLR (rw) register accessor: DMA active clear\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_active_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_active_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_active_clr`] module"]
#[doc(alias = "DMA_ACTIVE_CLR")]
pub type DmaActiveClr = crate::Reg<dma_active_clr::DmaActiveClrSpec>;
#[doc = "DMA active clear"]
pub mod dma_active_clr;
#[doc = "ERR_SET (rw) register accessor: DMA bus error set\n\nYou can [`read`](crate::Reg::read) this register and get [`err_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_set`] module"]
#[doc(alias = "ERR_SET")]
pub type ErrSet = crate::Reg<err_set::ErrSetSpec>;
#[doc = "DMA bus error set"]
pub mod err_set;
#[doc = "PERIPH_ID_4 (rw) register accessor: DMA Peripheral ID 4\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id_4`] module"]
#[doc(alias = "PERIPH_ID_4")]
pub type PeriphId4 = crate::Reg<periph_id_4::PeriphId4Spec>;
#[doc = "DMA Peripheral ID 4"]
pub mod periph_id_4;
#[doc = "PERIPH_ID_0 (rw) register accessor: DMA Peripheral ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id_0`] module"]
#[doc(alias = "PERIPH_ID_0")]
pub type PeriphId0 = crate::Reg<periph_id_0::PeriphId0Spec>;
#[doc = "DMA Peripheral ID 0"]
pub mod periph_id_0;
#[doc = "PERIPH_ID_1 (r) register accessor: DMA Peripheral ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id_1`] module"]
#[doc(alias = "PERIPH_ID_1")]
pub type PeriphId1 = crate::Reg<periph_id_1::PeriphId1Spec>;
#[doc = "DMA Peripheral ID 1"]
pub mod periph_id_1;
#[doc = "PERIPH_ID_2 (rw) register accessor: DMA Peripheral ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id_2`] module"]
#[doc(alias = "PERIPH_ID_2")]
pub type PeriphId2 = crate::Reg<periph_id_2::PeriphId2Spec>;
#[doc = "DMA Peripheral ID 2"]
pub mod periph_id_2;
#[doc = "PERIPH_ID_3 (rw) register accessor: DMA Peripheral ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id_3`] module"]
#[doc(alias = "PERIPH_ID_3")]
pub type PeriphId3 = crate::Reg<periph_id_3::PeriphId3Spec>;
#[doc = "DMA Peripheral ID 3"]
pub mod periph_id_3;
#[doc = "PRIMECELL_ID_0 (rw) register accessor: DMA PrimeCell ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`primecell_id_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`primecell_id_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@primecell_id_0`] module"]
#[doc(alias = "PRIMECELL_ID_0")]
pub type PrimecellId0 = crate::Reg<primecell_id_0::PrimecellId0Spec>;
#[doc = "DMA PrimeCell ID 0"]
pub mod primecell_id_0;
#[doc = "PRIMECELL_ID_1 (rw) register accessor: DMA PrimeCell ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`primecell_id_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`primecell_id_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@primecell_id_1`] module"]
#[doc(alias = "PRIMECELL_ID_1")]
pub type PrimecellId1 = crate::Reg<primecell_id_1::PrimecellId1Spec>;
#[doc = "DMA PrimeCell ID 1"]
pub mod primecell_id_1;
#[doc = "PRIMECELL_ID_2 (rw) register accessor: DMA PrimeCell ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`primecell_id_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`primecell_id_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@primecell_id_2`] module"]
#[doc(alias = "PRIMECELL_ID_2")]
pub type PrimecellId2 = crate::Reg<primecell_id_2::PrimecellId2Spec>;
#[doc = "DMA PrimeCell ID 2"]
pub mod primecell_id_2;
#[doc = "PRIMECELL_ID_3 (rw) register accessor: DMA PrimeCell ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`primecell_id_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`primecell_id_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@primecell_id_3`] module"]
#[doc(alias = "PRIMECELL_ID_3")]
pub type PrimecellId3 = crate::Reg<primecell_id_3::PrimecellId3Spec>;
#[doc = "DMA PrimeCell ID 3"]
pub mod primecell_id_3;
