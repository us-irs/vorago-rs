#[doc = "Register `PWM_VALUE` reader"]
pub type R = crate::R<PwmValueSpec>;
#[doc = "Register `PWM_VALUE` writer"]
pub type W = crate::W<PwmValueSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The Pulse Width Modulation Value\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmValueSpec;
impl crate::RegisterSpec for PwmValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_value::R`](R) reader structure"]
impl crate::Readable for PwmValueSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm_value::W`](W) writer structure"]
impl crate::Writable for PwmValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM_VALUE to value 0"]
impl crate::Resettable for PwmValueSpec {}
