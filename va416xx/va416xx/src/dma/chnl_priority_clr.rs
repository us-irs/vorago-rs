#[doc = "Register `CHNL_PRIORITY_CLR` writer"]
pub type W = crate::W<ChnlPriorityClrSpec>;
#[doc = "Field `CH0` writer - Channel PRIORITY clear"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Channel PRIORITY clear"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Channel PRIORITY clear"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Channel PRIORITY clear"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel PRIORITY clear"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, ChnlPriorityClrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel PRIORITY clear"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, ChnlPriorityClrSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel PRIORITY clear"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, ChnlPriorityClrSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel PRIORITY clear"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, ChnlPriorityClrSpec> {
        Ch3W::new(self, 3)
    }
}
#[doc = "DMA channel priority clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_priority_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlPriorityClrSpec;
impl crate::RegisterSpec for ChnlPriorityClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chnl_priority_clr::W`](W) writer structure"]
impl crate::Writable for ChnlPriorityClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL_PRIORITY_CLR to value 0"]
impl crate::Resettable for ChnlPriorityClrSpec {}
