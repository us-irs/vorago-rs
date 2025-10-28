#[doc = "Register `TC` reader"]
pub type R = crate::R<TcSpec>;
#[doc = "Register `TC` writer"]
pub type W = crate::W<TcSpec>;
#[doc = "Field `TIMECNT` reader - The current value of the system time counter"]
pub type TimecntR = crate::FieldReader;
#[doc = "Field `TIMECNT` writer - The current value of the system time counter"]
pub type TimecntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TIRQ_END` reader - The current value of the time control flags"]
pub type TirqEndR = crate::FieldReader;
#[doc = "Field `TIRQ_END` writer - The current value of the time control flags"]
pub type TirqEndW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - The current value of the system time counter"]
    #[inline(always)]
    pub fn timecnt(&self) -> TimecntR {
        TimecntR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - The current value of the time control flags"]
    #[inline(always)]
    pub fn tirq_end(&self) -> TirqEndR {
        TirqEndR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The current value of the system time counter"]
    #[inline(always)]
    pub fn timecnt(&mut self) -> TimecntW<'_, TcSpec> {
        TimecntW::new(self, 0)
    }
    #[doc = "Bits 6:7 - The current value of the time control flags"]
    #[inline(always)]
    pub fn tirq_end(&mut self) -> TirqEndW<'_, TcSpec> {
        TirqEndW::new(self, 6)
    }
}
#[doc = "Time Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcSpec;
impl crate::RegisterSpec for TcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tc::R`](R) reader structure"]
impl crate::Readable for TcSpec {}
#[doc = "`write(|w| ..)` method takes [`tc::W`](W) writer structure"]
impl crate::Writable for TcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TC to value 0"]
impl crate::Resettable for TcSpec {}
