#[doc = "Register `DMASEL1` reader"]
pub type R = crate::R<Dmasel1Spec>;
#[doc = "Register `DMASEL1` writer"]
pub type W = crate::W<Dmasel1Spec>;
#[doc = "Field `DMASEL` reader - DMA trigger source selection value"]
pub type DmaselR = crate::FieldReader;
#[doc = "Field `DMASEL` writer - DMA trigger source selection value"]
pub type DmaselW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - DMA trigger source selection value"]
    #[inline(always)]
    pub fn dmasel(&self) -> DmaselR {
        DmaselR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA trigger source selection value"]
    #[inline(always)]
    pub fn dmasel(&mut self) -> DmaselW<'_, Dmasel1Spec> {
        DmaselW::new(self, 0)
    }
}
#[doc = "Interrupt select for DMA channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmasel1Spec;
impl crate::RegisterSpec for Dmasel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasel1::R`](R) reader structure"]
impl crate::Readable for Dmasel1Spec {}
#[doc = "`write(|w| ..)` method takes [`dmasel1::W`](W) writer structure"]
impl crate::Writable for Dmasel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMASEL1 to value 0x7f"]
impl crate::Resettable for Dmasel1Spec {
    const RESET_VALUE: u32 = 0x7f;
}
