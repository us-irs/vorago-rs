#[doc = "Register `SYND_CHECK_32_44_SYND` reader"]
pub type R = crate::R<SyndCheck32_44SyndSpec>;
#[doc = "Field `SYND_CHECK_32_44_SYND` reader - Correct syndrome value"]
pub type SyndCheck32_44SyndR = crate::FieldReader<u16>;
#[doc = "Field `SBE` reader - Single bit error detect status"]
pub type SbeR = crate::FieldReader;
#[doc = "Field `MBE` reader - Multiple bit error detect status"]
pub type MbeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - Correct syndrome value"]
    #[inline(always)]
    pub fn synd_check_32_44_synd(&self) -> SyndCheck32_44SyndR {
        SyndCheck32_44SyndR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - Single bit error detect status"]
    #[inline(always)]
    pub fn sbe(&self) -> SbeR {
        SbeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Multiple bit error detect status"]
    #[inline(always)]
    pub fn mbe(&self) -> MbeR {
        MbeR::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "EDAC Decode Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_32_44_synd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndCheck32_44SyndSpec;
impl crate::RegisterSpec for SyndCheck32_44SyndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_check_32_44_synd::R`](R) reader structure"]
impl crate::Readable for SyndCheck32_44SyndSpec {}
#[doc = "`reset()` method sets SYND_CHECK_32_44_SYND to value 0"]
impl crate::Resettable for SyndCheck32_44SyndSpec {}
