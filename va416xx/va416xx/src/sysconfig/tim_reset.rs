#[doc = "Register `TIM_RESET` reader"]
pub type R = crate::R<TimResetSpec>;
#[doc = "Register `TIM_RESET` writer"]
pub type W = crate::W<TimResetSpec>;
#[doc = "Field `TIM_RESET` reader - Reset of a given TIMER"]
pub type TimResetR = crate::FieldReader<u32>;
#[doc = "Field `TIM_RESET` writer - Reset of a given TIMER"]
pub type TimResetW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Reset of a given TIMER"]
    #[inline(always)]
    pub fn tim_reset(&self) -> TimResetR {
        TimResetR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Reset of a given TIMER"]
    #[inline(always)]
    pub fn tim_reset(&mut self) -> TimResetW<'_, TimResetSpec> {
        TimResetW::new(self, 0)
    }
}
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
