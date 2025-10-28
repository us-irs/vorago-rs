#[doc = "Register `PRIMECELL_ID_2` reader"]
pub type R = crate::R<PrimecellId2Spec>;
#[doc = "Register `PRIMECELL_ID_2` writer"]
pub type W = crate::W<PrimecellId2Spec>;
#[doc = "Field `PRIMECELL_ID_2` reader - PrimeCell Identification"]
pub type PrimecellId2R = crate::FieldReader;
#[doc = "Field `PRIMECELL_ID_2` writer - PrimeCell Identification"]
pub type PrimecellId2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PrimeCell Identification"]
    #[inline(always)]
    pub fn primecell_id_2(&self) -> PrimecellId2R {
        PrimecellId2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PrimeCell Identification"]
    #[inline(always)]
    pub fn primecell_id_2(&mut self) -> PrimecellId2W<'_, PrimecellId2Spec> {
        PrimecellId2W::new(self, 0)
    }
}
#[doc = "DMA PrimeCell ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`primecell_id_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`primecell_id_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrimecellId2Spec;
impl crate::RegisterSpec for PrimecellId2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`primecell_id_2::R`](R) reader structure"]
impl crate::Readable for PrimecellId2Spec {}
#[doc = "`write(|w| ..)` method takes [`primecell_id_2::W`](W) writer structure"]
impl crate::Writable for PrimecellId2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIMECELL_ID_2 to value 0x05"]
impl crate::Resettable for PrimecellId2Spec {
    const RESET_VALUE: u32 = 0x05;
}
