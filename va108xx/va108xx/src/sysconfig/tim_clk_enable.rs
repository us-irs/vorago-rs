#[doc = "Register `TIM_CLK_ENABLE` reader"]
pub type R = crate::R<TimClkEnableSpec>;
#[doc = "Register `TIM_CLK_ENABLE` writer"]
pub type W = crate::W<TimClkEnableSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TIM Enable Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_clk_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_clk_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimClkEnableSpec;
impl crate::RegisterSpec for TimClkEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_clk_enable::R`](R) reader structure"]
impl crate::Readable for TimClkEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`tim_clk_enable::W`](W) writer structure"]
impl crate::Writable for TimClkEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_CLK_ENABLE to value 0"]
impl crate::Resettable for TimClkEnableSpec {}
