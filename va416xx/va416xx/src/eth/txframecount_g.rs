#[doc = "Register `TXFRAMECOUNT_G` reader"]
pub type R = crate::R<TxframecountGSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`txframecount_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxframecountGSpec;
impl crate::RegisterSpec for TxframecountGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txframecount_g::R`](R) reader structure"]
impl crate::Readable for TxframecountGSpec {}
#[doc = "`reset()` method sets TXFRAMECOUNT_G to value 0"]
impl crate::Resettable for TxframecountGSpec {}
