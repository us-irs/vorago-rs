#[doc = "Register `PERIPH_ID_0` reader"]
pub type R = crate::R<PeriphId0Spec>;
#[doc = "Register `PERIPH_ID_0` writer"]
pub type W = crate::W<PeriphId0Spec>;
#[doc = "Field `PART_NUMBER_0` reader - Part Number"]
pub type PartNumber0R = crate::FieldReader;
#[doc = "Field `PART_NUMBER_0` writer - Part Number"]
pub type PartNumber0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Part Number"]
    #[inline(always)]
    pub fn part_number_0(&self) -> PartNumber0R {
        PartNumber0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Part Number"]
    #[inline(always)]
    pub fn part_number_0(&mut self) -> PartNumber0W<'_, PeriphId0Spec> {
        PartNumber0W::new(self, 0)
    }
}
#[doc = "DMA Peripheral ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId0Spec;
impl crate::RegisterSpec for PeriphId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id_0::R`](R) reader structure"]
impl crate::Readable for PeriphId0Spec {}
#[doc = "`write(|w| ..)` method takes [`periph_id_0::W`](W) writer structure"]
impl crate::Writable for PeriphId0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIPH_ID_0 to value 0x30"]
impl crate::Resettable for PeriphId0Spec {
    const RESET_VALUE: u32 = 0x30;
}
