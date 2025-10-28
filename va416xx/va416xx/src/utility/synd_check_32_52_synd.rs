#[doc = "Register `SYND_CHECK_32_52_SYND` reader"]
pub type R = crate::R<SyndCheck32_52SyndSpec>;
#[doc = "Field `SYND_CHECK_32_52_SYND` reader - Corrected syndrome value"]
pub type SyndCheck32_52SyndR = crate::FieldReader<u32>;
#[doc = "Field `SBE` reader - Single bit error detect status"]
pub type SbeR = crate::FieldReader;
#[doc = "Field `MBE` reader - Multiple bit error detect status"]
pub type MbeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:19 - Corrected syndrome value"]
    #[inline(always)]
    pub fn synd_check_32_52_synd(&self) -> SyndCheck32_52SyndR {
        SyndCheck32_52SyndR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 24:27 - Single bit error detect status"]
    #[inline(always)]
    pub fn sbe(&self) -> SbeR {
        SbeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Multiple bit error detect status"]
    #[inline(always)]
    pub fn mbe(&self) -> MbeR {
        MbeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "EDAC Decode Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_32_52_synd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndCheck32_52SyndSpec;
impl crate::RegisterSpec for SyndCheck32_52SyndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_check_32_52_synd::R`](R) reader structure"]
impl crate::Readable for SyndCheck32_52SyndSpec {}
#[doc = "`reset()` method sets SYND_CHECK_32_52_SYND to value 0"]
impl crate::Resettable for SyndCheck32_52SyndSpec {}
