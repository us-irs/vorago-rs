#[doc = "Register `RXALIGNERROR` reader"]
pub type R = crate::R<RxalignerrorSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of frames received with alignment error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxalignerror::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxalignerrorSpec;
impl crate::RegisterSpec for RxalignerrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxalignerror::R`](R) reader structure"]
impl crate::Readable for RxalignerrorSpec {}
#[doc = "`reset()` method sets RXALIGNERROR to value 0"]
impl crate::Resettable for RxalignerrorSpec {}
