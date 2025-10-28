#[doc = "Register `CASCADE0` reader"]
pub type R = crate::R<Cascade0Spec>;
#[doc = "Register `CASCADE0` writer"]
pub type W = crate::W<Cascade0Spec>;
#[doc = "Field `CASSEL` reader - Cascade Selection"]
pub type CasselR = crate::FieldReader;
#[doc = "Field `CASSEL` writer - Cascade Selection"]
pub type CasselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Cascade Selection"]
    #[inline(always)]
    pub fn cassel(&self) -> CasselR {
        CasselR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cascade Selection"]
    #[inline(always)]
    pub fn cassel(&mut self) -> CasselW<'_, Cascade0Spec> {
        CasselW::new(self, 0)
    }
}
#[doc = "Cascade Enable Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`cascade0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cascade0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cascade0Spec;
impl crate::RegisterSpec for Cascade0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cascade0::R`](R) reader structure"]
impl crate::Readable for Cascade0Spec {}
#[doc = "`write(|w| ..)` method takes [`cascade0::W`](W) writer structure"]
impl crate::Writable for Cascade0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CASCADE0 to value 0"]
impl crate::Resettable for Cascade0Spec {}
