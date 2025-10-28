#[doc = "Register `WDOGPCELLID0` reader"]
pub type R = crate::R<Wdogpcellid0Spec>;
#[doc = "Field `PCELLID` reader - Prime Cell ID"]
pub type PcellidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Prime Cell ID"]
    #[inline(always)]
    pub fn pcellid(&self) -> PcellidR {
        PcellidR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PrimeCell ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogpcellid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdogpcellid0Spec;
impl crate::RegisterSpec for Wdogpcellid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogpcellid0::R`](R) reader structure"]
impl crate::Readable for Wdogpcellid0Spec {}
#[doc = "`reset()` method sets WDOGPCELLID0 to value 0x0d"]
impl crate::Resettable for Wdogpcellid0Spec {
    const RESET_VALUE: u32 = 0x0d;
}
