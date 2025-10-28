#[doc = "Register `TIM_RESET` reader"]
pub type R = crate::R<TimResetSpec>;
#[doc = "Register `TIM_RESET` writer"]
pub type W = crate::W<TimResetSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TIM Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimResetSpec;
impl crate::RegisterSpec for TimResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_reset::R`](R) reader structure"]
impl crate::Readable for TimResetSpec {}
#[doc = "`write(|w| ..)` method takes [`tim_reset::W`](W) writer structure"]
impl crate::Writable for TimResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM_RESET to value 0xffff_ffff"]
impl crate::Resettable for TimResetSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
