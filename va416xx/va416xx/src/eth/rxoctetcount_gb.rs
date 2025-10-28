#[doc = "Register `RXOCTETCOUNT_GB` reader"]
pub type R = crate::R<RxoctetcountGbSpec>;
#[doc = "Field `COUNT` reader - Number of bytes"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of bytes received in good and bad frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoctetcount_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxoctetcountGbSpec;
impl crate::RegisterSpec for RxoctetcountGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxoctetcount_gb::R`](R) reader structure"]
impl crate::Readable for RxoctetcountGbSpec {}
#[doc = "`reset()` method sets RXOCTETCOUNT_GB to value 0"]
impl crate::Resettable for RxoctetcountGbSpec {}
