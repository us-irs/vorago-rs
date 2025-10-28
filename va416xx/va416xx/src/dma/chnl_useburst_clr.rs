#[doc = "Register `CHNL_USEBURST_CLR` reader"]
pub type R = crate::R<ChnlUseburstClrSpec>;
#[doc = "Register `CHNL_USEBURST_CLR` writer"]
pub type W = crate::W<ChnlUseburstClrSpec>;
#[doc = "Field `CH0` reader - Channel use burst clear"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - Channel use burst clear"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Channel use burst clear"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - Channel use burst clear"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Channel use burst clear"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - Channel use burst clear"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Channel use burst clear"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - Channel use burst clear"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel use burst clear"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel use burst clear"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel use burst clear"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel use burst clear"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel use burst clear"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, ChnlUseburstClrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel use burst clear"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, ChnlUseburstClrSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel use burst clear"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, ChnlUseburstClrSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel use burst clear"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, ChnlUseburstClrSpec> {
        Ch3W::new(self, 3)
    }
}
#[doc = "DMA channel useburst clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl_useburst_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_useburst_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlUseburstClrSpec;
impl crate::RegisterSpec for ChnlUseburstClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl_useburst_clr::R`](R) reader structure"]
impl crate::Readable for ChnlUseburstClrSpec {}
#[doc = "`write(|w| ..)` method takes [`chnl_useburst_clr::W`](W) writer structure"]
impl crate::Writable for ChnlUseburstClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL_USEBURST_CLR to value 0"]
impl crate::Resettable for ChnlUseburstClrSpec {}
