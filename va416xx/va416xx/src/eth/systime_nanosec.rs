#[doc = "Register `SYSTIME_NANOSEC` reader"]
pub type R = crate::R<SystimeNanosecSpec>;
#[doc = "Field `TSSS` reader - Timestamp Sub Seconds"]
pub type TsssR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub Seconds"]
    #[inline(always)]
    pub fn tsss(&self) -> TsssR {
        TsssR::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "Holds 32 bits of the nano-second field of the system time\n\nYou can [`read`](crate::Reg::read) this register and get [`systime_nanosec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystimeNanosecSpec;
impl crate::RegisterSpec for SystimeNanosecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systime_nanosec::R`](R) reader structure"]
impl crate::Readable for SystimeNanosecSpec {}
#[doc = "`reset()` method sets SYSTIME_NANOSEC to value 0"]
impl crate::Resettable for SystimeNanosecSpec {}
