#[doc = "Register `TXOCTETCOUNT_GB` reader"]
pub type R = crate::R<TxoctetcountGbSpec>;
#[doc = "Field `COUNT` reader - Number of bytes"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Transmit Count\n\nYou can [`read`](crate::Reg::read) this register and get [`txoctetcount_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxoctetcountGbSpec;
impl crate::RegisterSpec for TxoctetcountGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txoctetcount_gb::R`](R) reader structure"]
impl crate::Readable for TxoctetcountGbSpec {}
#[doc = "`reset()` method sets TXOCTETCOUNT_GB to value 0"]
impl crate::Resettable for TxoctetcountGbSpec {}
