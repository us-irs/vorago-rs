#[doc = "Register `TXOVERSIZE_G` reader"]
pub type R = crate::R<TxoversizeGSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of frames transmitted without errors\n\nYou can [`read`](crate::Reg::read) this register and get [`txoversize_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxoversizeGSpec;
impl crate::RegisterSpec for TxoversizeGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txoversize_g::R`](R) reader structure"]
impl crate::Readable for TxoversizeGSpec {}
#[doc = "`reset()` method sets TXOVERSIZE_G to value 0"]
impl crate::Resettable for TxoversizeGSpec {}
