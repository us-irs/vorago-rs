#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mac_config: MacConfig,
    mac_frame_fltr: MacFrameFltr,
    _reserved2: [u8; 0x08],
    mac_gmii_addr: MacGmiiAddr,
    mac_gmii_data: MacGmiiData,
    mac_flow_ctrl: MacFlowCtrl,
    mac_vlan_tag: MacVlanTag,
    _reserved6: [u8; 0x04],
    mac_debug: MacDebug,
    _reserved7: [u8; 0x10],
    mac_intr_stat: MacIntrStat,
    mac_intr_mask: MacIntrMask,
    mac_addr_h: MacAddrH,
    mac_addr_l: MacAddrL,
    _reserved11: [u8; 0x94],
    mac_wdog_to: MacWdogTo,
    _reserved12: [u8; 0x20],
    mmc_cntrl: MmcCntrl,
    mmc_intr_rx: MmcIntrRx,
    mmc_intr_tx: MmcIntrTx,
    mmc_intr_mask_rx: MmcIntrMaskRx,
    mmc_intr_mask_tx: MmcIntrMaskTx,
    txoctetcount_gb: TxoctetcountGb,
    txframecount_gb: TxframecountGb,
    txbcastframes_g: TxbcastframesG,
    txmcastframes_g: TxmcastframesG,
    tx64oct_gb: Tx64octGb,
    tx65to127oct_gb: Tx65to127octGb,
    tx128to255oct_gb: Tx128to255octGb,
    tx256to511oct_gb: Tx256to511octGb,
    tx512to1023oct_gb: Tx512to1023octGb,
    tx1024maxoct_gb: Tx1024maxoctGb,
    txucastframe_gb: TxucastframeGb,
    txmcastframe_gb: TxmcastframeGb,
    txbcastframe_gb: TxbcastframeGb,
    txundererr: Txundererr,
    txsinglecol_g: TxsinglecolG,
    txmulticol_g: TxmulticolG,
    txdeferred: Txdeferred,
    txlatecol: Txlatecol,
    txexesscol: Txexesscol,
    txcarriererror: Txcarriererror,
    txoctetcount_g: TxoctetcountG,
    txframecount_g: TxframecountG,
    txexcessdef: Txexcessdef,
    txpauseframes: Txpauseframes,
    txlanframes_g: TxlanframesG,
    txoversize_g: TxoversizeG,
    _reserved43: [u8; 0x04],
    rxframecount_gb: RxframecountGb,
    rxoctetcount_gb: RxoctetcountGb,
    rxoctetcount_g: RxoctetcountG,
    rxbcastframes_g: RxbcastframesG,
    rxmcastframes_g: RxmcastframesG,
    rxcrcerror: Rxcrcerror,
    rxalignerror: Rxalignerror,
    rxrunterror: Rxrunterror,
    rxjabbererror: Rxjabbererror,
    rxundersize_g: RxundersizeG,
    rxoversize_g: RxoversizeG,
    rx64octets_gb: Rx64octetsGb,
    rx65to127oct_gb: Rx65to127octGb,
    rx128to255oct_gb: Rx128to255octGb,
    rx256to511oct_gb: Rx256to511octGb,
    rx512to1023oct_gb: Rx512to1023octGb,
    rx1024maxoct_gb: Rx1024maxoctGb,
    rxucastframes_g: RxucastframesG,
    rxlengtherror: Rxlengtherror,
    rxoutrangetype: Rxoutrangetype,
    rxpauseframes: Rxpauseframes,
    rxfifooverflow: Rxfifooverflow,
    rxvlanframes_gb: RxvlanframesGb,
    rxwdogerror: Rxwdogerror,
    rxrcverror: Rxrcverror,
    rxctrlframes_g: RxctrlframesG,
    _reserved69: [u8; 0x039c],
    vlan_increplace: VlanIncreplace,
    vlan_hashtable: VlanHashtable,
    _reserved71: [u8; 0x0174],
    timestamp_ctrl: TimestampCtrl,
    subsec_inc: SubsecInc,
    systime_seconds: SystimeSeconds,
    systime_nanosec: SystimeNanosec,
    systime_secsupdat: SystimeSecsupdat,
    systime_nsecup: SystimeNsecup,
    timestampaddend: Timestampaddend,
    target_time_secs: TargetTimeSecs,
    target_time_nsec: TargetTimeNsec,
    _reserved80: [u8; 0x08dc],
    dma_bus_mode: DmaBusMode,
    dma_tx_poll_demand: DmaTxPollDemand,
    dma_rx_poll_demand: DmaRxPollDemand,
    dma_rx_desc_list_addr: DmaRxDescListAddr,
    dma_tx_desc_list_addr: DmaTxDescListAddr,
    dma_status: DmaStatus,
    dma_oper_mode: DmaOperMode,
    dma_intr_en: DmaIntrEn,
    dma_miss_over_counter: DmaMissOverCounter,
    dma_rx_intr_wdog_timer: DmaRxIntrWdogTimer,
    _reserved90: [u8; 0x04],
    dma_ahb_status: DmaAhbStatus,
    _reserved91: [u8; 0x18],
    dma_curr_tx_desc: DmaCurrTxDesc,
    dma_curr_rx_desc: DmaCurrRxDesc,
    dma_curr_tx_bufr_addr: DmaCurrTxBufrAddr,
    dma_curr_rx_bufr_addr: DmaCurrRxBufrAddr,
}
impl RegisterBlock {
    #[doc = "0x00 - Operation mode register for the MAC"]
    #[inline(always)]
    pub const fn mac_config(&self) -> &MacConfig {
        &self.mac_config
    }
    #[doc = "0x04 - Contains the frame filtering controls"]
    #[inline(always)]
    pub const fn mac_frame_fltr(&self) -> &MacFrameFltr {
        &self.mac_frame_fltr
    }
    #[doc = "0x10 - Controls the management cycles to an external PHY"]
    #[inline(always)]
    pub const fn mac_gmii_addr(&self) -> &MacGmiiAddr {
        &self.mac_gmii_addr
    }
    #[doc = "0x14 - Contains the data to be written to or read from the PHY register"]
    #[inline(always)]
    pub const fn mac_gmii_data(&self) -> &MacGmiiData {
        &self.mac_gmii_data
    }
    #[doc = "0x18 - Controls the generation of control frames"]
    #[inline(always)]
    pub const fn mac_flow_ctrl(&self) -> &MacFlowCtrl {
        &self.mac_flow_ctrl
    }
    #[doc = "0x1c - Identifies IEEE 802.1Q VLAN type frames"]
    #[inline(always)]
    pub const fn mac_vlan_tag(&self) -> &MacVlanTag {
        &self.mac_vlan_tag
    }
    #[doc = "0x24 - Gives the status of the various internal blocks for debugging"]
    #[inline(always)]
    pub const fn mac_debug(&self) -> &MacDebug {
        &self.mac_debug
    }
    #[doc = "0x38 - Contains the interrupt status"]
    #[inline(always)]
    pub const fn mac_intr_stat(&self) -> &MacIntrStat {
        &self.mac_intr_stat
    }
    #[doc = "0x3c - Contains the masks for generating interrupt"]
    #[inline(always)]
    pub const fn mac_intr_mask(&self) -> &MacIntrMask {
        &self.mac_intr_mask
    }
    #[doc = "0x40 - Contains the high 16-bits of the first MAC Address"]
    #[inline(always)]
    pub const fn mac_addr_h(&self) -> &MacAddrH {
        &self.mac_addr_h
    }
    #[doc = "0x44 - Contains the Low 32-bits of the first MAC Address"]
    #[inline(always)]
    pub const fn mac_addr_l(&self) -> &MacAddrL {
        &self.mac_addr_l
    }
    #[doc = "0xdc - Controls the watchdog time-out for received frames"]
    #[inline(always)]
    pub const fn mac_wdog_to(&self) -> &MacWdogTo {
        &self.mac_wdog_to
    }
    #[doc = "0x100 - MMC Control Register"]
    #[inline(always)]
    pub const fn mmc_cntrl(&self) -> &MmcCntrl {
        &self.mmc_cntrl
    }
    #[doc = "0x104 - MMC Receive Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_intr_rx(&self) -> &MmcIntrRx {
        &self.mmc_intr_rx
    }
    #[doc = "0x108 - MMC Transmit Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_intr_tx(&self) -> &MmcIntrTx {
        &self.mmc_intr_tx
    }
    #[doc = "0x10c - MMC Receive Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_intr_mask_rx(&self) -> &MmcIntrMaskRx {
        &self.mmc_intr_mask_rx
    }
    #[doc = "0x110 - MMC Transmit Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_intr_mask_tx(&self) -> &MmcIntrMaskTx {
        &self.mmc_intr_mask_tx
    }
    #[doc = "0x114 - MMC Transmit Count"]
    #[inline(always)]
    pub const fn txoctetcount_gb(&self) -> &TxoctetcountGb {
        &self.txoctetcount_gb
    }
    #[doc = "0x118 - MMC Frame Count Register"]
    #[inline(always)]
    pub const fn txframecount_gb(&self) -> &TxframecountGb {
        &self.txframecount_gb
    }
    #[doc = "0x11c - MMC Good Broadcast Frames Register"]
    #[inline(always)]
    pub const fn txbcastframes_g(&self) -> &TxbcastframesG {
        &self.txbcastframes_g
    }
    #[doc = "0x120 - MMC Good Multicast Frames Register"]
    #[inline(always)]
    pub const fn txmcastframes_g(&self) -> &TxmcastframesG {
        &self.txmcastframes_g
    }
    #[doc = "0x124 - MMC Good and bad Frames transmitted with length 64"]
    #[inline(always)]
    pub const fn tx64oct_gb(&self) -> &Tx64octGb {
        &self.tx64oct_gb
    }
    #[doc = "0x128 - MMC Good and bad Frames transmitted with length 65 to 127"]
    #[inline(always)]
    pub const fn tx65to127oct_gb(&self) -> &Tx65to127octGb {
        &self.tx65to127oct_gb
    }
    #[doc = "0x12c - MMC Good and bad Frames transmitted with length 128 to 255"]
    #[inline(always)]
    pub const fn tx128to255oct_gb(&self) -> &Tx128to255octGb {
        &self.tx128to255oct_gb
    }
    #[doc = "0x130 - MMC Good and bad Frames transmitted with length 256 to 511"]
    #[inline(always)]
    pub const fn tx256to511oct_gb(&self) -> &Tx256to511octGb {
        &self.tx256to511oct_gb
    }
    #[doc = "0x134 - MMC Good and bad Frames transmitted with length 512 to 1023"]
    #[inline(always)]
    pub const fn tx512to1023oct_gb(&self) -> &Tx512to1023octGb {
        &self.tx512to1023oct_gb
    }
    #[doc = "0x138 - MMC Good and bad Frames transmitted with length 1024 to max bytes"]
    #[inline(always)]
    pub const fn tx1024maxoct_gb(&self) -> &Tx1024maxoctGb {
        &self.tx1024maxoct_gb
    }
    #[doc = "0x13c - MMC number of good and bad unicast frames transmitted"]
    #[inline(always)]
    pub const fn txucastframe_gb(&self) -> &TxucastframeGb {
        &self.txucastframe_gb
    }
    #[doc = "0x140 - MMC number of good and bad MULTIcast frames transmitted"]
    #[inline(always)]
    pub const fn txmcastframe_gb(&self) -> &TxmcastframeGb {
        &self.txmcastframe_gb
    }
    #[doc = "0x144 - MMC number of good and bad broadcast frames transmitted"]
    #[inline(always)]
    pub const fn txbcastframe_gb(&self) -> &TxbcastframeGb {
        &self.txbcastframe_gb
    }
    #[doc = "0x148 - MMC number of frames aborted because of frame underflow error"]
    #[inline(always)]
    pub const fn txundererr(&self) -> &Txundererr {
        &self.txundererr
    }
    #[doc = "0x14c - MMC Number of successfully transmitted frames after a single collision"]
    #[inline(always)]
    pub const fn txsinglecol_g(&self) -> &TxsinglecolG {
        &self.txsinglecol_g
    }
    #[doc = "0x150 - MMC Number of successfully transmitted frames after multiple collisions"]
    #[inline(always)]
    pub const fn txmulticol_g(&self) -> &TxmulticolG {
        &self.txmulticol_g
    }
    #[doc = "0x154 - MMC Number of successfully transmitted frames after a deferral"]
    #[inline(always)]
    pub const fn txdeferred(&self) -> &Txdeferred {
        &self.txdeferred
    }
    #[doc = "0x158 - MMC Number of aborted frames because of late collision error"]
    #[inline(always)]
    pub const fn txlatecol(&self) -> &Txlatecol {
        &self.txlatecol
    }
    #[doc = "0x15c - MMC Number of aborted frames because of excessive collision errors"]
    #[inline(always)]
    pub const fn txexesscol(&self) -> &Txexesscol {
        &self.txexesscol
    }
    #[doc = "0x160 - MMC Number of aborted frames because of carrier sense error"]
    #[inline(always)]
    pub const fn txcarriererror(&self) -> &Txcarriererror {
        &self.txcarriererror
    }
    #[doc = "0x164 - MMC Number of bytes transmitted frames only in good frames"]
    #[inline(always)]
    pub const fn txoctetcount_g(&self) -> &TxoctetcountG {
        &self.txoctetcount_g
    }
    #[doc = "0x168 - MMC Number of good frames transmitted"]
    #[inline(always)]
    pub const fn txframecount_g(&self) -> &TxframecountG {
        &self.txframecount_g
    }
    #[doc = "0x16c - MMC Number of frames aborted because of excessive deferral error"]
    #[inline(always)]
    pub const fn txexcessdef(&self) -> &Txexcessdef {
        &self.txexcessdef
    }
    #[doc = "0x170 - MMC Number of good pause frames transmitted"]
    #[inline(always)]
    pub const fn txpauseframes(&self) -> &Txpauseframes {
        &self.txpauseframes
    }
    #[doc = "0x174 - MMC Number of good VLAN frames transmitted"]
    #[inline(always)]
    pub const fn txlanframes_g(&self) -> &TxlanframesG {
        &self.txlanframes_g
    }
    #[doc = "0x178 - MMC Number of frames transmitted without errors"]
    #[inline(always)]
    pub const fn txoversize_g(&self) -> &TxoversizeG {
        &self.txoversize_g
    }
    #[doc = "0x180 - MMC Number of good and bad frames received"]
    #[inline(always)]
    pub const fn rxframecount_gb(&self) -> &RxframecountGb {
        &self.rxframecount_gb
    }
    #[doc = "0x184 - MMC Number of bytes received in good and bad frames"]
    #[inline(always)]
    pub const fn rxoctetcount_gb(&self) -> &RxoctetcountGb {
        &self.rxoctetcount_gb
    }
    #[doc = "0x188 - MMC Number of bytes received in good frames only"]
    #[inline(always)]
    pub const fn rxoctetcount_g(&self) -> &RxoctetcountG {
        &self.rxoctetcount_g
    }
    #[doc = "0x18c - MMC Number of good broadcast frames received"]
    #[inline(always)]
    pub const fn rxbcastframes_g(&self) -> &RxbcastframesG {
        &self.rxbcastframes_g
    }
    #[doc = "0x190 - MMC Number of good multicast frames received"]
    #[inline(always)]
    pub const fn rxmcastframes_g(&self) -> &RxmcastframesG {
        &self.rxmcastframes_g
    }
    #[doc = "0x194 - MMC Number of frames received with CRC error"]
    #[inline(always)]
    pub const fn rxcrcerror(&self) -> &Rxcrcerror {
        &self.rxcrcerror
    }
    #[doc = "0x198 - MMC Number of frames received with alignment error"]
    #[inline(always)]
    pub const fn rxalignerror(&self) -> &Rxalignerror {
        &self.rxalignerror
    }
    #[doc = "0x19c - MMC Number of frames received with runt error"]
    #[inline(always)]
    pub const fn rxrunterror(&self) -> &Rxrunterror {
        &self.rxrunterror
    }
    #[doc = "0x1a0 - MMC Number of giant frames received with length greater than 1518 bytes and with CRC error"]
    #[inline(always)]
    pub const fn rxjabbererror(&self) -> &Rxjabbererror {
        &self.rxjabbererror
    }
    #[doc = "0x1a4 - MMC Number of frames received with length less than 64 bytes"]
    #[inline(always)]
    pub const fn rxundersize_g(&self) -> &RxundersizeG {
        &self.rxundersize_g
    }
    #[doc = "0x1a8 - MMC Number of frames received without errors with length greater than the max size"]
    #[inline(always)]
    pub const fn rxoversize_g(&self) -> &RxoversizeG {
        &self.rxoversize_g
    }
    #[doc = "0x1ac - MMC Number of good and bad frames received with length 64 bytes"]
    #[inline(always)]
    pub const fn rx64octets_gb(&self) -> &Rx64octetsGb {
        &self.rx64octets_gb
    }
    #[doc = "0x1b0 - MMC Number of good and bad frames received with length between 65 and 127 bytes"]
    #[inline(always)]
    pub const fn rx65to127oct_gb(&self) -> &Rx65to127octGb {
        &self.rx65to127oct_gb
    }
    #[doc = "0x1b4 - MMC Number of good and bad frames received with length between 128 and 255 bytes"]
    #[inline(always)]
    pub const fn rx128to255oct_gb(&self) -> &Rx128to255octGb {
        &self.rx128to255oct_gb
    }
    #[doc = "0x1b8 - MMC Number of good and bad frames received with length between 256 and 511 bytes"]
    #[inline(always)]
    pub const fn rx256to511oct_gb(&self) -> &Rx256to511octGb {
        &self.rx256to511oct_gb
    }
    #[doc = "0x1bc - MMC Number of good and bad frames received with length between 512 and 1023 bytes"]
    #[inline(always)]
    pub const fn rx512to1023oct_gb(&self) -> &Rx512to1023octGb {
        &self.rx512to1023oct_gb
    }
    #[doc = "0x1c0 - MMC Number of good and bad frames received with length between 1024 and max size bytes"]
    #[inline(always)]
    pub const fn rx1024maxoct_gb(&self) -> &Rx1024maxoctGb {
        &self.rx1024maxoct_gb
    }
    #[doc = "0x1c4 - MMC Number of received good unicast frames"]
    #[inline(always)]
    pub const fn rxucastframes_g(&self) -> &RxucastframesG {
        &self.rxucastframes_g
    }
    #[doc = "0x1c8 - MMC Number of frames received with length error"]
    #[inline(always)]
    pub const fn rxlengtherror(&self) -> &Rxlengtherror {
        &self.rxlengtherror
    }
    #[doc = "0x1cc - MMC Number of frames received with length field not equal to the valid frame size"]
    #[inline(always)]
    pub const fn rxoutrangetype(&self) -> &Rxoutrangetype {
        &self.rxoutrangetype
    }
    #[doc = "0x1d0 - MMC Number of good and valid Pause frames received"]
    #[inline(always)]
    pub const fn rxpauseframes(&self) -> &Rxpauseframes {
        &self.rxpauseframes
    }
    #[doc = "0x1d4 - MMC Number of missed received frames because of FIFO overflow"]
    #[inline(always)]
    pub const fn rxfifooverflow(&self) -> &Rxfifooverflow {
        &self.rxfifooverflow
    }
    #[doc = "0x1d8 - MMC Number of good and bad VLAN frames received"]
    #[inline(always)]
    pub const fn rxvlanframes_gb(&self) -> &RxvlanframesGb {
        &self.rxvlanframes_gb
    }
    #[doc = "0x1dc - MMC Number of frames received with error because of watchdog timeout error"]
    #[inline(always)]
    pub const fn rxwdogerror(&self) -> &Rxwdogerror {
        &self.rxwdogerror
    }
    #[doc = "0x1e0 - MMC Number of frames received with Receive error or Frame Extension error"]
    #[inline(always)]
    pub const fn rxrcverror(&self) -> &Rxrcverror {
        &self.rxrcverror
    }
    #[doc = "0x1e4 - MMC Number of received good control frames"]
    #[inline(always)]
    pub const fn rxctrlframes_g(&self) -> &RxctrlframesG {
        &self.rxctrlframes_g
    }
    #[doc = "0x584 - Holds the VLAN Tag for insertion into or replacement in the transmit frames"]
    #[inline(always)]
    pub const fn vlan_increplace(&self) -> &VlanIncreplace {
        &self.vlan_increplace
    }
    #[doc = "0x588 - Holds the VLAN Hash Table"]
    #[inline(always)]
    pub const fn vlan_hashtable(&self) -> &VlanHashtable {
        &self.vlan_hashtable
    }
    #[doc = "0x700 - Controls the IEEE 1588 timestamp generation and update logic"]
    #[inline(always)]
    pub const fn timestamp_ctrl(&self) -> &TimestampCtrl {
        &self.timestamp_ctrl
    }
    #[doc = "0x704 - Holds the 8-bit value by which the Sub-Second register is incremented"]
    #[inline(always)]
    pub const fn subsec_inc(&self) -> &SubsecInc {
        &self.subsec_inc
    }
    #[doc = "0x708 - Holds the lower 32 bits of the second field of the system time"]
    #[inline(always)]
    pub const fn systime_seconds(&self) -> &SystimeSeconds {
        &self.systime_seconds
    }
    #[doc = "0x70c - Holds 32 bits of the nano-second field of the system time"]
    #[inline(always)]
    pub const fn systime_nanosec(&self) -> &SystimeNanosec {
        &self.systime_nanosec
    }
    #[doc = "0x710 - Holds the lower 32 bits of the second field to be written to, added to, or subtracted from the system time value"]
    #[inline(always)]
    pub const fn systime_secsupdat(&self) -> &SystimeSecsupdat {
        &self.systime_secsupdat
    }
    #[doc = "0x714 - Holds 32 bits of the nano-second field to be written to, added to, or subtracted from the system time value"]
    #[inline(always)]
    pub const fn systime_nsecup(&self) -> &SystimeNsecup {
        &self.systime_nsecup
    }
    #[doc = "0x718 - This register is used by software to re-adjust the clock frequency linearly to match the Master clock frequency"]
    #[inline(always)]
    pub const fn timestampaddend(&self) -> &Timestampaddend {
        &self.timestampaddend
    }
    #[doc = "0x71c - Holds the high 32-bits of time to be compared with the system time"]
    #[inline(always)]
    pub const fn target_time_secs(&self) -> &TargetTimeSecs {
        &self.target_time_secs
    }
    #[doc = "0x720 - Holds the lower 32-bits of time to be compared with the system time"]
    #[inline(always)]
    pub const fn target_time_nsec(&self) -> &TargetTimeNsec {
        &self.target_time_nsec
    }
    #[doc = "0x1000 - Controls the DMA Host Interface Mode"]
    #[inline(always)]
    pub const fn dma_bus_mode(&self) -> &DmaBusMode {
        &self.dma_bus_mode
    }
    #[doc = "0x1004 - Used by the host to instruct the DMA to poll the transmit Descriptor list"]
    #[inline(always)]
    pub const fn dma_tx_poll_demand(&self) -> &DmaTxPollDemand {
        &self.dma_tx_poll_demand
    }
    #[doc = "0x1008 - Used by the host to instruct the DMA to poll the Receive Descriptor list"]
    #[inline(always)]
    pub const fn dma_rx_poll_demand(&self) -> &DmaRxPollDemand {
        &self.dma_rx_poll_demand
    }
    #[doc = "0x100c - Points the DMA to the start of the Receive Descriptor list"]
    #[inline(always)]
    pub const fn dma_rx_desc_list_addr(&self) -> &DmaRxDescListAddr {
        &self.dma_rx_desc_list_addr
    }
    #[doc = "0x1010 - Points the DMA to the start of the Transmit Descriptor list"]
    #[inline(always)]
    pub const fn dma_tx_desc_list_addr(&self) -> &DmaTxDescListAddr {
        &self.dma_tx_desc_list_addr
    }
    #[doc = "0x1014 - Used to determine the status of the DMA"]
    #[inline(always)]
    pub const fn dma_status(&self) -> &DmaStatus {
        &self.dma_status
    }
    #[doc = "0x1018 - Sets the Receive and Transmit operation mode and command"]
    #[inline(always)]
    pub const fn dma_oper_mode(&self) -> &DmaOperMode {
        &self.dma_oper_mode
    }
    #[doc = "0x101c - Enables the interrupts reported in the status register"]
    #[inline(always)]
    pub const fn dma_intr_en(&self) -> &DmaIntrEn {
        &self.dma_intr_en
    }
    #[doc = "0x1020 - Contains the counters for discarded frames because no Receive Descriptor is available"]
    #[inline(always)]
    pub const fn dma_miss_over_counter(&self) -> &DmaMissOverCounter {
        &self.dma_miss_over_counter
    }
    #[doc = "0x1024 - Watchdog timeout for Receive Interrupt from DMA"]
    #[inline(always)]
    pub const fn dma_rx_intr_wdog_timer(&self) -> &DmaRxIntrWdogTimer {
        &self.dma_rx_intr_wdog_timer
    }
    #[doc = "0x102c - Provides the active status of the read and write channels of the AHB master interface"]
    #[inline(always)]
    pub const fn dma_ahb_status(&self) -> &DmaAhbStatus {
        &self.dma_ahb_status
    }
    #[doc = "0x1048 - Contains the start address of the current Transmit Descriptor read by the DMA"]
    #[inline(always)]
    pub const fn dma_curr_tx_desc(&self) -> &DmaCurrTxDesc {
        &self.dma_curr_tx_desc
    }
    #[doc = "0x104c - Contains the start address of the current Receive Descriptor read by the DMA"]
    #[inline(always)]
    pub const fn dma_curr_rx_desc(&self) -> &DmaCurrRxDesc {
        &self.dma_curr_rx_desc
    }
    #[doc = "0x1050 - Contains the start address of the current Receive Descriptor read by the DMA"]
    #[inline(always)]
    pub const fn dma_curr_tx_bufr_addr(&self) -> &DmaCurrTxBufrAddr {
        &self.dma_curr_tx_bufr_addr
    }
    #[doc = "0x1054 - Contains the current Receive Buffer address read by the DMA"]
    #[inline(always)]
    pub const fn dma_curr_rx_bufr_addr(&self) -> &DmaCurrRxBufrAddr {
        &self.dma_curr_rx_bufr_addr
    }
}
#[doc = "MAC_CONFIG (rw) register accessor: Operation mode register for the MAC\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_config`] module"]
#[doc(alias = "MAC_CONFIG")]
pub type MacConfig = crate::Reg<mac_config::MacConfigSpec>;
#[doc = "Operation mode register for the MAC"]
pub mod mac_config;
#[doc = "MAC_FRAME_FLTR (rw) register accessor: Contains the frame filtering controls\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_frame_fltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_frame_fltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_frame_fltr`] module"]
#[doc(alias = "MAC_FRAME_FLTR")]
pub type MacFrameFltr = crate::Reg<mac_frame_fltr::MacFrameFltrSpec>;
#[doc = "Contains the frame filtering controls"]
pub mod mac_frame_fltr;
#[doc = "MAC_GMII_ADDR (rw) register accessor: Controls the management cycles to an external PHY\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_gmii_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_gmii_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_gmii_addr`] module"]
#[doc(alias = "MAC_GMII_ADDR")]
pub type MacGmiiAddr = crate::Reg<mac_gmii_addr::MacGmiiAddrSpec>;
#[doc = "Controls the management cycles to an external PHY"]
pub mod mac_gmii_addr;
#[doc = "MAC_GMII_DATA (rw) register accessor: Contains the data to be written to or read from the PHY register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_gmii_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_gmii_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_gmii_data`] module"]
#[doc(alias = "MAC_GMII_DATA")]
pub type MacGmiiData = crate::Reg<mac_gmii_data::MacGmiiDataSpec>;
#[doc = "Contains the data to be written to or read from the PHY register"]
pub mod mac_gmii_data;
#[doc = "MAC_FLOW_CTRL (rw) register accessor: Controls the generation of control frames\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_flow_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_flow_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_flow_ctrl`] module"]
#[doc(alias = "MAC_FLOW_CTRL")]
pub type MacFlowCtrl = crate::Reg<mac_flow_ctrl::MacFlowCtrlSpec>;
#[doc = "Controls the generation of control frames"]
pub mod mac_flow_ctrl;
#[doc = "MAC_VLAN_TAG (rw) register accessor: Identifies IEEE 802.1Q VLAN type frames\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_vlan_tag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_vlan_tag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_vlan_tag`] module"]
#[doc(alias = "MAC_VLAN_TAG")]
pub type MacVlanTag = crate::Reg<mac_vlan_tag::MacVlanTagSpec>;
#[doc = "Identifies IEEE 802.1Q VLAN type frames"]
pub mod mac_vlan_tag;
#[doc = "MAC_DEBUG (r) register accessor: Gives the status of the various internal blocks for debugging\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_debug::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_debug`] module"]
#[doc(alias = "MAC_DEBUG")]
pub type MacDebug = crate::Reg<mac_debug::MacDebugSpec>;
#[doc = "Gives the status of the various internal blocks for debugging"]
pub mod mac_debug;
#[doc = "MAC_INTR_STAT (r) register accessor: Contains the interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_intr_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_intr_stat`] module"]
#[doc(alias = "MAC_INTR_STAT")]
pub type MacIntrStat = crate::Reg<mac_intr_stat::MacIntrStatSpec>;
#[doc = "Contains the interrupt status"]
pub mod mac_intr_stat;
#[doc = "MAC_INTR_MASK (rw) register accessor: Contains the masks for generating interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_intr_mask`] module"]
#[doc(alias = "MAC_INTR_MASK")]
pub type MacIntrMask = crate::Reg<mac_intr_mask::MacIntrMaskSpec>;
#[doc = "Contains the masks for generating interrupt"]
pub mod mac_intr_mask;
#[doc = "MAC_ADDR_H (rw) register accessor: Contains the high 16-bits of the first MAC Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_addr_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_addr_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_addr_h`] module"]
#[doc(alias = "MAC_ADDR_H")]
pub type MacAddrH = crate::Reg<mac_addr_h::MacAddrHSpec>;
#[doc = "Contains the high 16-bits of the first MAC Address"]
pub mod mac_addr_h;
#[doc = "MAC_ADDR_L (rw) register accessor: Contains the Low 32-bits of the first MAC Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_addr_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_addr_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_addr_l`] module"]
#[doc(alias = "MAC_ADDR_L")]
pub type MacAddrL = crate::Reg<mac_addr_l::MacAddrLSpec>;
#[doc = "Contains the Low 32-bits of the first MAC Address"]
pub mod mac_addr_l;
#[doc = "MAC_WDOG_TO (rw) register accessor: Controls the watchdog time-out for received frames\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_wdog_to::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_wdog_to::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_wdog_to`] module"]
#[doc(alias = "MAC_WDOG_TO")]
pub type MacWdogTo = crate::Reg<mac_wdog_to::MacWdogToSpec>;
#[doc = "Controls the watchdog time-out for received frames"]
pub mod mac_wdog_to;
#[doc = "MMC_CNTRL (rw) register accessor: MMC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_cntrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_cntrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_cntrl`] module"]
#[doc(alias = "MMC_CNTRL")]
pub type MmcCntrl = crate::Reg<mmc_cntrl::MmcCntrlSpec>;
#[doc = "MMC Control Register"]
pub mod mmc_cntrl;
#[doc = "MMC_INTR_RX (rw) register accessor: MMC Receive Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_intr_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_intr_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_intr_rx`] module"]
#[doc(alias = "MMC_INTR_RX")]
pub type MmcIntrRx = crate::Reg<mmc_intr_rx::MmcIntrRxSpec>;
#[doc = "MMC Receive Interrupt Register"]
pub mod mmc_intr_rx;
#[doc = "MMC_INTR_TX (rw) register accessor: MMC Transmit Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_intr_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_intr_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_intr_tx`] module"]
#[doc(alias = "MMC_INTR_TX")]
pub type MmcIntrTx = crate::Reg<mmc_intr_tx::MmcIntrTxSpec>;
#[doc = "MMC Transmit Interrupt Register"]
pub mod mmc_intr_tx;
#[doc = "MMC_INTR_MASK_RX (rw) register accessor: MMC Receive Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_intr_mask_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_intr_mask_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_intr_mask_rx`] module"]
#[doc(alias = "MMC_INTR_MASK_RX")]
pub type MmcIntrMaskRx = crate::Reg<mmc_intr_mask_rx::MmcIntrMaskRxSpec>;
#[doc = "MMC Receive Interrupt Mask Register"]
pub mod mmc_intr_mask_rx;
#[doc = "MMC_INTR_MASK_TX (rw) register accessor: MMC Transmit Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_intr_mask_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_intr_mask_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_intr_mask_tx`] module"]
#[doc(alias = "MMC_INTR_MASK_TX")]
pub type MmcIntrMaskTx = crate::Reg<mmc_intr_mask_tx::MmcIntrMaskTxSpec>;
#[doc = "MMC Transmit Interrupt Mask Register"]
pub mod mmc_intr_mask_tx;
#[doc = "TXOCTETCOUNT_GB (r) register accessor: MMC Transmit Count\n\nYou can [`read`](crate::Reg::read) this register and get [`txoctetcount_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txoctetcount_gb`] module"]
#[doc(alias = "TXOCTETCOUNT_GB")]
pub type TxoctetcountGb = crate::Reg<txoctetcount_gb::TxoctetcountGbSpec>;
#[doc = "MMC Transmit Count"]
pub mod txoctetcount_gb;
#[doc = "TXFRAMECOUNT_GB (r) register accessor: MMC Frame Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txframecount_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txframecount_gb`] module"]
#[doc(alias = "TXFRAMECOUNT_GB")]
pub type TxframecountGb = crate::Reg<txframecount_gb::TxframecountGbSpec>;
#[doc = "MMC Frame Count Register"]
pub mod txframecount_gb;
#[doc = "TXBCASTFRAMES_G (r) register accessor: MMC Good Broadcast Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcastframes_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcastframes_g`] module"]
#[doc(alias = "TXBCASTFRAMES_G")]
pub type TxbcastframesG = crate::Reg<txbcastframes_g::TxbcastframesGSpec>;
#[doc = "MMC Good Broadcast Frames Register"]
pub mod txbcastframes_g;
#[doc = "TXMCASTFRAMES_G (r) register accessor: MMC Good Multicast Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txmcastframes_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmcastframes_g`] module"]
#[doc(alias = "TXMCASTFRAMES_G")]
pub type TxmcastframesG = crate::Reg<txmcastframes_g::TxmcastframesGSpec>;
#[doc = "MMC Good Multicast Frames Register"]
pub mod txmcastframes_g;
#[doc = "TX64OCT_GB (r) register accessor: MMC Good and bad Frames transmitted with length 64\n\nYou can [`read`](crate::Reg::read) this register and get [`tx64oct_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx64oct_gb`] module"]
#[doc(alias = "TX64OCT_GB")]
pub type Tx64octGb = crate::Reg<tx64oct_gb::Tx64octGbSpec>;
#[doc = "MMC Good and bad Frames transmitted with length 64"]
pub mod tx64oct_gb;
#[doc = "TX65TO127OCT_GB (r) register accessor: MMC Good and bad Frames transmitted with length 65 to 127\n\nYou can [`read`](crate::Reg::read) this register and get [`tx65to127oct_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx65to127oct_gb`] module"]
#[doc(alias = "TX65TO127OCT_GB")]
pub type Tx65to127octGb = crate::Reg<tx65to127oct_gb::Tx65to127octGbSpec>;
#[doc = "MMC Good and bad Frames transmitted with length 65 to 127"]
pub mod tx65to127oct_gb;
#[doc = "TX128TO255OCT_GB (r) register accessor: MMC Good and bad Frames transmitted with length 128 to 255\n\nYou can [`read`](crate::Reg::read) this register and get [`tx128to255oct_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx128to255oct_gb`] module"]
#[doc(alias = "TX128TO255OCT_GB")]
pub type Tx128to255octGb = crate::Reg<tx128to255oct_gb::Tx128to255octGbSpec>;
#[doc = "MMC Good and bad Frames transmitted with length 128 to 255"]
pub mod tx128to255oct_gb;
#[doc = "TX256TO511OCT_GB (r) register accessor: MMC Good and bad Frames transmitted with length 256 to 511\n\nYou can [`read`](crate::Reg::read) this register and get [`tx256to511oct_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx256to511oct_gb`] module"]
#[doc(alias = "TX256TO511OCT_GB")]
pub type Tx256to511octGb = crate::Reg<tx256to511oct_gb::Tx256to511octGbSpec>;
#[doc = "MMC Good and bad Frames transmitted with length 256 to 511"]
pub mod tx256to511oct_gb;
#[doc = "TX512TO1023OCT_GB (r) register accessor: MMC Good and bad Frames transmitted with length 512 to 1023\n\nYou can [`read`](crate::Reg::read) this register and get [`tx512to1023oct_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx512to1023oct_gb`] module"]
#[doc(alias = "TX512TO1023OCT_GB")]
pub type Tx512to1023octGb = crate::Reg<tx512to1023oct_gb::Tx512to1023octGbSpec>;
#[doc = "MMC Good and bad Frames transmitted with length 512 to 1023"]
pub mod tx512to1023oct_gb;
#[doc = "TX1024MAXOCT_GB (r) register accessor: MMC Good and bad Frames transmitted with length 1024 to max bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`tx1024maxoct_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx1024maxoct_gb`] module"]
#[doc(alias = "TX1024MAXOCT_GB")]
pub type Tx1024maxoctGb = crate::Reg<tx1024maxoct_gb::Tx1024maxoctGbSpec>;
#[doc = "MMC Good and bad Frames transmitted with length 1024 to max bytes"]
pub mod tx1024maxoct_gb;
#[doc = "TXUCASTFRAME_GB (r) register accessor: MMC number of good and bad unicast frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`txucastframe_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txucastframe_gb`] module"]
#[doc(alias = "TXUCASTFRAME_GB")]
pub type TxucastframeGb = crate::Reg<txucastframe_gb::TxucastframeGbSpec>;
#[doc = "MMC number of good and bad unicast frames transmitted"]
pub mod txucastframe_gb;
#[doc = "TXMCASTFRAME_GB (r) register accessor: MMC number of good and bad MULTIcast frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`txmcastframe_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmcastframe_gb`] module"]
#[doc(alias = "TXMCASTFRAME_GB")]
pub type TxmcastframeGb = crate::Reg<txmcastframe_gb::TxmcastframeGbSpec>;
#[doc = "MMC number of good and bad MULTIcast frames transmitted"]
pub mod txmcastframe_gb;
#[doc = "TXBCASTFRAME_GB (r) register accessor: MMC number of good and bad broadcast frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcastframe_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcastframe_gb`] module"]
#[doc(alias = "TXBCASTFRAME_GB")]
pub type TxbcastframeGb = crate::Reg<txbcastframe_gb::TxbcastframeGbSpec>;
#[doc = "MMC number of good and bad broadcast frames transmitted"]
pub mod txbcastframe_gb;
#[doc = "TXUNDERERR (r) register accessor: MMC number of frames aborted because of frame underflow error\n\nYou can [`read`](crate::Reg::read) this register and get [`txundererr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txundererr`] module"]
#[doc(alias = "TXUNDERERR")]
pub type Txundererr = crate::Reg<txundererr::TxundererrSpec>;
#[doc = "MMC number of frames aborted because of frame underflow error"]
pub mod txundererr;
#[doc = "TXSINGLECOL_G (r) register accessor: MMC Number of successfully transmitted frames after a single collision\n\nYou can [`read`](crate::Reg::read) this register and get [`txsinglecol_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txsinglecol_g`] module"]
#[doc(alias = "TXSINGLECOL_G")]
pub type TxsinglecolG = crate::Reg<txsinglecol_g::TxsinglecolGSpec>;
#[doc = "MMC Number of successfully transmitted frames after a single collision"]
pub mod txsinglecol_g;
#[doc = "TXMULTICOL_G (r) register accessor: MMC Number of successfully transmitted frames after multiple collisions\n\nYou can [`read`](crate::Reg::read) this register and get [`txmulticol_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmulticol_g`] module"]
#[doc(alias = "TXMULTICOL_G")]
pub type TxmulticolG = crate::Reg<txmulticol_g::TxmulticolGSpec>;
#[doc = "MMC Number of successfully transmitted frames after multiple collisions"]
pub mod txmulticol_g;
#[doc = "TXDEFERRED (r) register accessor: MMC Number of successfully transmitted frames after a deferral\n\nYou can [`read`](crate::Reg::read) this register and get [`txdeferred::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdeferred`] module"]
#[doc(alias = "TXDEFERRED")]
pub type Txdeferred = crate::Reg<txdeferred::TxdeferredSpec>;
#[doc = "MMC Number of successfully transmitted frames after a deferral"]
pub mod txdeferred;
#[doc = "TXLATECOL (r) register accessor: MMC Number of aborted frames because of late collision error\n\nYou can [`read`](crate::Reg::read) this register and get [`txlatecol::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlatecol`] module"]
#[doc(alias = "TXLATECOL")]
pub type Txlatecol = crate::Reg<txlatecol::TxlatecolSpec>;
#[doc = "MMC Number of aborted frames because of late collision error"]
pub mod txlatecol;
#[doc = "TXEXESSCOL (r) register accessor: MMC Number of aborted frames because of excessive collision errors\n\nYou can [`read`](crate::Reg::read) this register and get [`txexesscol::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txexesscol`] module"]
#[doc(alias = "TXEXESSCOL")]
pub type Txexesscol = crate::Reg<txexesscol::TxexesscolSpec>;
#[doc = "MMC Number of aborted frames because of excessive collision errors"]
pub mod txexesscol;
#[doc = "TXCARRIERERROR (r) register accessor: MMC Number of aborted frames because of carrier sense error\n\nYou can [`read`](crate::Reg::read) this register and get [`txcarriererror::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcarriererror`] module"]
#[doc(alias = "TXCARRIERERROR")]
pub type Txcarriererror = crate::Reg<txcarriererror::TxcarriererrorSpec>;
#[doc = "MMC Number of aborted frames because of carrier sense error"]
pub mod txcarriererror;
#[doc = "TXOCTETCOUNT_G (r) register accessor: MMC Number of bytes transmitted frames only in good frames\n\nYou can [`read`](crate::Reg::read) this register and get [`txoctetcount_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txoctetcount_g`] module"]
#[doc(alias = "TXOCTETCOUNT_G")]
pub type TxoctetcountG = crate::Reg<txoctetcount_g::TxoctetcountGSpec>;
#[doc = "MMC Number of bytes transmitted frames only in good frames"]
pub mod txoctetcount_g;
#[doc = "TXFRAMECOUNT_G (r) register accessor: MMC Number of good frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`txframecount_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txframecount_g`] module"]
#[doc(alias = "TXFRAMECOUNT_G")]
pub type TxframecountG = crate::Reg<txframecount_g::TxframecountGSpec>;
#[doc = "MMC Number of good frames transmitted"]
pub mod txframecount_g;
#[doc = "TXEXCESSDEF (r) register accessor: MMC Number of frames aborted because of excessive deferral error\n\nYou can [`read`](crate::Reg::read) this register and get [`txexcessdef::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txexcessdef`] module"]
#[doc(alias = "TXEXCESSDEF")]
pub type Txexcessdef = crate::Reg<txexcessdef::TxexcessdefSpec>;
#[doc = "MMC Number of frames aborted because of excessive deferral error"]
pub mod txexcessdef;
#[doc = "TXPAUSEFRAMES (r) register accessor: MMC Number of good pause frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`txpauseframes::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpauseframes`] module"]
#[doc(alias = "TXPAUSEFRAMES")]
pub type Txpauseframes = crate::Reg<txpauseframes::TxpauseframesSpec>;
#[doc = "MMC Number of good pause frames transmitted"]
pub mod txpauseframes;
#[doc = "TXLANFRAMES_G (r) register accessor: MMC Number of good VLAN frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`txlanframes_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txlanframes_g`] module"]
#[doc(alias = "TXLANFRAMES_G")]
pub type TxlanframesG = crate::Reg<txlanframes_g::TxlanframesGSpec>;
#[doc = "MMC Number of good VLAN frames transmitted"]
pub mod txlanframes_g;
#[doc = "TXOVERSIZE_G (r) register accessor: MMC Number of frames transmitted without errors\n\nYou can [`read`](crate::Reg::read) this register and get [`txoversize_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txoversize_g`] module"]
#[doc(alias = "TXOVERSIZE_G")]
pub type TxoversizeG = crate::Reg<txoversize_g::TxoversizeGSpec>;
#[doc = "MMC Number of frames transmitted without errors"]
pub mod txoversize_g;
#[doc = "RXFRAMECOUNT_GB (r) register accessor: MMC Number of good and bad frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`rxframecount_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxframecount_gb`] module"]
#[doc(alias = "RXFRAMECOUNT_GB")]
pub type RxframecountGb = crate::Reg<rxframecount_gb::RxframecountGbSpec>;
#[doc = "MMC Number of good and bad frames received"]
pub mod rxframecount_gb;
#[doc = "RXOCTETCOUNT_GB (r) register accessor: MMC Number of bytes received in good and bad frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoctetcount_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxoctetcount_gb`] module"]
#[doc(alias = "RXOCTETCOUNT_GB")]
pub type RxoctetcountGb = crate::Reg<rxoctetcount_gb::RxoctetcountGbSpec>;
#[doc = "MMC Number of bytes received in good and bad frames"]
pub mod rxoctetcount_gb;
#[doc = "RXOCTETCOUNT_G (r) register accessor: MMC Number of bytes received in good frames only\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoctetcount_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxoctetcount_g`] module"]
#[doc(alias = "RXOCTETCOUNT_G")]
pub type RxoctetcountG = crate::Reg<rxoctetcount_g::RxoctetcountGSpec>;
#[doc = "MMC Number of bytes received in good frames only"]
pub mod rxoctetcount_g;
#[doc = "RXBCASTFRAMES_G (r) register accessor: MMC Number of good broadcast frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbcastframes_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbcastframes_g`] module"]
#[doc(alias = "RXBCASTFRAMES_G")]
pub type RxbcastframesG = crate::Reg<rxbcastframes_g::RxbcastframesGSpec>;
#[doc = "MMC Number of good broadcast frames received"]
pub mod rxbcastframes_g;
#[doc = "RXMCASTFRAMES_G (r) register accessor: MMC Number of good multicast frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmcastframes_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmcastframes_g`] module"]
#[doc(alias = "RXMCASTFRAMES_G")]
pub type RxmcastframesG = crate::Reg<rxmcastframes_g::RxmcastframesGSpec>;
#[doc = "MMC Number of good multicast frames received"]
pub mod rxmcastframes_g;
#[doc = "RXCRCERROR (r) register accessor: MMC Number of frames received with CRC error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcrcerror::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcrcerror`] module"]
#[doc(alias = "RXCRCERROR")]
pub type Rxcrcerror = crate::Reg<rxcrcerror::RxcrcerrorSpec>;
#[doc = "MMC Number of frames received with CRC error"]
pub mod rxcrcerror;
#[doc = "RXALIGNERROR (r) register accessor: MMC Number of frames received with alignment error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxalignerror::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxalignerror`] module"]
#[doc(alias = "RXALIGNERROR")]
pub type Rxalignerror = crate::Reg<rxalignerror::RxalignerrorSpec>;
#[doc = "MMC Number of frames received with alignment error"]
pub mod rxalignerror;
#[doc = "RXRUNTERROR (r) register accessor: MMC Number of frames received with runt error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxrunterror::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxrunterror`] module"]
#[doc(alias = "RXRUNTERROR")]
pub type Rxrunterror = crate::Reg<rxrunterror::RxrunterrorSpec>;
#[doc = "MMC Number of frames received with runt error"]
pub mod rxrunterror;
#[doc = "RXJABBERERROR (r) register accessor: MMC Number of giant frames received with length greater than 1518 bytes and with CRC error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxjabbererror::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxjabbererror`] module"]
#[doc(alias = "RXJABBERERROR")]
pub type Rxjabbererror = crate::Reg<rxjabbererror::RxjabbererrorSpec>;
#[doc = "MMC Number of giant frames received with length greater than 1518 bytes and with CRC error"]
pub mod rxjabbererror;
#[doc = "RXUNDERSIZE_G (r) register accessor: MMC Number of frames received with length less than 64 bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rxundersize_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxundersize_g`] module"]
#[doc(alias = "RXUNDERSIZE_G")]
pub type RxundersizeG = crate::Reg<rxundersize_g::RxundersizeGSpec>;
#[doc = "MMC Number of frames received with length less than 64 bytes"]
pub mod rxundersize_g;
#[doc = "RXOVERSIZE_G (r) register accessor: MMC Number of frames received without errors with length greater than the max size\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoversize_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxoversize_g`] module"]
#[doc(alias = "RXOVERSIZE_G")]
pub type RxoversizeG = crate::Reg<rxoversize_g::RxoversizeGSpec>;
#[doc = "MMC Number of frames received without errors with length greater than the max size"]
pub mod rxoversize_g;
#[doc = "RX64OCTETS_GB (r) register accessor: MMC Number of good and bad frames received with length 64 bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rx64octets_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx64octets_gb`] module"]
#[doc(alias = "RX64OCTETS_GB")]
pub type Rx64octetsGb = crate::Reg<rx64octets_gb::Rx64octetsGbSpec>;
#[doc = "MMC Number of good and bad frames received with length 64 bytes"]
pub mod rx64octets_gb;
#[doc = "RX65TO127OCT_GB (r) register accessor: MMC Number of good and bad frames received with length between 65 and 127 bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rx65to127oct_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx65to127oct_gb`] module"]
#[doc(alias = "RX65TO127OCT_GB")]
pub type Rx65to127octGb = crate::Reg<rx65to127oct_gb::Rx65to127octGbSpec>;
#[doc = "MMC Number of good and bad frames received with length between 65 and 127 bytes"]
pub mod rx65to127oct_gb;
#[doc = "RX128TO255OCT_GB (r) register accessor: MMC Number of good and bad frames received with length between 128 and 255 bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rx128to255oct_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx128to255oct_gb`] module"]
#[doc(alias = "RX128TO255OCT_GB")]
pub type Rx128to255octGb = crate::Reg<rx128to255oct_gb::Rx128to255octGbSpec>;
#[doc = "MMC Number of good and bad frames received with length between 128 and 255 bytes"]
pub mod rx128to255oct_gb;
#[doc = "RX256TO511OCT_GB (r) register accessor: MMC Number of good and bad frames received with length between 256 and 511 bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rx256to511oct_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx256to511oct_gb`] module"]
#[doc(alias = "RX256TO511OCT_GB")]
pub type Rx256to511octGb = crate::Reg<rx256to511oct_gb::Rx256to511octGbSpec>;
#[doc = "MMC Number of good and bad frames received with length between 256 and 511 bytes"]
pub mod rx256to511oct_gb;
#[doc = "RX512TO1023OCT_GB (r) register accessor: MMC Number of good and bad frames received with length between 512 and 1023 bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rx512to1023oct_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx512to1023oct_gb`] module"]
#[doc(alias = "RX512TO1023OCT_GB")]
pub type Rx512to1023octGb = crate::Reg<rx512to1023oct_gb::Rx512to1023octGbSpec>;
#[doc = "MMC Number of good and bad frames received with length between 512 and 1023 bytes"]
pub mod rx512to1023oct_gb;
#[doc = "RX1024MAXOCT_GB (r) register accessor: MMC Number of good and bad frames received with length between 1024 and max size bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rx1024maxoct_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx1024maxoct_gb`] module"]
#[doc(alias = "RX1024MAXOCT_GB")]
pub type Rx1024maxoctGb = crate::Reg<rx1024maxoct_gb::Rx1024maxoctGbSpec>;
#[doc = "MMC Number of good and bad frames received with length between 1024 and max size bytes"]
pub mod rx1024maxoct_gb;
#[doc = "RXUCASTFRAMES_G (r) register accessor: MMC Number of received good unicast frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rxucastframes_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxucastframes_g`] module"]
#[doc(alias = "RXUCASTFRAMES_G")]
pub type RxucastframesG = crate::Reg<rxucastframes_g::RxucastframesGSpec>;
#[doc = "MMC Number of received good unicast frames"]
pub mod rxucastframes_g;
#[doc = "RXLENGTHERROR (r) register accessor: MMC Number of frames received with length error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxlengtherror::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxlengtherror`] module"]
#[doc(alias = "RXLENGTHERROR")]
pub type Rxlengtherror = crate::Reg<rxlengtherror::RxlengtherrorSpec>;
#[doc = "MMC Number of frames received with length error"]
pub mod rxlengtherror;
#[doc = "RXOUTRANGETYPE (r) register accessor: MMC Number of frames received with length field not equal to the valid frame size\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoutrangetype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxoutrangetype`] module"]
#[doc(alias = "RXOUTRANGETYPE")]
pub type Rxoutrangetype = crate::Reg<rxoutrangetype::RxoutrangetypeSpec>;
#[doc = "MMC Number of frames received with length field not equal to the valid frame size"]
pub mod rxoutrangetype;
#[doc = "RXPAUSEFRAMES (r) register accessor: MMC Number of good and valid Pause frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`rxpauseframes::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxpauseframes`] module"]
#[doc(alias = "RXPAUSEFRAMES")]
pub type Rxpauseframes = crate::Reg<rxpauseframes::RxpauseframesSpec>;
#[doc = "MMC Number of good and valid Pause frames received"]
pub mod rxpauseframes;
#[doc = "RXFIFOOVERFLOW (r) register accessor: MMC Number of missed received frames because of FIFO overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifooverflow::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifooverflow`] module"]
#[doc(alias = "RXFIFOOVERFLOW")]
pub type Rxfifooverflow = crate::Reg<rxfifooverflow::RxfifooverflowSpec>;
#[doc = "MMC Number of missed received frames because of FIFO overflow"]
pub mod rxfifooverflow;
#[doc = "RXVLANFRAMES_GB (r) register accessor: MMC Number of good and bad VLAN frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`rxvlanframes_gb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxvlanframes_gb`] module"]
#[doc(alias = "RXVLANFRAMES_GB")]
pub type RxvlanframesGb = crate::Reg<rxvlanframes_gb::RxvlanframesGbSpec>;
#[doc = "MMC Number of good and bad VLAN frames received"]
pub mod rxvlanframes_gb;
#[doc = "RXWDOGERROR (r) register accessor: MMC Number of frames received with error because of watchdog timeout error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxwdogerror::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxwdogerror`] module"]
#[doc(alias = "RXWDOGERROR")]
pub type Rxwdogerror = crate::Reg<rxwdogerror::RxwdogerrorSpec>;
#[doc = "MMC Number of frames received with error because of watchdog timeout error"]
pub mod rxwdogerror;
#[doc = "RXRCVERROR (r) register accessor: MMC Number of frames received with Receive error or Frame Extension error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxrcverror::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxrcverror`] module"]
#[doc(alias = "RXRCVERROR")]
pub type Rxrcverror = crate::Reg<rxrcverror::RxrcverrorSpec>;
#[doc = "MMC Number of frames received with Receive error or Frame Extension error"]
pub mod rxrcverror;
#[doc = "RXCTRLFRAMES_G (r) register accessor: MMC Number of received good control frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctrlframes_g::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxctrlframes_g`] module"]
#[doc(alias = "RXCTRLFRAMES_G")]
pub type RxctrlframesG = crate::Reg<rxctrlframes_g::RxctrlframesGSpec>;
#[doc = "MMC Number of received good control frames"]
pub mod rxctrlframes_g;
#[doc = "VLAN_INCREPLACE (rw) register accessor: Holds the VLAN Tag for insertion into or replacement in the transmit frames\n\nYou can [`read`](crate::Reg::read) this register and get [`vlan_increplace::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vlan_increplace::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlan_increplace`] module"]
#[doc(alias = "VLAN_INCREPLACE")]
pub type VlanIncreplace = crate::Reg<vlan_increplace::VlanIncreplaceSpec>;
#[doc = "Holds the VLAN Tag for insertion into or replacement in the transmit frames"]
pub mod vlan_increplace;
#[doc = "VLAN_HASHTABLE (rw) register accessor: Holds the VLAN Hash Table\n\nYou can [`read`](crate::Reg::read) this register and get [`vlan_hashtable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vlan_hashtable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlan_hashtable`] module"]
#[doc(alias = "VLAN_HASHTABLE")]
pub type VlanHashtable = crate::Reg<vlan_hashtable::VlanHashtableSpec>;
#[doc = "Holds the VLAN Hash Table"]
pub mod vlan_hashtable;
#[doc = "TIMESTAMP_CTRL (rw) register accessor: Controls the IEEE 1588 timestamp generation and update logic\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestamp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp_ctrl`] module"]
#[doc(alias = "TIMESTAMP_CTRL")]
pub type TimestampCtrl = crate::Reg<timestamp_ctrl::TimestampCtrlSpec>;
#[doc = "Controls the IEEE 1588 timestamp generation and update logic"]
pub mod timestamp_ctrl;
#[doc = "SUBSEC_INC (rw) register accessor: Holds the 8-bit value by which the Sub-Second register is incremented\n\nYou can [`read`](crate::Reg::read) this register and get [`subsec_inc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subsec_inc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subsec_inc`] module"]
#[doc(alias = "SUBSEC_INC")]
pub type SubsecInc = crate::Reg<subsec_inc::SubsecIncSpec>;
#[doc = "Holds the 8-bit value by which the Sub-Second register is incremented"]
pub mod subsec_inc;
#[doc = "SYSTIME_SECONDS (r) register accessor: Holds the lower 32 bits of the second field of the system time\n\nYou can [`read`](crate::Reg::read) this register and get [`systime_seconds::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systime_seconds`] module"]
#[doc(alias = "SYSTIME_SECONDS")]
pub type SystimeSeconds = crate::Reg<systime_seconds::SystimeSecondsSpec>;
#[doc = "Holds the lower 32 bits of the second field of the system time"]
pub mod systime_seconds;
#[doc = "SYSTIME_NANOSEC (r) register accessor: Holds 32 bits of the nano-second field of the system time\n\nYou can [`read`](crate::Reg::read) this register and get [`systime_nanosec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systime_nanosec`] module"]
#[doc(alias = "SYSTIME_NANOSEC")]
pub type SystimeNanosec = crate::Reg<systime_nanosec::SystimeNanosecSpec>;
#[doc = "Holds 32 bits of the nano-second field of the system time"]
pub mod systime_nanosec;
#[doc = "SYSTIME_SECSUPDAT (rw) register accessor: Holds the lower 32 bits of the second field to be written to, added to, or subtracted from the system time value\n\nYou can [`read`](crate::Reg::read) this register and get [`systime_secsupdat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systime_secsupdat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systime_secsupdat`] module"]
#[doc(alias = "SYSTIME_SECSUPDAT")]
pub type SystimeSecsupdat = crate::Reg<systime_secsupdat::SystimeSecsupdatSpec>;
#[doc = "Holds the lower 32 bits of the second field to be written to, added to, or subtracted from the system time value"]
pub mod systime_secsupdat;
#[doc = "SYSTIME_NSECUP (rw) register accessor: Holds 32 bits of the nano-second field to be written to, added to, or subtracted from the system time value\n\nYou can [`read`](crate::Reg::read) this register and get [`systime_nsecup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systime_nsecup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systime_nsecup`] module"]
#[doc(alias = "SYSTIME_NSECUP")]
pub type SystimeNsecup = crate::Reg<systime_nsecup::SystimeNsecupSpec>;
#[doc = "Holds 32 bits of the nano-second field to be written to, added to, or subtracted from the system time value"]
pub mod systime_nsecup;
#[doc = "TIMESTAMPADDEND (rw) register accessor: This register is used by software to re-adjust the clock frequency linearly to match the Master clock frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`timestampaddend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestampaddend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestampaddend`] module"]
#[doc(alias = "TIMESTAMPADDEND")]
pub type Timestampaddend = crate::Reg<timestampaddend::TimestampaddendSpec>;
#[doc = "This register is used by software to re-adjust the clock frequency linearly to match the Master clock frequency"]
pub mod timestampaddend;
#[doc = "TARGET_TIME_SECS (rw) register accessor: Holds the high 32-bits of time to be compared with the system time\n\nYou can [`read`](crate::Reg::read) this register and get [`target_time_secs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target_time_secs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target_time_secs`] module"]
#[doc(alias = "TARGET_TIME_SECS")]
pub type TargetTimeSecs = crate::Reg<target_time_secs::TargetTimeSecsSpec>;
#[doc = "Holds the high 32-bits of time to be compared with the system time"]
pub mod target_time_secs;
#[doc = "TARGET_TIME_NSEC (rw) register accessor: Holds the lower 32-bits of time to be compared with the system time\n\nYou can [`read`](crate::Reg::read) this register and get [`target_time_nsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target_time_nsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target_time_nsec`] module"]
#[doc(alias = "TARGET_TIME_NSEC")]
pub type TargetTimeNsec = crate::Reg<target_time_nsec::TargetTimeNsecSpec>;
#[doc = "Holds the lower 32-bits of time to be compared with the system time"]
pub mod target_time_nsec;
#[doc = "DMA_BUS_MODE (rw) register accessor: Controls the DMA Host Interface Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_bus_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_bus_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_bus_mode`] module"]
#[doc(alias = "DMA_BUS_MODE")]
pub type DmaBusMode = crate::Reg<dma_bus_mode::DmaBusModeSpec>;
#[doc = "Controls the DMA Host Interface Mode"]
pub mod dma_bus_mode;
#[doc = "DMA_TX_POLL_DEMAND (rw) register accessor: Used by the host to instruct the DMA to poll the transmit Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tx_poll_demand::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tx_poll_demand::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tx_poll_demand`] module"]
#[doc(alias = "DMA_TX_POLL_DEMAND")]
pub type DmaTxPollDemand = crate::Reg<dma_tx_poll_demand::DmaTxPollDemandSpec>;
#[doc = "Used by the host to instruct the DMA to poll the transmit Descriptor list"]
pub mod dma_tx_poll_demand;
#[doc = "DMA_RX_POLL_DEMAND (rw) register accessor: Used by the host to instruct the DMA to poll the Receive Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rx_poll_demand::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rx_poll_demand::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_rx_poll_demand`] module"]
#[doc(alias = "DMA_RX_POLL_DEMAND")]
pub type DmaRxPollDemand = crate::Reg<dma_rx_poll_demand::DmaRxPollDemandSpec>;
#[doc = "Used by the host to instruct the DMA to poll the Receive Descriptor list"]
pub mod dma_rx_poll_demand;
#[doc = "DMA_RX_DESC_LIST_ADDR (rw) register accessor: Points the DMA to the start of the Receive Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rx_desc_list_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rx_desc_list_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_rx_desc_list_addr`] module"]
#[doc(alias = "DMA_RX_DESC_LIST_ADDR")]
pub type DmaRxDescListAddr = crate::Reg<dma_rx_desc_list_addr::DmaRxDescListAddrSpec>;
#[doc = "Points the DMA to the start of the Receive Descriptor list"]
pub mod dma_rx_desc_list_addr;
#[doc = "DMA_TX_DESC_LIST_ADDR (rw) register accessor: Points the DMA to the start of the Transmit Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tx_desc_list_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tx_desc_list_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tx_desc_list_addr`] module"]
#[doc(alias = "DMA_TX_DESC_LIST_ADDR")]
pub type DmaTxDescListAddr = crate::Reg<dma_tx_desc_list_addr::DmaTxDescListAddrSpec>;
#[doc = "Points the DMA to the start of the Transmit Descriptor list"]
pub mod dma_tx_desc_list_addr;
#[doc = "DMA_STATUS (r) register accessor: Used to determine the status of the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_status`] module"]
#[doc(alias = "DMA_STATUS")]
pub type DmaStatus = crate::Reg<dma_status::DmaStatusSpec>;
#[doc = "Used to determine the status of the DMA"]
pub mod dma_status;
#[doc = "DMA_OPER_MODE (rw) register accessor: Sets the Receive and Transmit operation mode and command\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_oper_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_oper_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_oper_mode`] module"]
#[doc(alias = "DMA_OPER_MODE")]
pub type DmaOperMode = crate::Reg<dma_oper_mode::DmaOperModeSpec>;
#[doc = "Sets the Receive and Transmit operation mode and command"]
pub mod dma_oper_mode;
#[doc = "DMA_INTR_EN (rw) register accessor: Enables the interrupts reported in the status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_intr_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_intr_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_intr_en`] module"]
#[doc(alias = "DMA_INTR_EN")]
pub type DmaIntrEn = crate::Reg<dma_intr_en::DmaIntrEnSpec>;
#[doc = "Enables the interrupts reported in the status register"]
pub mod dma_intr_en;
#[doc = "DMA_MISS_OVER_COUNTER (rw) register accessor: Contains the counters for discarded frames because no Receive Descriptor is available\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_miss_over_counter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_miss_over_counter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_miss_over_counter`] module"]
#[doc(alias = "DMA_MISS_OVER_COUNTER")]
pub type DmaMissOverCounter = crate::Reg<dma_miss_over_counter::DmaMissOverCounterSpec>;
#[doc = "Contains the counters for discarded frames because no Receive Descriptor is available"]
pub mod dma_miss_over_counter;
#[doc = "DMA_RX_INTR_WDOG_TIMER (rw) register accessor: Watchdog timeout for Receive Interrupt from DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rx_intr_wdog_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rx_intr_wdog_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_rx_intr_wdog_timer`] module"]
#[doc(alias = "DMA_RX_INTR_WDOG_TIMER")]
pub type DmaRxIntrWdogTimer = crate::Reg<dma_rx_intr_wdog_timer::DmaRxIntrWdogTimerSpec>;
#[doc = "Watchdog timeout for Receive Interrupt from DMA"]
pub mod dma_rx_intr_wdog_timer;
#[doc = "DMA_AHB_STATUS (rw) register accessor: Provides the active status of the read and write channels of the AHB master interface\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ahb_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ahb_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ahb_status`] module"]
#[doc(alias = "DMA_AHB_STATUS")]
pub type DmaAhbStatus = crate::Reg<dma_ahb_status::DmaAhbStatusSpec>;
#[doc = "Provides the active status of the read and write channels of the AHB master interface"]
pub mod dma_ahb_status;
#[doc = "DMA_CURR_TX_DESC (rw) register accessor: Contains the start address of the current Transmit Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_curr_tx_desc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_curr_tx_desc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_curr_tx_desc`] module"]
#[doc(alias = "DMA_CURR_TX_DESC")]
pub type DmaCurrTxDesc = crate::Reg<dma_curr_tx_desc::DmaCurrTxDescSpec>;
#[doc = "Contains the start address of the current Transmit Descriptor read by the DMA"]
pub mod dma_curr_tx_desc;
#[doc = "DMA_CURR_RX_DESC (rw) register accessor: Contains the start address of the current Receive Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_curr_rx_desc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_curr_rx_desc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_curr_rx_desc`] module"]
#[doc(alias = "DMA_CURR_RX_DESC")]
pub type DmaCurrRxDesc = crate::Reg<dma_curr_rx_desc::DmaCurrRxDescSpec>;
#[doc = "Contains the start address of the current Receive Descriptor read by the DMA"]
pub mod dma_curr_rx_desc;
#[doc = "DMA_CURR_TX_BUFR_ADDR (rw) register accessor: Contains the start address of the current Receive Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_curr_tx_bufr_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_curr_tx_bufr_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_curr_tx_bufr_addr`] module"]
#[doc(alias = "DMA_CURR_TX_BUFR_ADDR")]
pub type DmaCurrTxBufrAddr = crate::Reg<dma_curr_tx_bufr_addr::DmaCurrTxBufrAddrSpec>;
#[doc = "Contains the start address of the current Receive Descriptor read by the DMA"]
pub mod dma_curr_tx_bufr_addr;
#[doc = "DMA_CURR_RX_BUFR_ADDR (rw) register accessor: Contains the current Receive Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_curr_rx_bufr_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_curr_rx_bufr_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_curr_rx_bufr_addr`] module"]
#[doc(alias = "DMA_CURR_RX_BUFR_ADDR")]
pub type DmaCurrRxBufrAddr = crate::Reg<dma_curr_rx_bufr_addr::DmaCurrRxBufrAddrSpec>;
#[doc = "Contains the current Receive Buffer address read by the DMA"]
pub mod dma_curr_rx_bufr_addr;
