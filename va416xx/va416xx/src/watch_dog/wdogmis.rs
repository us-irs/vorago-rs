#[doc = "Register `WDOGMIS` reader"]
pub type R = crate::R<WdogmisSpec>;
#[doc = "Field `INTERRUPT` reader - Masked Interrupt Status"]
pub type InterruptR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked Interrupt Status"]
    #[inline(always)]
    pub fn interrupt(&self) -> InterruptR {
        InterruptR::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogmis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogmisSpec;
impl crate::RegisterSpec for WdogmisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogmis::R`](R) reader structure"]
impl crate::Readable for WdogmisSpec {}
#[doc = "`reset()` method sets WDOGMIS to value 0"]
impl crate::Resettable for WdogmisSpec {}
