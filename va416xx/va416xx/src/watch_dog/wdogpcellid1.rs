#[doc = "Register `WDOGPCELLID1` reader"]
pub type R = crate::R<Wdogpcellid1Spec>;
#[doc = "Field `PCELLID` reader - Prime Cell ID"]
pub type PcellidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Prime Cell ID"]
    #[inline(always)]
    pub fn pcellid(&self) -> PcellidR {
        PcellidR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PrimeCell ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogpcellid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdogpcellid1Spec;
impl crate::RegisterSpec for Wdogpcellid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogpcellid1::R`](R) reader structure"]
impl crate::Readable for Wdogpcellid1Spec {}
#[doc = "`reset()` method sets WDOGPCELLID1 to value 0xf0"]
impl crate::Resettable for Wdogpcellid1Spec {
    const RESET_VALUE: u32 = 0xf0;
}
