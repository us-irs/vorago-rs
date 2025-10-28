#[doc = "Register `SYND_CHECK_64_DATA1` reader"]
pub type R = crate::R<SyndCheck64Data1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Synd 64 bit Corrected Data 1\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_64_data1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndCheck64Data1Spec;
impl crate::RegisterSpec for SyndCheck64Data1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_check_64_data1::R`](R) reader structure"]
impl crate::Readable for SyndCheck64Data1Spec {}
#[doc = "`reset()` method sets SYND_CHECK_64_DATA1 to value 0"]
impl crate::Resettable for SyndCheck64Data1Spec {}
