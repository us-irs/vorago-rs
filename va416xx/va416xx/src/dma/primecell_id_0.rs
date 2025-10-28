#[doc = "Register `PRIMECELL_ID_0` reader"]
pub type R = crate::R<PrimecellId0Spec>;
#[doc = "Register `PRIMECELL_ID_0` writer"]
pub type W = crate::W<PrimecellId0Spec>;
#[doc = "Field `PRIMECELL_ID_0` reader - PrimeCell Identification"]
pub type PrimecellId0R = crate::FieldReader;
#[doc = "Field `PRIMECELL_ID_0` writer - PrimeCell Identification"]
pub type PrimecellId0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PrimeCell Identification"]
    #[inline(always)]
    pub fn primecell_id_0(&self) -> PrimecellId0R {
        PrimecellId0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PrimeCell Identification"]
    #[inline(always)]
    pub fn primecell_id_0(&mut self) -> PrimecellId0W<'_, PrimecellId0Spec> {
        PrimecellId0W::new(self, 0)
    }
}
#[doc = "DMA PrimeCell ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`primecell_id_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`primecell_id_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrimecellId0Spec;
impl crate::RegisterSpec for PrimecellId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`primecell_id_0::R`](R) reader structure"]
impl crate::Readable for PrimecellId0Spec {}
#[doc = "`write(|w| ..)` method takes [`primecell_id_0::W`](W) writer structure"]
impl crate::Writable for PrimecellId0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIMECELL_ID_0 to value 0x0d"]
impl crate::Resettable for PrimecellId0Spec {
    const RESET_VALUE: u32 = 0x0d;
}
