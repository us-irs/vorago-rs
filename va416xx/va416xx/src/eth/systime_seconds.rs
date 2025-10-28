#[doc = "Register `SYSTIME_SECONDS` reader"]
pub type R = crate::R<SystimeSecondsSpec>;
#[doc = "Field `TSS` reader - Timestamp Second"]
pub type TssR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn tss(&self) -> TssR {
        TssR::new(self.bits)
    }
}
#[doc = "Holds the lower 32 bits of the second field of the system time\n\nYou can [`read`](crate::Reg::read) this register and get [`systime_seconds::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystimeSecondsSpec;
impl crate::RegisterSpec for SystimeSecondsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systime_seconds::R`](R) reader structure"]
impl crate::Readable for SystimeSecondsSpec {}
#[doc = "`reset()` method sets SYSTIME_SECONDS to value 0"]
impl crate::Resettable for SystimeSecondsSpec {}
