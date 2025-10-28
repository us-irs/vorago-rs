#[doc = "Register `WDOGPCELLID2` reader"]
pub type R = crate::R<Wdogpcellid2Spec>;
#[doc = "Field `PCELLID` reader - Prime Cell ID"]
pub type PcellidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Prime Cell ID"]
    #[inline(always)]
    pub fn pcellid(&self) -> PcellidR {
        PcellidR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PrimeCell ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogpcellid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdogpcellid2Spec;
impl crate::RegisterSpec for Wdogpcellid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogpcellid2::R`](R) reader structure"]
impl crate::Readable for Wdogpcellid2Spec {}
#[doc = "`reset()` method sets WDOGPCELLID2 to value 0x05"]
impl crate::Resettable for Wdogpcellid2Spec {
    const RESET_VALUE: u32 = 0x05;
}
