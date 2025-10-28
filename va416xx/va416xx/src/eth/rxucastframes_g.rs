#[doc = "Register `RXUCASTFRAMES_G` reader"]
pub type R = crate::R<RxucastframesGSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of received good unicast frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rxucastframes_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxucastframesGSpec;
impl crate::RegisterSpec for RxucastframesGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxucastframes_g::R`](R) reader structure"]
impl crate::Readable for RxucastframesGSpec {}
#[doc = "`reset()` method sets RXUCASTFRAMES_G to value 0"]
impl crate::Resettable for RxucastframesGSpec {}
