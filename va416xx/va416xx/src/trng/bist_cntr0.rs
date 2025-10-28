#[doc = "Register `BIST_CNTR0` reader"]
pub type R = crate::R<BistCntr0Spec>;
#[doc = "Field `ROSC_CNTR_VAL` reader - Returns the results of the TRNG BIST counter"]
pub type RoscCntrValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Returns the results of the TRNG BIST counter"]
    #[inline(always)]
    pub fn rosc_cntr_val(&self) -> RoscCntrValR {
        RoscCntrValR::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "BIST Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bist_cntr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BistCntr0Spec;
impl crate::RegisterSpec for BistCntr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bist_cntr0::R`](R) reader structure"]
impl crate::Readable for BistCntr0Spec {}
#[doc = "`reset()` method sets BIST_CNTR0 to value 0"]
impl crate::Resettable for BistCntr0Spec {}
