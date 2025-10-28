#[doc = "Register `EHR_DATA0` reader"]
pub type R = crate::R<EhrData0Spec>;
#[doc = "Field `EHR_DATA` reader - 32 Bits of Entropy Holding Register"]
pub type EhrDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 32 Bits of Entropy Holding Register"]
    #[inline(always)]
    pub fn ehr_data(&self) -> EhrDataR {
        EhrDataR::new(self.bits)
    }
}
#[doc = "Entropy Holding Register Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehr_data0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EhrData0Spec;
impl crate::RegisterSpec for EhrData0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehr_data0::R`](R) reader structure"]
impl crate::Readable for EhrData0Spec {}
#[doc = "`reset()` method sets EHR_DATA0 to value 0"]
impl crate::Resettable for EhrData0Spec {}
