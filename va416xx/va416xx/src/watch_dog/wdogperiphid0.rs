#[doc = "Register `WDOGPERIPHID0` reader"]
pub type R = crate::R<Wdogperiphid0Spec>;
#[doc = "Field `PERIPHID` reader - Peripheral ID"]
pub type PeriphidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID"]
    #[inline(always)]
    pub fn periphid(&self) -> PeriphidR {
        PeriphidR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogperiphid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdogperiphid0Spec;
impl crate::RegisterSpec for Wdogperiphid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogperiphid0::R`](R) reader structure"]
impl crate::Readable for Wdogperiphid0Spec {}
#[doc = "`reset()` method sets WDOGPERIPHID0 to value 0x24"]
impl crate::Resettable for Wdogperiphid0Spec {
    const RESET_VALUE: u32 = 0x24;
}
