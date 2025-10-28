#[doc = "Register `TXFRAMECOUNT_GB` reader"]
pub type R = crate::R<TxframecountGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Frame Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txframecount_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxframecountGbSpec;
impl crate::RegisterSpec for TxframecountGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txframecount_gb::R`](R) reader structure"]
impl crate::Readable for TxframecountGbSpec {}
#[doc = "`reset()` method sets TXFRAMECOUNT_GB to value 0"]
impl crate::Resettable for TxframecountGbSpec {}
