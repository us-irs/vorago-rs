#[doc = "Register `WDOGPCELLID3` reader"]
pub type R = crate::R<Wdogpcellid3Spec>;
#[doc = "Field `PCELLID` reader - Prime Cell ID"]
pub type PcellidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Prime Cell ID"]
    #[inline(always)]
    pub fn pcellid(&self) -> PcellidR {
        PcellidR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PrimeCell ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogpcellid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdogpcellid3Spec;
impl crate::RegisterSpec for Wdogpcellid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogpcellid3::R`](R) reader structure"]
impl crate::Readable for Wdogpcellid3Spec {}
#[doc = "`reset()` method sets WDOGPCELLID3 to value 0xb1"]
impl crate::Resettable for Wdogpcellid3Spec {
    const RESET_VALUE: u32 = 0xb1;
}
