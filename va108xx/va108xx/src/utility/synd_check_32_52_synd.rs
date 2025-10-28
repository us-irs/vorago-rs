#[doc = "Register `SYND_CHECK_32_52_SYND` reader"]
pub type R = crate::R<SyndCheck32_52SyndSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Synd 32/52 bit Corrected Syndrome and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_32_52_synd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndCheck32_52SyndSpec;
impl crate::RegisterSpec for SyndCheck32_52SyndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_check_32_52_synd::R`](R) reader structure"]
impl crate::Readable for SyndCheck32_52SyndSpec {}
#[doc = "`reset()` method sets SYND_CHECK_32_52_SYND to value 0"]
impl crate::Resettable for SyndCheck32_52SyndSpec {}
