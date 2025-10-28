#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<Ctrl0Spec>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<Ctrl0Spec>;
#[doc = "Field `EXT_TRIG_EN` reader - Enables external trigger"]
pub type ExtTrigEnR = crate::BitReader;
#[doc = "Field `EXT_TRIG_EN` writer - Enables external trigger"]
pub type ExtTrigEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAN_TRIG_EN` reader - Enables manual trigger"]
pub type ManTrigEnR = crate::BitReader;
#[doc = "Field `MAN_TRIG_EN` writer - Enables manual trigger"]
pub type ManTrigEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - Enables external trigger"]
    #[inline(always)]
    pub fn ext_trig_en(&self) -> ExtTrigEnR {
        ExtTrigEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables manual trigger"]
    #[inline(always)]
    pub fn man_trig_en(&self) -> ManTrigEnR {
        ManTrigEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Enables external trigger"]
    #[inline(always)]
    pub fn ext_trig_en(&mut self) -> ExtTrigEnW<'_, Ctrl0Spec> {
        ExtTrigEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Enables manual trigger"]
    #[inline(always)]
    pub fn man_trig_en(&mut self) -> ManTrigEnW<'_, Ctrl0Spec> {
        ManTrigEnW::new(self, 11)
    }
}
#[doc = "Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl0Spec;
impl crate::RegisterSpec for Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for Ctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for Ctrl0Spec {}
