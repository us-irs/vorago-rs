#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `RND_SRC_SEL` reader - Selects the number of inverters (out of four possible selections) in the ring oscillator"]
pub type RndSrcSelR = crate::FieldReader;
#[doc = "Field `RND_SRC_SEL` writer - Selects the number of inverters (out of four possible selections) in the ring oscillator"]
pub type RndSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Selects the number of inverters (out of four possible selections) in the ring oscillator"]
    #[inline(always)]
    pub fn rnd_src_sel(&self) -> RndSrcSelR {
        RndSrcSelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the number of inverters (out of four possible selections) in the ring oscillator"]
    #[inline(always)]
    pub fn rnd_src_sel(&mut self) -> RndSrcSelW<'_, ConfigSpec> {
        RndSrcSelW::new(self, 0)
    }
}
#[doc = "Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {}
