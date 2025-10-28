#[doc = "Register `SYND_ENC_32_52` reader"]
pub type R = crate::R<SyndEnc32_52Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Synd 32/52 bit Encoded Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_enc_32_52::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndEnc32_52Spec;
impl crate::RegisterSpec for SyndEnc32_52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_enc_32_52::R`](R) reader structure"]
impl crate::Readable for SyndEnc32_52Spec {}
#[doc = "`reset()` method sets SYND_ENC_32_52 to value 0"]
impl crate::Resettable for SyndEnc32_52Spec {}
