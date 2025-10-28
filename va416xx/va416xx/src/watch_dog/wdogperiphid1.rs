#[doc = "Register `WDOGPERIPHID1` reader"]
pub type R = crate::R<Wdogperiphid1Spec>;
#[doc = "Field `PERIPHID` reader - Peripheral ID"]
pub type PeriphidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID"]
    #[inline(always)]
    pub fn periphid(&self) -> PeriphidR {
        PeriphidR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogperiphid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdogperiphid1Spec;
impl crate::RegisterSpec for Wdogperiphid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogperiphid1::R`](R) reader structure"]
impl crate::Readable for Wdogperiphid1Spec {}
#[doc = "`reset()` method sets WDOGPERIPHID1 to value 0xb8"]
impl crate::Resettable for Wdogperiphid1Spec {
    const RESET_VALUE: u32 = 0xb8;
}
