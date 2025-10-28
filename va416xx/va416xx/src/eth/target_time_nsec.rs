#[doc = "Register `TARGET_TIME_NSEC` reader"]
pub type R = crate::R<TargetTimeNsecSpec>;
#[doc = "Register `TARGET_TIME_NSEC` writer"]
pub type W = crate::W<TargetTimeNsecSpec>;
#[doc = "Field `TTSLO` reader - Target Timestamp Low Register"]
pub type TtsloR = crate::FieldReader<u32>;
#[doc = "Field `TTSLO` writer - Target Timestamp Low Register"]
pub type TtsloW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `TRGTBUSY` reader - 32 Bits of Hash Table"]
pub type TrgtbusyR = crate::BitReader;
#[doc = "Field `TRGTBUSY` writer - 32 Bits of Hash Table"]
pub type TrgtbusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn ttslo(&self) -> TtsloR {
        TtsloR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - 32 Bits of Hash Table"]
    #[inline(always)]
    pub fn trgtbusy(&self) -> TrgtbusyR {
        TrgtbusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn ttslo(&mut self) -> TtsloW<'_, TargetTimeNsecSpec> {
        TtsloW::new(self, 0)
    }
    #[doc = "Bit 31 - 32 Bits of Hash Table"]
    #[inline(always)]
    pub fn trgtbusy(&mut self) -> TrgtbusyW<'_, TargetTimeNsecSpec> {
        TrgtbusyW::new(self, 31)
    }
}
#[doc = "Holds the lower 32-bits of time to be compared with the system time\n\nYou can [`read`](crate::Reg::read) this register and get [`target_time_nsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target_time_nsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TargetTimeNsecSpec;
impl crate::RegisterSpec for TargetTimeNsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target_time_nsec::R`](R) reader structure"]
impl crate::Readable for TargetTimeNsecSpec {}
#[doc = "`write(|w| ..)` method takes [`target_time_nsec::W`](W) writer structure"]
impl crate::Writable for TargetTimeNsecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TARGET_TIME_NSEC to value 0"]
impl crate::Resettable for TargetTimeNsecSpec {}
