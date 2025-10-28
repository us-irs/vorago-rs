#[doc = "Register `SYND_ENC_32` reader"]
pub type R = crate::R<SyndEnc32Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Synd 32 bit Encoded Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_enc_32::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndEnc32Spec;
impl crate::RegisterSpec for SyndEnc32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_enc_32::R`](R) reader structure"]
impl crate::Readable for SyndEnc32Spec {}
#[doc = "`reset()` method sets SYND_ENC_32 to value 0"]
impl crate::Resettable for SyndEnc32Spec {}
