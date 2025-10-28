#[doc = "Register `WDOGPERIPHID2` reader"]
pub type R = crate::R<Wdogperiphid2Spec>;
#[doc = "Field `PERIPHID` reader - Peripheral ID"]
pub type PeriphidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID"]
    #[inline(always)]
    pub fn periphid(&self) -> PeriphidR {
        PeriphidR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogperiphid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdogperiphid2Spec;
impl crate::RegisterSpec for Wdogperiphid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogperiphid2::R`](R) reader structure"]
impl crate::Readable for Wdogperiphid2Spec {}
#[doc = "`reset()` method sets WDOGPERIPHID2 to value 0x1b"]
impl crate::Resettable for Wdogperiphid2Spec {
    const RESET_VALUE: u32 = 0x1b;
}
