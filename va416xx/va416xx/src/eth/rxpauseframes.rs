#[doc = "Register `RXPAUSEFRAMES` reader"]
pub type R = crate::R<RxpauseframesSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good and valid Pause frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`rxpauseframes::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxpauseframesSpec;
impl crate::RegisterSpec for RxpauseframesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxpauseframes::R`](R) reader structure"]
impl crate::Readable for RxpauseframesSpec {}
#[doc = "`reset()` method sets RXPAUSEFRAMES to value 0"]
impl crate::Resettable for RxpauseframesSpec {}
