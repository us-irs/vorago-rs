#[doc = "Register `RXOVERSIZE_G` reader"]
pub type R = crate::R<RxoversizeGSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of frames received without errors with length greater than the max size\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoversize_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxoversizeGSpec;
impl crate::RegisterSpec for RxoversizeGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxoversize_g::R`](R) reader structure"]
impl crate::Readable for RxoversizeGSpec {}
#[doc = "`reset()` method sets RXOVERSIZE_G to value 0"]
impl crate::Resettable for RxoversizeGSpec {}
