#[doc = "Register `ADCSEL` reader"]
pub type R = crate::R<AdcselSpec>;
#[doc = "Register `ADCSEL` writer"]
pub type W = crate::W<AdcselSpec>;
#[doc = "Field `ADCSEL` reader - ADC trigger source selection value"]
pub type AdcselR = crate::FieldReader;
#[doc = "Field `ADCSEL` writer - ADC trigger source selection value"]
pub type AdcselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - ADC trigger source selection value"]
    #[inline(always)]
    pub fn adcsel(&self) -> AdcselR {
        AdcselR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC trigger source selection value"]
    #[inline(always)]
    pub fn adcsel(&mut self) -> AdcselW<'_, AdcselSpec> {
        AdcselW::new(self, 0)
    }
}
#[doc = "Interrupt select for ADC\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcselSpec;
impl crate::RegisterSpec for AdcselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcsel::R`](R) reader structure"]
impl crate::Readable for AdcselSpec {}
#[doc = "`write(|w| ..)` method takes [`adcsel::W`](W) writer structure"]
impl crate::Writable for AdcselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCSEL to value 0x1f"]
impl crate::Resettable for AdcselSpec {
    const RESET_VALUE: u32 = 0x1f;
}
