#[doc = "Register `CHNL_SW_REQUEST` writer"]
pub type W = crate::W<ChnlSwRequestSpec>;
#[doc = "Field `CH0` writer - Channel SW request"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Channel SW request"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Channel SW request"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Channel SW request"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel SW request"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, ChnlSwRequestSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel SW request"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, ChnlSwRequestSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel SW request"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, ChnlSwRequestSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel SW request"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, ChnlSwRequestSpec> {
        Ch3W::new(self, 3)
    }
}
#[doc = "DMA channel software request\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_sw_request::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlSwRequestSpec;
impl crate::RegisterSpec for ChnlSwRequestSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chnl_sw_request::W`](W) writer structure"]
impl crate::Writable for ChnlSwRequestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL_SW_REQUEST to value 0"]
impl crate::Resettable for ChnlSwRequestSpec {}
