#[doc = "Register `DACSEL0` reader"]
pub type R = crate::R<Dacsel0Spec>;
#[doc = "Register `DACSEL0` writer"]
pub type W = crate::W<Dacsel0Spec>;
#[doc = "Field `DACSEL` reader - DAC trigger source selection value"]
pub type DacselR = crate::FieldReader;
#[doc = "Field `DACSEL` writer - DAC trigger source selection value"]
pub type DacselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DAC trigger source selection value"]
    #[inline(always)]
    pub fn dacsel(&self) -> DacselR {
        DacselR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DAC trigger source selection value"]
    #[inline(always)]
    pub fn dacsel(&mut self) -> DacselW<'_, Dacsel0Spec> {
        DacselW::new(self, 0)
    }
}
#[doc = "Interrupt select for DAC0\n\nYou can [`read`](crate::Reg::read) this register and get [`dacsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dacsel0Spec;
impl crate::RegisterSpec for Dacsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacsel0::R`](R) reader structure"]
impl crate::Readable for Dacsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`dacsel0::W`](W) writer structure"]
impl crate::Writable for Dacsel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DACSEL0 to value 0x1f"]
impl crate::Resettable for Dacsel0Spec {
    const RESET_VALUE: u32 = 0x1f;
}
