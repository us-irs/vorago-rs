#[doc = "Register `SYND_ENC_64` reader"]
pub type R = crate::R<SyndEnc64Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Synd 64 bit Encoded Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_enc_64::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndEnc64Spec;
impl crate::RegisterSpec for SyndEnc64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_enc_64::R`](R) reader structure"]
impl crate::Readable for SyndEnc64Spec {}
#[doc = "`reset()` method sets SYND_ENC_64 to value 0"]
impl crate::Resettable for SyndEnc64Spec {}
