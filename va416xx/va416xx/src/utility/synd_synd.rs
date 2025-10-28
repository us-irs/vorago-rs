#[doc = "Register `SYND_SYND` reader"]
pub type R = crate::R<SyndSyndSpec>;
#[doc = "Register `SYND_SYND` writer"]
pub type W = crate::W<SyndSyndSpec>;
#[doc = "Field `SYND_SYND` reader - Provides bits 11:0 for syndrome, 2x6-bit"]
pub type SyndSyndR = crate::FieldReader<u16>;
#[doc = "Field `SYND_SYND` writer - Provides bits 11:0 for syndrome, 2x6-bit"]
pub type SyndSyndW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Provides bits 11:0 for syndrome, 2x6-bit"]
    #[inline(always)]
    pub fn synd_synd(&self) -> SyndSyndR {
        SyndSyndR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Provides bits 11:0 for syndrome, 2x6-bit"]
    #[inline(always)]
    pub fn synd_synd(&mut self) -> SyndSyndW<'_, SyndSyndSpec> {
        SyndSyndW::new(self, 0)
    }
}
#[doc = "Syndrome Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_synd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synd_synd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndSyndSpec;
impl crate::RegisterSpec for SyndSyndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_synd::R`](R) reader structure"]
impl crate::Readable for SyndSyndSpec {}
#[doc = "`write(|w| ..)` method takes [`synd_synd::W`](W) writer structure"]
impl crate::Writable for SyndSyndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYND_SYND to value 0"]
impl crate::Resettable for SyndSyndSpec {}
