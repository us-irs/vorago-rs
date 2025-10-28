#[doc = "Register `SYND_CHECK_32_SYND` reader"]
pub type R = crate::R<SyndCheck32SyndSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Synd 32 bit Corrected Syndrome and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_32_synd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndCheck32SyndSpec;
impl crate::RegisterSpec for SyndCheck32SyndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_check_32_synd::R`](R) reader structure"]
impl crate::Readable for SyndCheck32SyndSpec {}
#[doc = "`reset()` method sets SYND_CHECK_32_SYND to value 0"]
impl crate::Resettable for SyndCheck32SyndSpec {}
