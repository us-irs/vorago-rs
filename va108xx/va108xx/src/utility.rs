#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    synd_data0: SyndData0,
    synd_data1: SyndData1,
    synd_synd: SyndSynd,
    synd_enc_32: SyndEnc32,
    synd_check_32_data: SyndCheck32Data,
    synd_check_32_synd: SyndCheck32Synd,
    synd_enc_64: SyndEnc64,
    synd_check_64_data0: SyndCheck64Data0,
    synd_check_64_data1: SyndCheck64Data1,
    synd_check_64_synd: SyndCheck64Synd,
    synd_enc_32_52: SyndEnc32_52,
    synd_check_32_52_data: SyndCheck32_52Data,
    synd_check_32_52_synd: SyndCheck32_52Synd,
    _reserved13: [u8; 0x0fc8],
    perid: Perid,
}
impl RegisterBlock {
    #[doc = "0x00 - Synd Data 0 Register"]
    #[inline(always)]
    pub const fn synd_data0(&self) -> &SyndData0 {
        &self.synd_data0
    }
    #[doc = "0x04 - Synd Data 1 Register"]
    #[inline(always)]
    pub const fn synd_data1(&self) -> &SyndData1 {
        &self.synd_data1
    }
    #[doc = "0x08 - Synd Parity Register"]
    #[inline(always)]
    pub const fn synd_synd(&self) -> &SyndSynd {
        &self.synd_synd
    }
    #[doc = "0x0c - Synd 32 bit Encoded Syndrome"]
    #[inline(always)]
    pub const fn synd_enc_32(&self) -> &SyndEnc32 {
        &self.synd_enc_32
    }
    #[doc = "0x10 - Synd 32 bit Corrected Data"]
    #[inline(always)]
    pub const fn synd_check_32_data(&self) -> &SyndCheck32Data {
        &self.synd_check_32_data
    }
    #[doc = "0x14 - Synd 32 bit Corrected Syndrome and Status"]
    #[inline(always)]
    pub const fn synd_check_32_synd(&self) -> &SyndCheck32Synd {
        &self.synd_check_32_synd
    }
    #[doc = "0x18 - Synd 64 bit Encoded Syndrome"]
    #[inline(always)]
    pub const fn synd_enc_64(&self) -> &SyndEnc64 {
        &self.synd_enc_64
    }
    #[doc = "0x1c - Synd 64 bit Corrected Data 0"]
    #[inline(always)]
    pub const fn synd_check_64_data0(&self) -> &SyndCheck64Data0 {
        &self.synd_check_64_data0
    }
    #[doc = "0x20 - Synd 64 bit Corrected Data 1"]
    #[inline(always)]
    pub const fn synd_check_64_data1(&self) -> &SyndCheck64Data1 {
        &self.synd_check_64_data1
    }
    #[doc = "0x24 - Synd 64 bit Corrected Parity and Status"]
    #[inline(always)]
    pub const fn synd_check_64_synd(&self) -> &SyndCheck64Synd {
        &self.synd_check_64_synd
    }
    #[doc = "0x28 - Synd 32/52 bit Encoded Syndrome"]
    #[inline(always)]
    pub const fn synd_enc_32_52(&self) -> &SyndEnc32_52 {
        &self.synd_enc_32_52
    }
    #[doc = "0x2c - Synd 32/52 bit Corrected Data"]
    #[inline(always)]
    pub const fn synd_check_32_52_data(&self) -> &SyndCheck32_52Data {
        &self.synd_check_32_52_data
    }
    #[doc = "0x30 - Synd 32/52 bit Corrected Syndrome and Status"]
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
#[doc = "SYND_DATA0 (rw) register accessor: Synd Data 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synd_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_data0`] module"]
#[doc(alias = "SYND_DATA0")]
pub type SyndData0 = crate::Reg<synd_data0::SyndData0Spec>;
#[doc = "Synd Data 0 Register"]
pub mod synd_data0;
#[doc = "SYND_DATA1 (rw) register accessor: Synd Data 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synd_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_data1`] module"]
#[doc(alias = "SYND_DATA1")]
pub type SyndData1 = crate::Reg<synd_data1::SyndData1Spec>;
#[doc = "Synd Data 1 Register"]
pub mod synd_data1;
#[doc = "SYND_SYND (rw) register accessor: Synd Parity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_synd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synd_synd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_synd`] module"]
#[doc(alias = "SYND_SYND")]
pub type SyndSynd = crate::Reg<synd_synd::SyndSyndSpec>;
#[doc = "Synd Parity Register"]
pub mod synd_synd;
#[doc = "SYND_ENC_32 (r) register accessor: Synd 32 bit Encoded Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_enc_32::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_enc_32`] module"]
#[doc(alias = "SYND_ENC_32")]
pub type SyndEnc32 = crate::Reg<synd_enc_32::SyndEnc32Spec>;
#[doc = "Synd 32 bit Encoded Syndrome"]
pub mod synd_enc_32;
#[doc = "SYND_CHECK_32_DATA (r) register accessor: Synd 32 bit Corrected Data\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_32_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_check_32_data`] module"]
#[doc(alias = "SYND_CHECK_32_DATA")]
pub type SyndCheck32Data = crate::Reg<synd_check_32_data::SyndCheck32DataSpec>;
#[doc = "Synd 32 bit Corrected Data"]
pub mod synd_check_32_data;
#[doc = "SYND_CHECK_32_SYND (r) register accessor: Synd 32 bit Corrected Syndrome and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_32_synd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_check_32_synd`] module"]
#[doc(alias = "SYND_CHECK_32_SYND")]
pub type SyndCheck32Synd = crate::Reg<synd_check_32_synd::SyndCheck32SyndSpec>;
#[doc = "Synd 32 bit Corrected Syndrome and Status"]
pub mod synd_check_32_synd;
#[doc = "SYND_ENC_64 (r) register accessor: Synd 64 bit Encoded Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_enc_64::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_enc_64`] module"]
#[doc(alias = "SYND_ENC_64")]
pub type SyndEnc64 = crate::Reg<synd_enc_64::SyndEnc64Spec>;
#[doc = "Synd 64 bit Encoded Syndrome"]
pub mod synd_enc_64;
#[doc = "SYND_CHECK_64_DATA0 (r) register accessor: Synd 64 bit Corrected Data 0\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_64_data0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_check_64_data0`] module"]
#[doc(alias = "SYND_CHECK_64_DATA0")]
pub type SyndCheck64Data0 = crate::Reg<synd_check_64_data0::SyndCheck64Data0Spec>;
#[doc = "Synd 64 bit Corrected Data 0"]
pub mod synd_check_64_data0;
#[doc = "SYND_CHECK_64_DATA1 (r) register accessor: Synd 64 bit Corrected Data 1\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_64_data1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_check_64_data1`] module"]
#[doc(alias = "SYND_CHECK_64_DATA1")]
pub type SyndCheck64Data1 = crate::Reg<synd_check_64_data1::SyndCheck64Data1Spec>;
#[doc = "Synd 64 bit Corrected Data 1"]
pub mod synd_check_64_data1;
#[doc = "SYND_CHECK_64_SYND (r) register accessor: Synd 64 bit Corrected Parity and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_64_synd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_check_64_synd`] module"]
#[doc(alias = "SYND_CHECK_64_SYND")]
pub type SyndCheck64Synd = crate::Reg<synd_check_64_synd::SyndCheck64SyndSpec>;
#[doc = "Synd 64 bit Corrected Parity and Status"]
pub mod synd_check_64_synd;
#[doc = "SYND_ENC_32_52 (r) register accessor: Synd 32/52 bit Encoded Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_enc_32_52::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_enc_32_52`] module"]
#[doc(alias = "SYND_ENC_32_52")]
pub type SyndEnc32_52 = crate::Reg<synd_enc_32_52::SyndEnc32_52Spec>;
#[doc = "Synd 32/52 bit Encoded Syndrome"]
pub mod synd_enc_32_52;
#[doc = "SYND_CHECK_32_52_DATA (r) register accessor: Synd 32/52 bit Corrected Data\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_32_52_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_check_32_52_data`] module"]
#[doc(alias = "SYND_CHECK_32_52_DATA")]
pub type SyndCheck32_52Data = crate::Reg<synd_check_32_52_data::SyndCheck32_52DataSpec>;
#[doc = "Synd 32/52 bit Corrected Data"]
pub mod synd_check_32_52_data;
#[doc = "SYND_CHECK_32_52_SYND (r) register accessor: Synd 32/52 bit Corrected Syndrome and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_32_52_synd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synd_check_32_52_synd`] module"]
#[doc(alias = "SYND_CHECK_32_52_SYND")]
pub type SyndCheck32_52Synd = crate::Reg<synd_check_32_52_synd::SyndCheck32_52SyndSpec>;
#[doc = "Synd 32/52 bit Corrected Syndrome and Status"]
pub mod synd_check_32_52_synd;
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`] module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
