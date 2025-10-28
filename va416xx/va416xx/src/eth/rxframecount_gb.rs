#[doc = "Register `RXFRAMECOUNT_GB` reader"]
pub type R = crate::R<RxframecountGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good and bad frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`rxframecount_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxframecountGbSpec;
impl crate::RegisterSpec for RxframecountGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxframecount_gb::R`](R) reader structure"]
impl crate::Readable for RxframecountGbSpec {}
#[doc = "`reset()` method sets RXFRAMECOUNT_GB to value 0"]
impl crate::Resettable for RxframecountGbSpec {}
