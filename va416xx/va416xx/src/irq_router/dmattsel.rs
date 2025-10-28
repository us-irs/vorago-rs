#[doc = "Register `DMATTSEL` reader"]
pub type R = crate::R<DmattselSpec>;
#[doc = "Register `DMATTSEL` writer"]
pub type W = crate::W<DmattselSpec>;
#[doc = "Field `DMATTSEL` reader - DMA trigger type selection value"]
pub type DmattselR = crate::FieldReader;
#[doc = "Field `DMATTSEL` writer - DMA trigger type selection value"]
pub type DmattselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DMA trigger type selection value"]
    #[inline(always)]
    pub fn dmattsel(&self) -> DmattselR {
        DmattselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMA trigger type selection value"]
    #[inline(always)]
    pub fn dmattsel(&mut self) -> DmattselW<'_, DmattselSpec> {
        DmattselW::new(self, 0)
    }
}
#[doc = "Trigger select for the DMA channels\n\nYou can [`read`](crate::Reg::read) this register and get [`dmattsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmattsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmattselSpec;
impl crate::RegisterSpec for DmattselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmattsel::R`](R) reader structure"]
impl crate::Readable for DmattselSpec {}
#[doc = "`write(|w| ..)` method takes [`dmattsel::W`](W) writer structure"]
impl crate::Writable for DmattselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMATTSEL to value 0"]
impl crate::Resettable for DmattselSpec {}
