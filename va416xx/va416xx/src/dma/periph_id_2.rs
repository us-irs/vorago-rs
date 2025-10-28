#[doc = "Register `PERIPH_ID_2` reader"]
pub type R = crate::R<PeriphId2Spec>;
#[doc = "Register `PERIPH_ID_2` writer"]
pub type W = crate::W<PeriphId2Spec>;
#[doc = "Field `JEP106_ID_6_4` reader - JEP106"]
pub type Jep106Id6_4R = crate::FieldReader;
#[doc = "Field `JEP106_ID_6_4` writer - JEP106"]
pub type Jep106Id6_4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JEDEC_USED` reader - JEDEC"]
pub type JedecUsedR = crate::BitReader;
#[doc = "Field `JEDEC_USED` writer - JEDEC"]
pub type JedecUsedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REVISION` reader - Revision"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - Revision"]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - JEP106"]
    #[inline(always)]
    pub fn jep106_id_6_4(&self) -> Jep106Id6_4R {
        Jep106Id6_4R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - JEDEC"]
    #[inline(always)]
    pub fn jedec_used(&self) -> JedecUsedR {
        JedecUsedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - JEP106"]
    #[inline(always)]
    pub fn jep106_id_6_4(&mut self) -> Jep106Id6_4W<'_, PeriphId2Spec> {
        Jep106Id6_4W::new(self, 0)
    }
    #[doc = "Bit 3 - JEDEC"]
    #[inline(always)]
    pub fn jedec_used(&mut self) -> JedecUsedW<'_, PeriphId2Spec> {
        JedecUsedW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Revision"]
    #[inline(always)]
    pub fn revision(&mut self) -> RevisionW<'_, PeriphId2Spec> {
        RevisionW::new(self, 4)
    }
}
#[doc = "DMA Peripheral ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId2Spec;
impl crate::RegisterSpec for PeriphId2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id_2::R`](R) reader structure"]
impl crate::Readable for PeriphId2Spec {}
#[doc = "`write(|w| ..)` method takes [`periph_id_2::W`](W) writer structure"]
impl crate::Writable for PeriphId2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIPH_ID_2 to value 0xbc"]
impl crate::Resettable for PeriphId2Spec {
    const RESET_VALUE: u32 = 0xbc;
}
