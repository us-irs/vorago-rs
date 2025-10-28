#[doc = "Register `TIMESTAMPADDEND` reader"]
pub type R = crate::R<TimestampaddendSpec>;
#[doc = "Register `TIMESTAMPADDEND` writer"]
pub type W = crate::W<TimestampaddendSpec>;
#[doc = "Field `TSAR` reader - Timestamp Addend Register"]
pub type TsarR = crate::FieldReader<u32>;
#[doc = "Field `TSAR` writer - Timestamp Addend Register"]
pub type TsarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    pub fn tsar(&self) -> TsarR {
        TsarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    pub fn tsar(&mut self) -> TsarW<'_, TimestampaddendSpec> {
        TsarW::new(self, 0)
    }
}
#[doc = "This register is used by software to re-adjust the clock frequency linearly to match the Master clock frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`timestampaddend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestampaddend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimestampaddendSpec;
impl crate::RegisterSpec for TimestampaddendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestampaddend::R`](R) reader structure"]
impl crate::Readable for TimestampaddendSpec {}
#[doc = "`write(|w| ..)` method takes [`timestampaddend::W`](W) writer structure"]
impl crate::Writable for TimestampaddendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMESTAMPADDEND to value 0"]
impl crate::Resettable for TimestampaddendSpec {}
