#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    synd_data: SyndData,
    synd_synd: SyndSynd,
    synd_enc_32_44: SyndEnc32_44,
    synd_check_32_44_data: SyndCheck32_44Data,
    synd_check_32_44_synd: SyndCheck32_44Synd,
    rom_trap_address: RomTrapAddress,
    rom_trap_synd: RomTrapSynd,
    ram_trap_addr0: RamTrapAddr0,
    ram_trap_synd0: RamTrapSynd0,
    ram_trap_addr1: RamTrapAddr1,
    ram_trap_synd1: RamTrapSynd1,
    _reserved11: [u8; 0xf4],
    synd_enc_32_52: SyndEnc32_52,
    synd_check_32_52_data: SyndCheck32_52Data,
    synd_check_32_52_synd: SyndCheck32_52Synd,
    _reserved14: [u8; 0x0ed0],
    perid: Perid,
}
impl RegisterBlock {
    #[doc = "0x00 - Data Register"]
    #[inline(always)]
    pub const fn synd_data(&self) -> &SyndData {
        &self.synd_data
    }
    #[doc = "0x04 - Syndrome Data Register"]
    #[inline(always)]
    pub const fn synd_synd(&self) -> &SyndSynd {
        &self.synd_synd
    }
    #[doc = "0x08 - EDAC Encode"]
    #[inline(always)]
    pub const fn synd_enc_32_44(&self) -> &SyndEnc32_44 {
        &self.synd_enc_32_44
    }
    #[doc = "0x0c - EDAC Decode Data"]
    #[inline(always)]
    pub const fn synd_check_32_44_data(&self) -> &SyndCheck32_44Data {
        &self.synd_check_32_44_data
    }
    #[doc = "0x10 - EDAC Decode Syndrome"]
    #[inline(always)]
    pub const fn synd_check_32_44_synd(&self) -> &SyndCheck32_44Synd {
        &self.synd_check_32_44_synd
    }
    #[doc = "0x14 - ROM EDAC Trap Address"]
    #[inline(always)]
    pub const fn rom_trap_address(&self) -> &RomTrapAddress {
        &self.rom_trap_address
    }
    #[doc = "0x18 - ROM EDAC Trap Syndrome"]
    #[inline(always)]
    pub const fn rom_trap_synd(&self) -> &RomTrapSynd {
        &self.rom_trap_synd
    }
    #[doc = "0x1c - RAM0 EDAC Trap Address"]
    #[inline(always)]
    pub const fn ram_trap_addr0(&self) -> &RamTrapAddr0 {
        &self.ram_trap_addr0
    }
    #[doc = "0x20 - RAM0 EDAC Trap Syndrome"]
    #[inline(always)]
    pub const fn ram_trap_synd0(&self) -> &RamTrapSynd0 {
        &self.ram_trap_synd0
    }
    #[doc = "0x24 - RAM1 EDAC Trap Address"]
    #[inline(always)]
    pub const fn ram_trap_addr1(&self) -> &RamTrapAddr1 {
        &self.ram_trap_addr1
    }
    #[doc = "0x28 - RAM1 EDAC Trap Syndrome"]
    #[inline(always)]
    pub const fn ram_trap_synd1(&self) -> &RamTrapSynd1 {
        &self.ram_trap_synd1
    }
    #[doc = "0x120 - EDAC Encode"]
    #[inline(always)]
    pub const fn synd_enc_32_52(&self) -> &SyndEnc32_52 {
        &self.synd_enc_32_52
    }
    #[doc = "0x124 - EDAC Decode Data"]
    #[inline(always)]
    pub const fn synd_check_32_52_data(&self) -> &SyndCheck32_52Data {
        &self.synd_check_32_52_data
    }
    #[doc = "0x128 - EDAC Decode Syndrome"]
    #[inline(always)]
    pub const fn synd_check_32_52_synd(&self) -> &SyndCheck32_52Synd {
        &self.synd_check_32_52_synd
    }
    #[doc = "0xffc - Peripheral ID Register"]
    #[inline(always)]
    pub const fn perid(&self) -> &Perid {
        &self.perid
    }
}
#[doc = "SYND_DATA (rw) register accessor: Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synd_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_data`] module"]
#[doc(alias = "SYND_DATA")]
pub type SyndData = crate::Reg<synd_data::SyndDataSpec>;
#[doc = "Data Register"]
pub mod synd_data;
#[doc = "SYND_SYND (rw) register accessor: Syndrome Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_synd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synd_synd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_synd`] module"]
#[doc(alias = "SYND_SYND")]
pub type SyndSynd = crate::Reg<synd_synd::SyndSyndSpec>;
#[doc = "Syndrome Data Register"]
pub mod synd_synd;
#[doc = "SYND_ENC_32_44 (rw) register accessor: EDAC Encode\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_enc_32_44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synd_enc_32_44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_enc_32_44`] module"]
#[doc(alias = "SYND_ENC_32_44")]
pub type SyndEnc32_44 = crate::Reg<synd_enc_32_44::SyndEnc32_44Spec>;
#[doc = "EDAC Encode"]
pub mod synd_enc_32_44;
#[doc = "SYND_CHECK_32_44_DATA (r) register accessor: EDAC Decode Data\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_32_44_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_check_32_44_data`] module"]
#[doc(alias = "SYND_CHECK_32_44_DATA")]
pub type SyndCheck32_44Data = crate::Reg<synd_check_32_44_data::SyndCheck32_44DataSpec>;
#[doc = "EDAC Decode Data"]
pub mod synd_check_32_44_data;
#[doc = "SYND_CHECK_32_44_SYND (r) register accessor: EDAC Decode Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_32_44_synd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_check_32_44_synd`] module"]
#[doc(alias = "SYND_CHECK_32_44_SYND")]
pub type SyndCheck32_44Synd = crate::Reg<synd_check_32_44_synd::SyndCheck32_44SyndSpec>;
#[doc = "EDAC Decode Syndrome"]
pub mod synd_check_32_44_synd;
#[doc = "ROM_TRAP_ADDRESS (rw) register accessor: ROM EDAC Trap Address\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_trap_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_trap_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_trap_address`] module"]
#[doc(alias = "ROM_TRAP_ADDRESS")]
pub type RomTrapAddress = crate::Reg<rom_trap_address::RomTrapAddressSpec>;
#[doc = "ROM EDAC Trap Address"]
pub mod rom_trap_address;
#[doc = "ROM_TRAP_SYND (rw) register accessor: ROM EDAC Trap Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_trap_synd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_trap_synd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_trap_synd`] module"]
#[doc(alias = "ROM_TRAP_SYND")]
pub type RomTrapSynd = crate::Reg<rom_trap_synd::RomTrapSyndSpec>;
#[doc = "ROM EDAC Trap Syndrome"]
pub mod rom_trap_synd;
#[doc = "RAM_TRAP_ADDR0 (rw) register accessor: RAM0 EDAC Trap Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_trap_addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_trap_addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_trap_addr0`] module"]
#[doc(alias = "RAM_TRAP_ADDR0")]
pub type RamTrapAddr0 = crate::Reg<ram_trap_addr0::RamTrapAddr0Spec>;
#[doc = "RAM0 EDAC Trap Address"]
pub mod ram_trap_addr0;
#[doc = "RAM_TRAP_SYND0 (rw) register accessor: RAM0 EDAC Trap Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_trap_synd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_trap_synd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_trap_synd0`] module"]
#[doc(alias = "RAM_TRAP_SYND0")]
pub type RamTrapSynd0 = crate::Reg<ram_trap_synd0::RamTrapSynd0Spec>;
#[doc = "RAM0 EDAC Trap Syndrome"]
pub mod ram_trap_synd0;
#[doc = "RAM_TRAP_ADDR1 (rw) register accessor: RAM1 EDAC Trap Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_trap_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_trap_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_trap_addr1`] module"]
#[doc(alias = "RAM_TRAP_ADDR1")]
pub type RamTrapAddr1 = crate::Reg<ram_trap_addr1::RamTrapAddr1Spec>;
#[doc = "RAM1 EDAC Trap Address"]
pub mod ram_trap_addr1;
#[doc = "RAM_TRAP_SYND1 (rw) register accessor: RAM1 EDAC Trap Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_trap_synd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_trap_synd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_trap_synd1`] module"]
#[doc(alias = "RAM_TRAP_SYND1")]
pub type RamTrapSynd1 = crate::Reg<ram_trap_synd1::RamTrapSynd1Spec>;
#[doc = "RAM1 EDAC Trap Syndrome"]
pub mod ram_trap_synd1;
#[doc = "SYND_ENC_32_52 (r) register accessor: EDAC Encode\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_enc_32_52::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_enc_32_52`] module"]
#[doc(alias = "SYND_ENC_32_52")]
pub type SyndEnc32_52 = crate::Reg<synd_enc_32_52::SyndEnc32_52Spec>;
#[doc = "EDAC Encode"]
pub mod synd_enc_32_52;
#[doc = "SYND_CHECK_32_52_DATA (r) register accessor: EDAC Decode Data\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_32_52_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_check_32_52_data`] module"]
#[doc(alias = "SYND_CHECK_32_52_DATA")]
pub type SyndCheck32_52Data = crate::Reg<synd_check_32_52_data::SyndCheck32_52DataSpec>;
#[doc = "EDAC Decode Data"]
pub mod synd_check_32_52_data;
#[doc = "SYND_CHECK_32_52_SYND (r) register accessor: EDAC Decode Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_32_52_synd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_check_32_52_synd`] module"]
#[doc(alias = "SYND_CHECK_32_52_SYND")]
pub type SyndCheck32_52Synd = crate::Reg<synd_check_32_52_synd::SyndCheck32_52SyndSpec>;
#[doc = "EDAC Decode Syndrome"]
pub mod synd_check_32_52_synd;
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`] module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
