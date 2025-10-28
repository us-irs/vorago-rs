#[doc = "Register `SUBSEC_INC` reader"]
pub type R = crate::R<SubsecIncSpec>;
#[doc = "Register `SUBSEC_INC` writer"]
pub type W = crate::W<SubsecIncSpec>;
#[doc = "Field `SSINC` reader - Sub-Second Increment Valuee"]
pub type SsincR = crate::FieldReader;
#[doc = "Field `SSINC` writer - Sub-Second Increment Valuee"]
pub type SsincW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sub-Second Increment Valuee"]
    #[inline(always)]
    pub fn ssinc(&self) -> SsincR {
        SsincR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sub-Second Increment Valuee"]
    #[inline(always)]
    pub fn ssinc(&mut self) -> SsincW<'_, SubsecIncSpec> {
        SsincW::new(self, 0)
    }
}
#[doc = "Holds the 8-bit value by which the Sub-Second register is incremented\n\nYou can [`read`](crate::Reg::read) this register and get [`subsec_inc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subsec_inc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubsecIncSpec;
impl crate::RegisterSpec for SubsecIncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`subsec_inc::R`](R) reader structure"]
impl crate::Readable for SubsecIncSpec {}
#[doc = "`write(|w| ..)` method takes [`subsec_inc::W`](W) writer structure"]
impl crate::Writable for SubsecIncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SUBSEC_INC to value 0"]
impl crate::Resettable for SubsecIncSpec {}
