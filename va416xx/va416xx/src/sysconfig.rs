#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rst_stat: RstStat,
    rst_cntl_rom: RstCntlRom,
    rst_cntl_ram0: RstCntlRam0,
    rst_cntl_ram1: RstCntlRam1,
    rom_prot: RomProt,
    rom_scrub: RomScrub,
    ram0_scrub: Ram0Scrub,
    ram1_scrub: Ram1Scrub,
    irq_enb: IrqEnb,
    irq_raw: IrqRaw,
    irq_end: IrqEnd,
    irq_clr: IrqClr,
    ram0_sbe: Ram0Sbe,
    ram1_sbe: Ram1Sbe,
    ram0_mbe: Ram0Mbe,
    ram1_mbe: Ram1Mbe,
    rom_sbe: RomSbe,
    rom_mbe: RomMbe,
    rom_retries: RomRetries,
    refresh_config_h: RefreshConfigH,
    tim_reset: TimReset,
    tim_clk_enable: TimClkEnable,
    peripheral_reset: PeripheralReset,
    peripheral_clk_enable: PeripheralClkEnable,
    spw_m4_ctrl: SpwM4Ctrl,
    pmu_ctrl: PmuCtrl,
    wakeup_cnt: WakeupCnt,
    ebi_cfg0: EbiCfg0,
    ebi_cfg1: EbiCfg1,
    ebi_cfg2: EbiCfg2,
    ebi_cfg3: EbiCfg3,
    analog_cntl: AnalogCntl,
    sw_clkdiv10: SwClkdiv10,
    refresh_config_l: RefreshConfigL,
    _reserved34: [u8; 0x0f48],
    dac0_cal: Dac0Cal,
    dac1_cal: Dac1Cal,
    adc_cal: AdcCal,
    bg_cal: BgCal,
    dreg_cal: DregCal,
    areg_cal: AregCal,
    hbo_cal: HboCal,
    ef_config: EfConfig,
    ef_id0: EfId0,
    ef_id1: EfId1,
    procid: Procid,
    perid: Perid,
}
impl RegisterBlock {
    #[doc = "0x00 - System Reset Status"]
    #[inline(always)]
    pub const fn rst_stat(&self) -> &RstStat {
        &self.rst_stat
    }
    #[doc = "0x04 - ROM Reset Control"]
    #[inline(always)]
    pub const fn rst_cntl_rom(&self) -> &RstCntlRom {
        &self.rst_cntl_rom
    }
    #[doc = "0x08 - RAM Reset Control"]
    #[inline(always)]
    pub const fn rst_cntl_ram0(&self) -> &RstCntlRam0 {
        &self.rst_cntl_ram0
    }
    #[doc = "0x0c - RAM Reset Control"]
    #[inline(always)]
    pub const fn rst_cntl_ram1(&self) -> &RstCntlRam1 {
        &self.rst_cntl_ram1
    }
    #[doc = "0x10 - ROM Protection Configuration"]
    #[inline(always)]
    pub const fn rom_prot(&self) -> &RomProt {
        &self.rom_prot
    }
    #[doc = "0x14 - ROM Scrub Period Configuration"]
    #[inline(always)]
    pub const fn rom_scrub(&self) -> &RomScrub {
        &self.rom_scrub
    }
    #[doc = "0x18 - RAM0 Scrub Period Configuration"]
    #[inline(always)]
    pub const fn ram0_scrub(&self) -> &Ram0Scrub {
        &self.ram0_scrub
    }
    #[doc = "0x1c - RAM1 Scrub Period Configuration"]
    #[inline(always)]
    pub const fn ram1_scrub(&self) -> &Ram1Scrub {
        &self.ram1_scrub
    }
    #[doc = "0x20 - Enable EDAC Error Interrupt Register"]
    #[inline(always)]
    pub const fn irq_enb(&self) -> &IrqEnb {
        &self.irq_enb
    }
    #[doc = "0x24 - Raw EDAC Error Interrupt Status"]
    #[inline(always)]
    pub const fn irq_raw(&self) -> &IrqRaw {
        &self.irq_raw
    }
    #[doc = "0x28 - Enabled EDAC Error Interrupt Status"]
    #[inline(always)]
    pub const fn irq_end(&self) -> &IrqEnd {
        &self.irq_end
    }
    #[doc = "0x2c - Clear EDAC Error Interrupt Status"]
    #[inline(always)]
    pub const fn irq_clr(&self) -> &IrqClr {
        &self.irq_clr
    }
    #[doc = "0x30 - Count of RAM0 EDAC Single Bit Errors"]
    #[inline(always)]
    pub const fn ram0_sbe(&self) -> &Ram0Sbe {
        &self.ram0_sbe
    }
    #[doc = "0x34 - Count of RAM1 EDAC Single Bit Errors"]
    #[inline(always)]
    pub const fn ram1_sbe(&self) -> &Ram1Sbe {
        &self.ram1_sbe
    }
    #[doc = "0x38 - Count of RAM0 EDAC Multi Bit Errors"]
    #[inline(always)]
    pub const fn ram0_mbe(&self) -> &Ram0Mbe {
        &self.ram0_mbe
    }
    #[doc = "0x3c - Count of RAM1 EDAC Multi Bit Errors"]
    #[inline(always)]
    pub const fn ram1_mbe(&self) -> &Ram1Mbe {
        &self.ram1_mbe
    }
    #[doc = "0x40 - Count of ROM EDAC Single Bit Errors"]
    #[inline(always)]
    pub const fn rom_sbe(&self) -> &RomSbe {
        &self.rom_sbe
    }
    #[doc = "0x44 - Count of ROM EDAC Multi Bit Errors"]
    #[inline(always)]
    pub const fn rom_mbe(&self) -> &RomMbe {
        &self.rom_mbe
    }
    #[doc = "0x48 - ROM BOOT Retry count"]
    #[inline(always)]
    pub const fn rom_retries(&self) -> &RomRetries {
        &self.rom_retries
    }
    #[doc = "0x4c - Register Refresh Rate for TMR registers"]
    #[inline(always)]
    pub const fn refresh_config_h(&self) -> &RefreshConfigH {
        &self.refresh_config_h
    }
    #[doc = "0x50 - TIM Reset Control"]
    #[inline(always)]
    pub const fn tim_reset(&self) -> &TimReset {
        &self.tim_reset
    }
    #[doc = "0x54 - TIM Enable Control"]
    #[inline(always)]
    pub const fn tim_clk_enable(&self) -> &TimClkEnable {
        &self.tim_clk_enable
    }
    #[doc = "0x58 - Peripheral Reset Control"]
    #[inline(always)]
    pub const fn peripheral_reset(&self) -> &PeripheralReset {
        &self.peripheral_reset
    }
    #[doc = "0x5c - Peripheral Enable Control"]
    #[inline(always)]
    pub const fn peripheral_clk_enable(&self) -> &PeripheralClkEnable {
        &self.peripheral_clk_enable
    }
    #[doc = "0x60 - SPW M4 control register"]
    #[inline(always)]
    pub const fn spw_m4_ctrl(&self) -> &SpwM4Ctrl {
        &self.spw_m4_ctrl
    }
    #[doc = "0x64 - PMU Control Register"]
    #[inline(always)]
    pub const fn pmu_ctrl(&self) -> &PmuCtrl {
        &self.pmu_ctrl
    }
    #[doc = "0x68 - Wakeup Control"]
    #[inline(always)]
    pub const fn wakeup_cnt(&self) -> &WakeupCnt {
        &self.wakeup_cnt
    }
    #[doc = "0x6c - EBI Config Register 0"]
    #[inline(always)]
    pub const fn ebi_cfg0(&self) -> &EbiCfg0 {
        &self.ebi_cfg0
    }
    #[doc = "0x70 - EBI Config Register 1"]
    #[inline(always)]
    pub const fn ebi_cfg1(&self) -> &EbiCfg1 {
        &self.ebi_cfg1
    }
    #[doc = "0x74 - EBI Config Register 2"]
    #[inline(always)]
    pub const fn ebi_cfg2(&self) -> &EbiCfg2 {
        &self.ebi_cfg2
    }
    #[doc = "0x78 - EBI Config Register 3"]
    #[inline(always)]
    pub const fn ebi_cfg3(&self) -> &EbiCfg3 {
        &self.ebi_cfg3
    }
    #[doc = "0x7c - Analog Control Register"]
    #[inline(always)]
    pub const fn analog_cntl(&self) -> &AnalogCntl {
        &self.analog_cntl
    }
    #[doc = "0x80 - Initial SpW Clock Divider Value"]
    #[inline(always)]
    pub const fn sw_clkdiv10(&self) -> &SwClkdiv10 {
        &self.sw_clkdiv10
    }
    #[doc = "0x84 - Register Refresh Rate for TMR registers"]
    #[inline(always)]
    pub const fn refresh_config_l(&self) -> &RefreshConfigL {
        &self.refresh_config_l
    }
    #[doc = "0xfd0 - DAC0 Calibration Register"]
    #[inline(always)]
    pub const fn dac0_cal(&self) -> &Dac0Cal {
        &self.dac0_cal
    }
    #[doc = "0xfd4 - DAC1 Calibration Register"]
    #[inline(always)]
    pub const fn dac1_cal(&self) -> &Dac1Cal {
        &self.dac1_cal
    }
    #[doc = "0xfd8 - ADC Calibration Register"]
    #[inline(always)]
    pub const fn adc_cal(&self) -> &AdcCal {
        &self.adc_cal
    }
    #[doc = "0xfdc - Bandgap Calibration Register"]
    #[inline(always)]
    pub const fn bg_cal(&self) -> &BgCal {
        &self.bg_cal
    }
    #[doc = "0xfe0 - Digital LDO Regulator Calibration Register"]
    #[inline(always)]
    pub const fn dreg_cal(&self) -> &DregCal {
        &self.dreg_cal
    }
    #[doc = "0xfe4 - Analog LDO Regulator Calibration Register"]
    #[inline(always)]
    pub const fn areg_cal(&self) -> &AregCal {
        &self.areg_cal
    }
    #[doc = "0xfe8 - Heart Beat OSC Calibration Register"]
    #[inline(always)]
    pub const fn hbo_cal(&self) -> &HboCal {
        &self.hbo_cal
    }
    #[doc = "0xfec - EFuse Config Register"]
    #[inline(always)]
    pub const fn ef_config(&self) -> &EfConfig {
        &self.ef_config
    }
    #[doc = "0xff0 - EFuse ID0 Register"]
    #[inline(always)]
    pub const fn ef_id0(&self) -> &EfId0 {
        &self.ef_id0
    }
    #[doc = "0xff4 - EFuse ID1 Register"]
    #[inline(always)]
    pub const fn ef_id1(&self) -> &EfId1 {
        &self.ef_id1
    }
    #[doc = "0xff8 - Processor ID Register"]
    #[inline(always)]
    pub const fn procid(&self) -> &Procid {
        &self.procid
    }
    #[doc = "0xffc - Peripheral ID Register"]
    #[inline(always)]
    pub const fn perid(&self) -> &Perid {
        &self.perid
    }
}
#[doc = "RST_STAT (rw) register accessor: System Reset Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_stat`] module"]
#[doc(alias = "RST_STAT")]
pub type RstStat = crate::Reg<rst_stat::RstStatSpec>;
#[doc = "System Reset Status"]
pub mod rst_stat;
pub use rst_stat as rst_cntl_rom;
pub use rst_stat as rst_cntl_ram0;
pub use rst_stat as rst_cntl_ram1;
pub use RstStat as RstCntlRom;
pub use RstStat as RstCntlRam0;
pub use RstStat as RstCntlRam1;
#[doc = "ROM_PROT (rw) register accessor: ROM Protection Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_prot`] module"]
#[doc(alias = "ROM_PROT")]
pub type RomProt = crate::Reg<rom_prot::RomProtSpec>;
#[doc = "ROM Protection Configuration"]
pub mod rom_prot;
#[doc = "ROM_SCRUB (rw) register accessor: ROM Scrub Period Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_scrub::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_scrub::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_scrub`] module"]
#[doc(alias = "ROM_SCRUB")]
pub type RomScrub = crate::Reg<rom_scrub::RomScrubSpec>;
#[doc = "ROM Scrub Period Configuration"]
pub mod rom_scrub;
pub use rom_scrub as ram0_scrub;
pub use rom_scrub as ram1_scrub;
pub use RomScrub as Ram0Scrub;
pub use RomScrub as Ram1Scrub;
#[doc = "IRQ_ENB (rw) register accessor: Enable EDAC Error Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_enb`] module"]
#[doc(alias = "IRQ_ENB")]
pub type IrqEnb = crate::Reg<irq_enb::IrqEnbSpec>;
#[doc = "Enable EDAC Error Interrupt Register"]
pub mod irq_enb;
pub use irq_enb as irq_raw;
pub use irq_enb as irq_end;
pub use irq_enb as irq_clr;
pub use IrqEnb as IrqRaw;
pub use IrqEnb as IrqEnd;
pub use IrqEnb as IrqClr;
#[doc = "RAM0_SBE (rw) register accessor: Count of RAM0 EDAC Single Bit Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`ram0_sbe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram0_sbe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram0_sbe`] module"]
#[doc(alias = "RAM0_SBE")]
pub type Ram0Sbe = crate::Reg<ram0_sbe::Ram0SbeSpec>;
#[doc = "Count of RAM0 EDAC Single Bit Errors"]
pub mod ram0_sbe;
pub use ram0_sbe as ram1_sbe;
pub use Ram0Sbe as Ram1Sbe;
#[doc = "RAM0_MBE (rw) register accessor: Count of RAM0 EDAC Multi Bit Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`ram0_mbe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram0_mbe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram0_mbe`] module"]
#[doc(alias = "RAM0_MBE")]
pub type Ram0Mbe = crate::Reg<ram0_mbe::Ram0MbeSpec>;
#[doc = "Count of RAM0 EDAC Multi Bit Errors"]
pub mod ram0_mbe;
pub use ram0_mbe as ram1_mbe;
pub use ram0_mbe as rom_mbe;
pub use ram0_sbe as rom_sbe;
pub use Ram0Mbe as Ram1Mbe;
pub use Ram0Mbe as RomMbe;
pub use Ram0Sbe as RomSbe;
#[doc = "ROM_RETRIES (r) register accessor: ROM BOOT Retry count\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_retries::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_retries`] module"]
#[doc(alias = "ROM_RETRIES")]
pub type RomRetries = crate::Reg<rom_retries::RomRetriesSpec>;
#[doc = "ROM BOOT Retry count"]
pub mod rom_retries;
#[doc = "REFRESH_CONFIG_H (rw) register accessor: Register Refresh Rate for TMR registers\n\nYou can [`read`](crate::Reg::read) this register and get [`refresh_config_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refresh_config_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refresh_config_h`] module"]
#[doc(alias = "REFRESH_CONFIG_H")]
pub type RefreshConfigH = crate::Reg<refresh_config_h::RefreshConfigHSpec>;
#[doc = "Register Refresh Rate for TMR registers"]
pub mod refresh_config_h;
#[doc = "TIM_RESET (rw) register accessor: TIM Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim_reset`] module"]
#[doc(alias = "TIM_RESET")]
pub type TimReset = crate::Reg<tim_reset::TimResetSpec>;
#[doc = "TIM Reset Control"]
pub mod tim_reset;
#[doc = "TIM_CLK_ENABLE (rw) register accessor: TIM Enable Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_clk_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_clk_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim_clk_enable`] module"]
#[doc(alias = "TIM_CLK_ENABLE")]
pub type TimClkEnable = crate::Reg<tim_clk_enable::TimClkEnableSpec>;
#[doc = "TIM Enable Control"]
pub mod tim_clk_enable;
#[doc = "PERIPHERAL_RESET (rw) register accessor: Peripheral Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheral_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheral_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peripheral_reset`] module"]
#[doc(alias = "PERIPHERAL_RESET")]
pub type PeripheralReset = crate::Reg<peripheral_reset::PeripheralResetSpec>;
#[doc = "Peripheral Reset Control"]
pub mod peripheral_reset;
pub use peripheral_reset as peripheral_clk_enable;
pub use PeripheralReset as PeripheralClkEnable;
#[doc = "SPW_M4_CTRL (rw) register accessor: SPW M4 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spw_m4_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spw_m4_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spw_m4_ctrl`] module"]
#[doc(alias = "SPW_M4_CTRL")]
pub type SpwM4Ctrl = crate::Reg<spw_m4_ctrl::SpwM4CtrlSpec>;
#[doc = "SPW M4 control register"]
pub mod spw_m4_ctrl;
#[doc = "PMU_CTRL (rw) register accessor: PMU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmu_ctrl`] module"]
#[doc(alias = "PMU_CTRL")]
pub type PmuCtrl = crate::Reg<pmu_ctrl::PmuCtrlSpec>;
#[doc = "PMU Control Register"]
pub mod pmu_ctrl;
#[doc = "WAKEUP_CNT (rw) register accessor: Wakeup Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_cnt`] module"]
#[doc(alias = "WAKEUP_CNT")]
pub type WakeupCnt = crate::Reg<wakeup_cnt::WakeupCntSpec>;
#[doc = "Wakeup Control"]
pub mod wakeup_cnt;
#[doc = "EBI_CFG0 (rw) register accessor: EBI Config Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ebi_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ebi_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ebi_cfg0`] module"]
#[doc(alias = "EBI_CFG0")]
pub type EbiCfg0 = crate::Reg<ebi_cfg0::EbiCfg0Spec>;
#[doc = "EBI Config Register 0"]
pub mod ebi_cfg0;
pub use ebi_cfg0 as ebi_cfg1;
pub use ebi_cfg0 as ebi_cfg2;
pub use ebi_cfg0 as ebi_cfg3;
pub use EbiCfg0 as EbiCfg1;
pub use EbiCfg0 as EbiCfg2;
pub use EbiCfg0 as EbiCfg3;
#[doc = "ANALOG_CNTL (rw) register accessor: Analog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`analog_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`analog_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_cntl`] module"]
#[doc(alias = "ANALOG_CNTL")]
pub type AnalogCntl = crate::Reg<analog_cntl::AnalogCntlSpec>;
#[doc = "Analog Control Register"]
pub mod analog_cntl;
#[doc = "SW_CLKDIV10 (rw) register accessor: Initial SpW Clock Divider Value\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_clkdiv10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_clkdiv10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_clkdiv10`] module"]
#[doc(alias = "SW_CLKDIV10")]
pub type SwClkdiv10 = crate::Reg<sw_clkdiv10::SwClkdiv10Spec>;
#[doc = "Initial SpW Clock Divider Value"]
pub mod sw_clkdiv10;
#[doc = "REFRESH_CONFIG_L (rw) register accessor: Register Refresh Rate for TMR registers\n\nYou can [`read`](crate::Reg::read) this register and get [`refresh_config_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refresh_config_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refresh_config_l`] module"]
#[doc(alias = "REFRESH_CONFIG_L")]
pub type RefreshConfigL = crate::Reg<refresh_config_l::RefreshConfigLSpec>;
#[doc = "Register Refresh Rate for TMR registers"]
pub mod refresh_config_l;
#[doc = "DAC0_CAL (r) register accessor: DAC0 Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac0_cal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0_cal`] module"]
#[doc(alias = "DAC0_CAL")]
pub type Dac0Cal = crate::Reg<dac0_cal::Dac0CalSpec>;
#[doc = "DAC0 Calibration Register"]
pub mod dac0_cal;
#[doc = "DAC1_CAL (r) register accessor: DAC1 Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac1_cal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1_cal`] module"]
#[doc(alias = "DAC1_CAL")]
pub type Dac1Cal = crate::Reg<dac1_cal::Dac1CalSpec>;
#[doc = "DAC1 Calibration Register"]
pub mod dac1_cal;
#[doc = "ADC_CAL (r) register accessor: ADC Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_cal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cal`] module"]
#[doc(alias = "ADC_CAL")]
pub type AdcCal = crate::Reg<adc_cal::AdcCalSpec>;
#[doc = "ADC Calibration Register"]
pub mod adc_cal;
#[doc = "BG_CAL (r) register accessor: Bandgap Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bg_cal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bg_cal`] module"]
#[doc(alias = "BG_CAL")]
pub type BgCal = crate::Reg<bg_cal::BgCalSpec>;
#[doc = "Bandgap Calibration Register"]
pub mod bg_cal;
#[doc = "DREG_CAL (r) register accessor: Digital LDO Regulator Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dreg_cal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dreg_cal`] module"]
#[doc(alias = "DREG_CAL")]
pub type DregCal = crate::Reg<dreg_cal::DregCalSpec>;
#[doc = "Digital LDO Regulator Calibration Register"]
pub mod dreg_cal;
#[doc = "AREG_CAL (r) register accessor: Analog LDO Regulator Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`areg_cal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@areg_cal`] module"]
#[doc(alias = "AREG_CAL")]
pub type AregCal = crate::Reg<areg_cal::AregCalSpec>;
#[doc = "Analog LDO Regulator Calibration Register"]
pub mod areg_cal;
#[doc = "HBO_CAL (r) register accessor: Heart Beat OSC Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hbo_cal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbo_cal`] module"]
#[doc(alias = "HBO_CAL")]
pub type HboCal = crate::Reg<hbo_cal::HboCalSpec>;
#[doc = "Heart Beat OSC Calibration Register"]
pub mod hbo_cal;
#[doc = "EF_CONFIG (r) register accessor: EFuse Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_config::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_config`] module"]
#[doc(alias = "EF_CONFIG")]
pub type EfConfig = crate::Reg<ef_config::EfConfigSpec>;
#[doc = "EFuse Config Register"]
pub mod ef_config;
#[doc = "EF_ID0 (r) register accessor: EFuse ID0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_id0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_id0`] module"]
#[doc(alias = "EF_ID0")]
pub type EfId0 = crate::Reg<ef_id0::EfId0Spec>;
#[doc = "EFuse ID0 Register"]
pub mod ef_id0;
#[doc = "EF_ID1 (r) register accessor: EFuse ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_id1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_id1`] module"]
#[doc(alias = "EF_ID1")]
pub type EfId1 = crate::Reg<ef_id1::EfId1Spec>;
#[doc = "EFuse ID1 Register"]
pub mod ef_id1;
#[doc = "PROCID (r) register accessor: Processor ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`procid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@procid`] module"]
#[doc(alias = "PROCID")]
pub type Procid = crate::Reg<procid::ProcidSpec>;
#[doc = "Processor ID Register"]
pub mod procid;
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`] module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
