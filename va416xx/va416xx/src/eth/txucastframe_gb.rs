#[doc = "Register `TXUCASTFRAME_GB` reader"]
pub type R = crate::R<TxucastframeGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC number of good and bad unicast frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`txucastframe_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxucastframeGbSpec;
impl crate::RegisterSpec for TxucastframeGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txucastframe_gb::R`](R) reader structure"]
impl crate::Readable for TxucastframeGbSpec {}
#[doc = "`reset()` method sets TXUCASTFRAME_GB to value 0"]
impl crate::Resettable for TxucastframeGbSpec {}
