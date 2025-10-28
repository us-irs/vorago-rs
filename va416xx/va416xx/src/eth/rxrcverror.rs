#[doc = "Register `RXRCVERROR` reader"]
pub type R = crate::R<RxrcverrorSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of frames received with Receive error or Frame Extension error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxrcverror::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxrcverrorSpec;
impl crate::RegisterSpec for RxrcverrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxrcverror::R`](R) reader structure"]
impl crate::Readable for RxrcverrorSpec {}
#[doc = "`reset()` method sets RXRCVERROR to value 0"]
impl crate::Resettable for RxrcverrorSpec {}
