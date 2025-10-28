#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `EHR_VALID` reader - 192 bits have been collected in the TRNG"]
pub type EhrValidR = crate::BitReader;
#[doc = "Field `AUTOCORR_ERR` reader - Indicates that the Autocorrelation test failed four times in a row"]
pub type AutocorrErrR = crate::BitReader;
#[doc = "Field `CRNGT_ERR` reader - Indicates a Continuous Random Number Generation Testing (CRNGT) error"]
pub type CrngtErrR = crate::BitReader;
#[doc = "Field `VN_ERR` reader - Indicates a Von Neumann error"]
pub type VnErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 192 bits have been collected in the TRNG"]
    #[inline(always)]
    pub fn ehr_valid(&self) -> EhrValidR {
        EhrValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that the Autocorrelation test failed four times in a row"]
    #[inline(always)]
    pub fn autocorr_err(&self) -> AutocorrErrR {
        AutocorrErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates a Continuous Random Number Generation Testing (CRNGT) error"]
    #[inline(always)]
    pub fn crngt_err(&self) -> CrngtErrR {
        CrngtErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates a Von Neumann error"]
    #[inline(always)]
    pub fn vn_err(&self) -> VnErrR {
        VnErrR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
