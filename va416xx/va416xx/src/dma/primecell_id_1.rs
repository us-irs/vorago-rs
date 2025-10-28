#[doc = "Register `PRIMECELL_ID_1` reader"]
pub type R = crate::R<PrimecellId1Spec>;
#[doc = "Register `PRIMECELL_ID_1` writer"]
pub type W = crate::W<PrimecellId1Spec>;
#[doc = "Field `PRIMECELL_ID_1` reader - PrimeCell Identification"]
pub type PrimecellId1R = crate::FieldReader;
#[doc = "Field `PRIMECELL_ID_1` writer - PrimeCell Identification"]
pub type PrimecellId1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PrimeCell Identification"]
    #[inline(always)]
    pub fn primecell_id_1(&self) -> PrimecellId1R {
        PrimecellId1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PrimeCell Identification"]
    #[inline(always)]
    pub fn primecell_id_1(&mut self) -> PrimecellId1W<'_, PrimecellId1Spec> {
        PrimecellId1W::new(self, 0)
    }
}
#[doc = "DMA PrimeCell ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`primecell_id_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`primecell_id_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrimecellId1Spec;
impl crate::RegisterSpec for PrimecellId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`primecell_id_1::R`](R) reader structure"]
impl crate::Readable for PrimecellId1Spec {}
#[doc = "`write(|w| ..)` method takes [`primecell_id_1::W`](W) writer structure"]
impl crate::Writable for PrimecellId1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIMECELL_ID_1 to value 0xf0"]
impl crate::Resettable for PrimecellId1Spec {
    const RESET_VALUE: u32 = 0xf0;
}
