#[doc = "Register `TXOCTETCOUNT_G` reader"]
pub type R = crate::R<TxoctetcountGSpec>;
#[doc = "Field `COUNT` reader - Number of bytes"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of bytes transmitted frames only in good frames\n\nYou can [`read`](crate::Reg::read) this register and get [`txoctetcount_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxoctetcountGSpec;
impl crate::RegisterSpec for TxoctetcountGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txoctetcount_g::R`](R) reader structure"]
impl crate::Readable for TxoctetcountGSpec {}
#[doc = "`reset()` method sets TXOCTETCOUNT_G to value 0"]
impl crate::Resettable for TxoctetcountGSpec {}
