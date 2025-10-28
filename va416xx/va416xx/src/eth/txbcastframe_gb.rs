#[doc = "Register `TXBCASTFRAME_GB` reader"]
pub type R = crate::R<TxbcastframeGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC number of good and bad broadcast frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcastframe_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcastframeGbSpec;
impl crate::RegisterSpec for TxbcastframeGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcastframe_gb::R`](R) reader structure"]
impl crate::Readable for TxbcastframeGbSpec {}
#[doc = "`reset()` method sets TXBCASTFRAME_GB to value 0"]
impl crate::Resettable for TxbcastframeGbSpec {}
