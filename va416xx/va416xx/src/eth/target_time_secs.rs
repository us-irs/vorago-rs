#[doc = "Register `TARGET_TIME_SECS` reader"]
pub type R = crate::R<TargetTimeSecsSpec>;
#[doc = "Register `TARGET_TIME_SECS` writer"]
pub type W = crate::W<TargetTimeSecsSpec>;
#[doc = "Field `TSTR` reader - Target Time Seconds Registe"]
pub type TstrR = crate::FieldReader<u32>;
#[doc = "Field `TSTR` writer - Target Time Seconds Registe"]
pub type TstrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Target Time Seconds Registe"]
    #[inline(always)]
    pub fn tstr(&self) -> TstrR {
        TstrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target Time Seconds Registe"]
    #[inline(always)]
    pub fn tstr(&mut self) -> TstrW<'_, TargetTimeSecsSpec> {
        TstrW::new(self, 0)
    }
}
#[doc = "Holds the high 32-bits of time to be compared with the system time\n\nYou can [`read`](crate::Reg::read) this register and get [`target_time_secs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target_time_secs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TargetTimeSecsSpec;
impl crate::RegisterSpec for TargetTimeSecsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target_time_secs::R`](R) reader structure"]
impl crate::Readable for TargetTimeSecsSpec {}
#[doc = "`write(|w| ..)` method takes [`target_time_secs::W`](W) writer structure"]
impl crate::Writable for TargetTimeSecsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TARGET_TIME_SECS to value 0"]
impl crate::Resettable for TargetTimeSecsSpec {}
