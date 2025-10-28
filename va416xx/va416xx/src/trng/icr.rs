#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `EHR_VALID` reader - Set to 1 after the EHR_DATA\\[0,1,2,3,4,5\\] registers have been read"]
pub type EhrValidR = crate::BitReader;
#[doc = "Field `EHR_VALID` writer - Set to 1 after the EHR_DATA\\[0,1,2,3,4,5\\] registers have been read"]
pub type EhrValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCORR_ERR` reader - Software cannot clear this bit. Only a TRNG reset can clear this bit"]
pub type AutocorrErrR = crate::BitReader;
#[doc = "Field `AUTOCORR_ERR` writer - Software cannot clear this bit. Only a TRNG reset can clear this bit"]
pub type AutocorrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRNGT_ERR` reader - Clear a Continuous Random Number Generation Testing (CRNGT) error"]
pub type CrngtErrR = crate::BitReader;
#[doc = "Field `CRNGT_ERR` writer - Clear a Continuous Random Number Generation Testing (CRNGT) error"]
pub type CrngtErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VN_ERR` reader - Clears a Von Neumann error"]
pub type VnErrR = crate::BitReader;
#[doc = "Field `VN_ERR` writer - Clears a Von Neumann error"]
pub type VnErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set to 1 after the EHR_DATA\\[0,1,2,3,4,5\\] registers have been read"]
    #[inline(always)]
    pub fn ehr_valid(&self) -> EhrValidR {
        EhrValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software cannot clear this bit. Only a TRNG reset can clear this bit"]
    #[inline(always)]
    pub fn autocorr_err(&self) -> AutocorrErrR {
        AutocorrErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear a Continuous Random Number Generation Testing (CRNGT) error"]
    #[inline(always)]
    pub fn crngt_err(&self) -> CrngtErrR {
        CrngtErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clears a Von Neumann error"]
    #[inline(always)]
    pub fn vn_err(&self) -> VnErrR {
        VnErrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to 1 after the EHR_DATA\\[0,1,2,3,4,5\\] registers have been read"]
    #[inline(always)]
    pub fn ehr_valid(&mut self) -> EhrValidW<'_, IcrSpec> {
        EhrValidW::new(self, 0)
    }
    #[doc = "Bit 1 - Software cannot clear this bit. Only a TRNG reset can clear this bit"]
    #[inline(always)]
    pub fn autocorr_err(&mut self) -> AutocorrErrW<'_, IcrSpec> {
        AutocorrErrW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear a Continuous Random Number Generation Testing (CRNGT) error"]
    #[inline(always)]
    pub fn crngt_err(&mut self) -> CrngtErrW<'_, IcrSpec> {
        CrngtErrW::new(self, 2)
    }
    #[doc = "Bit 3 - Clears a Von Neumann error"]
    #[inline(always)]
    pub fn vn_err(&mut self) -> VnErrW<'_, IcrSpec> {
        VnErrW::new(self, 3)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
