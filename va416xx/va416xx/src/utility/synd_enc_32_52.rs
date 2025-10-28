#[doc = "Register `SYND_ENC_32_52` reader"]
pub type R = crate::R<SyndEnc32_52Spec>;
#[doc = "Field `SYND_ENC_32_52` reader - Computed syndrome value for bits 15-0"]
pub type SyndEnc32_52R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Computed syndrome value for bits 15-0"]
    #[inline(always)]
    pub fn synd_enc_32_52(&self) -> SyndEnc32_52R {
        SyndEnc32_52R::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "EDAC Encode\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_enc_32_52::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndEnc32_52Spec;
impl crate::RegisterSpec for SyndEnc32_52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_enc_32_52::R`](R) reader structure"]
impl crate::Readable for SyndEnc32_52Spec {}
#[doc = "`reset()` method sets SYND_ENC_32_52 to value 0"]
impl crate::Resettable for SyndEnc32_52Spec {}
