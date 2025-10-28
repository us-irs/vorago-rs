#[doc = "Register `RXWDOGERROR` reader"]
pub type R = crate::R<RxwdogerrorSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of frames received with error because of watchdog timeout error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxwdogerror::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxwdogerrorSpec;
impl crate::RegisterSpec for RxwdogerrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxwdogerror::R`](R) reader structure"]
impl crate::Readable for RxwdogerrorSpec {}
#[doc = "`reset()` method sets RXWDOGERROR to value 0"]
impl crate::Resettable for RxwdogerrorSpec {}
