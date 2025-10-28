#[doc = "Register `RXRUNTERROR` reader"]
pub type R = crate::R<RxrunterrorSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of frames received with runt error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxrunterror::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxrunterrorSpec;
impl crate::RegisterSpec for RxrunterrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxrunterror::R`](R) reader structure"]
impl crate::Readable for RxrunterrorSpec {}
#[doc = "`reset()` method sets RXRUNTERROR to value 0"]
impl crate::Resettable for RxrunterrorSpec {}
