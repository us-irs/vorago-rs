#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rst_stat: RstStat,
    rst_cntl_rom: RstCntlRom,
    rst_cntl_ram: RstCntlRam,
    rom_prot: RomProt,
    rom_scrub: RomScrub,
    ram_scrub: RamScrub,
    rom_trap_addr: RomTrapAddr,
    rom_trap_synd: RomTrapSynd,
    ram_trap_addr: RamTrapAddr,
    ram_trap_synd: RamTrapSynd,
    irq_enb: IrqEnb,
    irq_raw: IrqRaw,
    irq_end: IrqEnd,
    irq_clr: IrqClr,
    ram_sbe: RamSbe,
    ram_mbe: RamMbe,
    rom_sbe: RomSbe,
    rom_mbe: RomMbe,
    ioconfig_clkdiv0: IoconfigClkdiv0,
    ioconfig_clkdiv: [IoconfigClkdiv; 7],
    rom_retries: RomRetries,
    refresh_config: RefreshConfig,
    tim_reset: TimReset,
    tim_clk_enable: TimClkEnable,
    peripheral_reset: PeripheralReset,
    peripheral_clk_enable: PeripheralClkEnable,
    lockup_reset: LockupReset,
    _reserved27: [u8; 0x0f6c],
    ef_config: EfConfig,
    ef_id: EfId,
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
    pub const fn rst_cntl_ram(&self) -> &RstCntlRam {
        &self.rst_cntl_ram
    }
    #[doc = "0x0c - ROM Protection Configuration"]
    #[inline(always)]
    pub const fn rom_prot(&self) -> &RomProt {
        &self.rom_prot
    }
    #[doc = "0x10 - ROM Scrub Period Configuration"]
    #[inline(always)]
    pub const fn rom_scrub(&self) -> &RomScrub {
        &self.rom_scrub
    }
    #[doc = "0x14 - RAM Scrub Period Configuration"]
    #[inline(always)]
    pub const fn ram_scrub(&self) -> &RamScrub {
        &self.ram_scrub
    }
    #[doc = "0x18 - ROM Trap Address"]
    #[inline(always)]
    pub const fn rom_trap_addr(&self) -> &RomTrapAddr {
        &self.rom_trap_addr
    }
    #[doc = "0x1c - ROM Trap Syndrome"]
    #[inline(always)]
    pub const fn rom_trap_synd(&self) -> &RomTrapSynd {
        &self.rom_trap_synd
    }
    #[doc = "0x20 - RAM Trap Address"]
    #[inline(always)]
    pub const fn ram_trap_addr(&self) -> &RamTrapAddr {
        &self.ram_trap_addr
    }
    #[doc = "0x24 - RAM Trap Syndrome"]
    #[inline(always)]
    pub const fn ram_trap_synd(&self) -> &RamTrapSynd {
        &self.ram_trap_synd
    }
    #[doc = "0x28 - Enable EDAC Error Interrupt Register"]
    #[inline(always)]
    pub const fn irq_enb(&self) -> &IrqEnb {
        &self.irq_enb
    }
    #[doc = "0x2c - Raw EDAC Error Interrupt Status"]
    #[inline(always)]
    pub const fn irq_raw(&self) -> &IrqRaw {
        &self.irq_raw
    }
    #[doc = "0x30 - Enabled EDAC Error Interrupt Status"]
    #[inline(always)]
    pub const fn irq_end(&self) -> &IrqEnd {
        &self.irq_end
    }
    #[doc = "0x34 - Clear EDAC Error Interrupt Status"]
    #[inline(always)]
    pub const fn irq_clr(&self) -> &IrqClr {
        &self.irq_clr
    }
    #[doc = "0x38 - Count of RAM EDAC Single Bit Errors"]
    #[inline(always)]
    pub const fn ram_sbe(&self) -> &RamSbe {
        &self.ram_sbe
    }
    #[doc = "0x3c - Count of RAM EDAC Multi Bit Errors"]
    #[inline(always)]
    pub const fn ram_mbe(&self) -> &RamMbe {
        &self.ram_mbe
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
    #[doc = "0x48 - IO Configuration Clock Divider Register"]
    #[inline(always)]
    pub const fn ioconfig_clkdiv0(&self) -> &IoconfigClkdiv0 {
        &self.ioconfig_clkdiv0
    }
    #[doc = "0x4c..0x68 - IO Configuration Clock Divider Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `IOCONFIG_CLKDIV1` register.</div>"]
    #[inline(always)]
    pub const fn ioconfig_clkdiv(&self, n: usize) -> &IoconfigClkdiv {
        &self.ioconfig_clkdiv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4c..0x68 - IO Configuration Clock Divider Register"]
    #[inline(always)]
    pub fn ioconfig_clkdiv_iter(&self) -> impl Iterator<Item = &IoconfigClkdiv> {
        self.ioconfig_clkdiv.iter()
    }
    #[doc = "0x4c - IO Configuration Clock Divider Register"]
    #[inline(always)]
    pub const fn ioconfig_clkdiv1(&self) -> &IoconfigClkdiv {
        self.ioconfig_clkdiv(0)
    }
    #[doc = "0x50 - IO Configuration Clock Divider Register"]
    #[inline(always)]
    pub const fn ioconfig_clkdiv2(&self) -> &IoconfigClkdiv {
        self.ioconfig_clkdiv(1)
    }
    #[doc = "0x54 - IO Configuration Clock Divider Register"]
    #[inline(always)]
    pub const fn ioconfig_clkdiv3(&self) -> &IoconfigClkdiv {
        self.ioconfig_clkdiv(2)
    }
    #[doc = "0x58 - IO Configuration Clock Divider Register"]
    #[inline(always)]
    pub const fn ioconfig_clkdiv4(&self) -> &IoconfigClkdiv {
        self.ioconfig_clkdiv(3)
    }
    #[doc = "0x5c - IO Configuration Clock Divider Register"]
    #[inline(always)]
    pub const fn ioconfig_clkdiv5(&self) -> &IoconfigClkdiv {
        self.ioconfig_clkdiv(4)
    }
    #[doc = "0x60 - IO Configuration Clock Divider Register"]
    #[inline(always)]
    pub const fn ioconfig_clkdiv6(&self) -> &IoconfigClkdiv {
        self.ioconfig_clkdiv(5)
    }
    #[doc = "0x64 - IO Configuration Clock Divider Register"]
    #[inline(always)]
    pub const fn ioconfig_clkdiv7(&self) -> &IoconfigClkdiv {
        self.ioconfig_clkdiv(6)
    }
    #[doc = "0x68 - ROM BOOT Retry count"]
    #[inline(always)]
    pub const fn rom_retries(&self) -> &RomRetries {
        &self.rom_retries
    }
    #[doc = "0x6c - Register Refresh Control"]
    #[inline(always)]
    pub const fn refresh_config(&self) -> &RefreshConfig {
        &self.refresh_config
    }
    #[doc = "0x70 - TIM Reset Control"]
    #[inline(always)]
    pub const fn tim_reset(&self) -> &TimReset {
        &self.tim_reset
    }
    #[doc = "0x74 - TIM Enable Control"]
    #[inline(always)]
    pub const fn tim_clk_enable(&self) -> &TimClkEnable {
        &self.tim_clk_enable
    }
    #[doc = "0x78 - Peripheral Reset Control"]
    #[inline(always)]
    pub const fn peripheral_reset(&self) -> &PeripheralReset {
        &self.peripheral_reset
    }
    #[doc = "0x7c - Peripheral Enable Control"]
    #[inline(always)]
    pub const fn peripheral_clk_enable(&self) -> &PeripheralClkEnable {
        &self.peripheral_clk_enable
    }
    #[doc = "0x80 - Lockup Reset Configuration"]
    #[inline(always)]
    pub const fn lockup_reset(&self) -> &LockupReset {
        &self.lockup_reset
    }
    #[doc = "0xff0 - EFuse Config Register"]
    #[inline(always)]
    pub const fn ef_config(&self) -> &EfConfig {
        &self.ef_config
    }
    #[doc = "0xff4 - EFuse ID Register"]
    #[inline(always)]
    pub const fn ef_id(&self) -> &EfId {
        &self.ef_id
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
pub use rst_stat as rst_cntl_ram;
pub use RstStat as RstCntlRom;
pub use RstStat as RstCntlRam;
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
pub use rom_scrub as ram_scrub;
pub use RomScrub as RamScrub;
#[doc = "ROM_TRAP_ADDR (rw) register accessor: ROM Trap Address\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_trap_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_trap_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_trap_addr`] module"]
#[doc(alias = "ROM_TRAP_ADDR")]
pub type RomTrapAddr = crate::Reg<rom_trap_addr::RomTrapAddrSpec>;
#[doc = "ROM Trap Address"]
pub mod rom_trap_addr;
#[doc = "ROM_TRAP_SYND (rw) register accessor: ROM Trap Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_trap_synd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_trap_synd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_trap_synd`] module"]
#[doc(alias = "ROM_TRAP_SYND")]
pub type RomTrapSynd = crate::Reg<rom_trap_synd::RomTrapSyndSpec>;
#[doc = "ROM Trap Syndrome"]
pub mod rom_trap_synd;
pub use rom_trap_addr as ram_trap_addr;
pub use rom_trap_synd as ram_trap_synd;
pub use RomTrapAddr as RamTrapAddr;
pub use RomTrapSynd as RamTrapSynd;
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
#[doc = "RAM_SBE (rw) register accessor: Count of RAM EDAC Single Bit Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_sbe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_sbe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_sbe`] module"]
#[doc(alias = "RAM_SBE")]
pub type RamSbe = crate::Reg<ram_sbe::RamSbeSpec>;
#[doc = "Count of RAM EDAC Single Bit Errors"]
pub mod ram_sbe;
pub use ram_sbe as ram_mbe;
pub use ram_sbe as rom_sbe;
pub use ram_sbe as rom_mbe;
pub use RamSbe as RamMbe;
pub use RamSbe as RomSbe;
pub use RamSbe as RomMbe;
#[doc = "IOCONFIG_CLKDIV0 (r) register accessor: IO Configuration Clock Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ioconfig_clkdiv0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioconfig_clkdiv0`] module"]
#[doc(alias = "IOCONFIG_CLKDIV0")]
pub type IoconfigClkdiv0 = crate::Reg<ioconfig_clkdiv0::IoconfigClkdiv0Spec>;
#[doc = "IO Configuration Clock Divider Register"]
pub mod ioconfig_clkdiv0;
#[doc = "IOCONFIG_CLKDIV (rw) register accessor: IO Configuration Clock Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ioconfig_clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioconfig_clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioconfig_clkdiv`] module"]
#[doc(alias = "IOCONFIG_CLKDIV")]
pub type IoconfigClkdiv = crate::Reg<ioconfig_clkdiv::IoconfigClkdivSpec>;
#[doc = "IO Configuration Clock Divider Register"]
pub mod ioconfig_clkdiv;
#[doc = "ROM_RETRIES (r) register accessor: ROM BOOT Retry count\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_retries::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_retries`] module"]
#[doc(alias = "ROM_RETRIES")]
pub type RomRetries = crate::Reg<rom_retries::RomRetriesSpec>;
#[doc = "ROM BOOT Retry count"]
pub mod rom_retries;
#[doc = "REFRESH_CONFIG (rw) register accessor: Register Refresh Control\n\nYou can [`read`](crate::Reg::read) this register and get [`refresh_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refresh_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refresh_config`] module"]
#[doc(alias = "REFRESH_CONFIG")]
pub type RefreshConfig = crate::Reg<refresh_config::RefreshConfigSpec>;
#[doc = "Register Refresh Control"]
pub mod refresh_config;
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
#[doc = "PERIPHERAL_CLK_ENABLE (rw) register accessor: Peripheral Enable Control\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheral_clk_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheral_clk_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peripheral_clk_enable`] module"]
#[doc(alias = "PERIPHERAL_CLK_ENABLE")]
pub type PeripheralClkEnable = crate::Reg<peripheral_clk_enable::PeripheralClkEnableSpec>;
#[doc = "Peripheral Enable Control"]
pub mod peripheral_clk_enable;
#[doc = "LOCKUP_RESET (rw) register accessor: Lockup Reset Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`lockup_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockup_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockup_reset`] module"]
#[doc(alias = "LOCKUP_RESET")]
pub type LockupReset = crate::Reg<lockup_reset::LockupResetSpec>;
#[doc = "Lockup Reset Configuration"]
pub mod lockup_reset;
#[doc = "EF_CONFIG (r) register accessor: EFuse Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_config::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_config`] module"]
#[doc(alias = "EF_CONFIG")]
pub type EfConfig = crate::Reg<ef_config::EfConfigSpec>;
#[doc = "EFuse Config Register"]
pub mod ef_config;
#[doc = "EF_ID (r) register accessor: EFuse ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_id`] module"]
#[doc(alias = "EF_ID")]
pub type EfId = crate::Reg<ef_id::EfIdSpec>;
#[doc = "EFuse ID Register"]
pub mod ef_id;
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
