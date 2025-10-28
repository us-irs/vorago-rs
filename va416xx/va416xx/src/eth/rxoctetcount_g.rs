#[doc = "Register `RXOCTETCOUNT_G` reader"]
pub type R = crate::R<RxoctetcountGSpec>;
#[doc = "Field `COUNT` reader - Number of bytes"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of bytes received in good frames only\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoctetcount_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxoctetcountGSpec;
impl crate::RegisterSpec for RxoctetcountGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxoctetcount_g::R`](R) reader structure"]
impl crate::Readable for RxoctetcountGSpec {}
#[doc = "`reset()` method sets RXOCTETCOUNT_G to value 0"]
impl crate::Resettable for RxoctetcountGSpec {}
