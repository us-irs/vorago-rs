#[doc = "Register `RXBCASTFRAMES_G` reader"]
pub type R = crate::R<RxbcastframesGSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good broadcast frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbcastframes_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxbcastframesGSpec;
impl crate::RegisterSpec for RxbcastframesGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbcastframes_g::R`](R) reader structure"]
impl crate::Readable for RxbcastframesGSpec {}
#[doc = "`reset()` method sets RXBCASTFRAMES_G to value 0"]
impl crate::Resettable for RxbcastframesGSpec {}
