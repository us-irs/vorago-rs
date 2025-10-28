#[doc = "Register `RND_SOURCE_ENABLE` reader"]
pub type R = crate::R<RndSourceEnableSpec>;
#[doc = "Register `RND_SOURCE_ENABLE` writer"]
pub type W = crate::W<RndSourceEnableSpec>;
#[doc = "Field `RND_SRC_EN` reader - The entropy source, ring oscillator, is enabled"]
pub type RndSrcEnR = crate::BitReader;
#[doc = "Field `RND_SRC_EN` writer - The entropy source, ring oscillator, is enabled"]
pub type RndSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The entropy source, ring oscillator, is enabled"]
    #[inline(always)]
    pub fn rnd_src_en(&self) -> RndSrcEnR {
        RndSrcEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The entropy source, ring oscillator, is enabled"]
    #[inline(always)]
    pub fn rnd_src_en(&mut self) -> RndSrcEnW<'_, RndSourceEnableSpec> {
        RndSrcEnW::new(self, 0)
    }
}
#[doc = "Random Source Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rnd_source_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rnd_source_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RndSourceEnableSpec;
impl crate::RegisterSpec for RndSourceEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnd_source_enable::R`](R) reader structure"]
impl crate::Readable for RndSourceEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`rnd_source_enable::W`](W) writer structure"]
impl crate::Writable for RndSourceEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RND_SOURCE_ENABLE to value 0"]
impl crate::Resettable for RndSourceEnableSpec {}
