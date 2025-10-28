#[doc = "Register `WDOGPERIPHID3` reader"]
pub type R = crate::R<Wdogperiphid3Spec>;
#[doc = "Field `PERIPHID` reader - Peripheral ID"]
pub type PeriphidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID"]
    #[inline(always)]
    pub fn periphid(&self) -> PeriphidR {
        PeriphidR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogperiphid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdogperiphid3Spec;
impl crate::RegisterSpec for Wdogperiphid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogperiphid3::R`](R) reader structure"]
impl crate::Readable for Wdogperiphid3Spec {}
#[doc = "`reset()` method sets WDOGPERIPHID3 to value 0"]
impl crate::Resettable for Wdogperiphid3Spec {}
