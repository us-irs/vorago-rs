#[doc = "Register `PERIPH_ID_3` reader"]
pub type R = crate::R<PeriphId3Spec>;
#[doc = "Register `PERIPH_ID_3` writer"]
pub type W = crate::W<PeriphId3Spec>;
#[doc = "Field `MOD_NUMBER` reader - Controller Modifications"]
pub type ModNumberR = crate::FieldReader;
#[doc = "Field `MOD_NUMBER` writer - Controller Modifications"]
pub type ModNumberW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Controller Modifications"]
    #[inline(always)]
    pub fn mod_number(&self) -> ModNumberR {
        ModNumberR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Controller Modifications"]
    #[inline(always)]
    pub fn mod_number(&mut self) -> ModNumberW<'_, PeriphId3Spec> {
        ModNumberW::new(self, 0)
    }
}
#[doc = "DMA Peripheral ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId3Spec;
impl crate::RegisterSpec for PeriphId3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id_3::R`](R) reader structure"]
impl crate::Readable for PeriphId3Spec {}
#[doc = "`write(|w| ..)` method takes [`periph_id_3::W`](W) writer structure"]
impl crate::Writable for PeriphId3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIPH_ID_3 to value 0"]
impl crate::Resettable for PeriphId3Spec {}
