#[doc = "Register `RXLENGTHERROR` reader"]
pub type R = crate::R<RxlengtherrorSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of frames received with length error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxlengtherror::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxlengtherrorSpec;
impl crate::RegisterSpec for RxlengtherrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlengtherror::R`](R) reader structure"]
impl crate::Readable for RxlengtherrorSpec {}
#[doc = "`reset()` method sets RXLENGTHERROR to value 0"]
impl crate::Resettable for RxlengtherrorSpec {}
