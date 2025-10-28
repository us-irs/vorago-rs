#[doc = "Register `PRIMECELL_ID_3` reader"]
pub type R = crate::R<PrimecellId3Spec>;
#[doc = "Register `PRIMECELL_ID_3` writer"]
pub type W = crate::W<PrimecellId3Spec>;
#[doc = "Field `PRIMECELL_ID_3` reader - PrimeCell Identification"]
pub type PrimecellId3R = crate::FieldReader;
#[doc = "Field `PRIMECELL_ID_3` writer - PrimeCell Identification"]
pub type PrimecellId3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PrimeCell Identification"]
    #[inline(always)]
    pub fn primecell_id_3(&self) -> PrimecellId3R {
        PrimecellId3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PrimeCell Identification"]
    #[inline(always)]
    pub fn primecell_id_3(&mut self) -> PrimecellId3W<'_, PrimecellId3Spec> {
        PrimecellId3W::new(self, 0)
    }
}
#[doc = "DMA PrimeCell ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`primecell_id_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`primecell_id_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrimecellId3Spec;
impl crate::RegisterSpec for PrimecellId3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`primecell_id_3::R`](R) reader structure"]
impl crate::Readable for PrimecellId3Spec {}
#[doc = "`write(|w| ..)` method takes [`primecell_id_3::W`](W) writer structure"]
impl crate::Writable for PrimecellId3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIMECELL_ID_3 to value 0xb1"]
impl crate::Resettable for PrimecellId3Spec {
    const RESET_VALUE: u32 = 0xb1;
}
