#[doc = "Register `RXUNDERSIZE_G` reader"]
pub type R = crate::R<RxundersizeGSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of frames received with length less than 64 bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rxundersize_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxundersizeGSpec;
impl crate::RegisterSpec for RxundersizeGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxundersize_g::R`](R) reader structure"]
impl crate::Readable for RxundersizeGSpec {}
#[doc = "`reset()` method sets RXUNDERSIZE_G to value 0"]
impl crate::Resettable for RxundersizeGSpec {}
