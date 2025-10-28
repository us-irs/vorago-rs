#[doc = "Register `WDOGVALUE` reader"]
pub type R = crate::R<WdogvalueSpec>;
#[doc = "Field `CNT` reader - Actual Count"]
pub type CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Actual Count"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
#[doc = "Down Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogvalue::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogvalueSpec;
impl crate::RegisterSpec for WdogvalueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogvalue::R`](R) reader structure"]
impl crate::Readable for WdogvalueSpec {}
#[doc = "`reset()` method sets WDOGVALUE to value 0xffff_ffff"]
impl crate::Resettable for WdogvalueSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
