#[doc = "Register `PERIPH_ID_4` reader"]
pub type R = crate::R<PeriphId4Spec>;
#[doc = "Register `PERIPH_ID_4` writer"]
pub type W = crate::W<PeriphId4Spec>;
#[doc = "Field `JEP106_C_CODE` reader - JEP106"]
pub type Jep106CCodeR = crate::FieldReader;
#[doc = "Field `JEP106_C_CODE` writer - JEP106"]
pub type Jep106CCodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BLOCK_COUNT` reader - The Number of 4k Address Blocks Required"]
pub type BlockCountR = crate::FieldReader;
#[doc = "Field `BLOCK_COUNT` writer - The Number of 4k Address Blocks Required"]
pub type BlockCountW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - JEP106"]
    #[inline(always)]
    pub fn jep106_c_code(&self) -> Jep106CCodeR {
        Jep106CCodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The Number of 4k Address Blocks Required"]
    #[inline(always)]
    pub fn block_count(&self) -> BlockCountR {
        BlockCountR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - JEP106"]
    #[inline(always)]
    pub fn jep106_c_code(&mut self) -> Jep106CCodeW<'_, PeriphId4Spec> {
        Jep106CCodeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - The Number of 4k Address Blocks Required"]
    #[inline(always)]
    pub fn block_count(&mut self) -> BlockCountW<'_, PeriphId4Spec> {
        BlockCountW::new(self, 4)
    }
}
#[doc = "DMA Peripheral ID 4\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId4Spec;
impl crate::RegisterSpec for PeriphId4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id_4::R`](R) reader structure"]
impl crate::Readable for PeriphId4Spec {}
#[doc = "`write(|w| ..)` method takes [`periph_id_4::W`](W) writer structure"]
impl crate::Writable for PeriphId4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIPH_ID_4 to value 0x04"]
impl crate::Resettable for PeriphId4Spec {
    const RESET_VALUE: u32 = 0x04;
}
