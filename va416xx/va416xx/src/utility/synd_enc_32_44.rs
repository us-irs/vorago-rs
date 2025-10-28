#[doc = "Register `SYND_ENC_32_44` reader"]
pub type R = crate::R<SyndEnc32_44Spec>;
#[doc = "Register `SYND_ENC_32_44` writer"]
pub type W = crate::W<SyndEnc32_44Spec>;
#[doc = "Field `SYND_ENC_7_0` reader - Computed syndrome value for bits 15-0"]
pub type SyndEnc7_0R = crate::FieldReader;
#[doc = "Field `SYND_ENC_7_0` writer - Computed syndrome value for bits 15-0"]
pub type SyndEnc7_0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SYND_ENC_31_16` reader - Computed syndrome value for bits 31-16"]
pub type SyndEnc31_16R = crate::FieldReader;
#[doc = "Field `SYND_ENC_31_16` writer - Computed syndrome value for bits 31-16"]
pub type SyndEnc31_16W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Computed syndrome value for bits 15-0"]
    #[inline(always)]
    pub fn synd_enc_7_0(&self) -> SyndEnc7_0R {
        SyndEnc7_0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Computed syndrome value for bits 31-16"]
    #[inline(always)]
    pub fn synd_enc_31_16(&self) -> SyndEnc31_16R {
        SyndEnc31_16R::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Computed syndrome value for bits 15-0"]
    #[inline(always)]
    pub fn synd_enc_7_0(&mut self) -> SyndEnc7_0W<'_, SyndEnc32_44Spec> {
        SyndEnc7_0W::new(self, 0)
    }
    #[doc = "Bits 6:11 - Computed syndrome value for bits 31-16"]
    #[inline(always)]
    pub fn synd_enc_31_16(&mut self) -> SyndEnc31_16W<'_, SyndEnc32_44Spec> {
        SyndEnc31_16W::new(self, 6)
    }
}
#[doc = "EDAC Encode\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_enc_32_44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synd_enc_32_44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndEnc32_44Spec;
impl crate::RegisterSpec for SyndEnc32_44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_enc_32_44::R`](R) reader structure"]
impl crate::Readable for SyndEnc32_44Spec {}
#[doc = "`write(|w| ..)` method takes [`synd_enc_32_44::W`](W) writer structure"]
impl crate::Writable for SyndEnc32_44Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYND_ENC_32_44 to value 0"]
impl crate::Resettable for SyndEnc32_44Spec {}
