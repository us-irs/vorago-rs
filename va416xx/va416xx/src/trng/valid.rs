#[doc = "Register `VALID` reader"]
pub type R = crate::R<ValidSpec>;
#[doc = "Field `EHR_VALID` reader - Indicates that the collection of bits in the TRNG is complete"]
pub type EhrValidR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates that the collection of bits in the TRNG is complete"]
    #[inline(always)]
    pub fn ehr_valid(&self) -> EhrValidR {
        EhrValidR::new((self.bits & 1) != 0)
    }
}
#[doc = "Valid Register\n\nYou can [`read`](crate::Reg::read) this register and get [`valid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValidSpec;
impl crate::RegisterSpec for ValidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`valid::R`](R) reader structure"]
impl crate::Readable for ValidSpec {}
#[doc = "`reset()` method sets VALID to value 0"]
impl crate::Resettable for ValidSpec {}
