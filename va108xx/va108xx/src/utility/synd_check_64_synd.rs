#[doc = "Register `SYND_CHECK_64_SYND` reader"]
pub type R = crate::R<SyndCheck64SyndSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Synd 64 bit Corrected Parity and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_64_synd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndCheck64SyndSpec;
impl crate::RegisterSpec for SyndCheck64SyndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_check_64_synd::R`](R) reader structure"]
impl crate::Readable for SyndCheck64SyndSpec {}
#[doc = "`reset()` method sets SYND_CHECK_64_SYND to value 0"]
impl crate::Resettable for SyndCheck64SyndSpec {}
