#[doc = "Register `TXMCASTFRAME_GB` reader"]
pub type R = crate::R<TxmcastframeGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC number of good and bad MULTIcast frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`txmcastframe_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxmcastframeGbSpec;
impl crate::RegisterSpec for TxmcastframeGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmcastframe_gb::R`](R) reader structure"]
impl crate::Readable for TxmcastframeGbSpec {}
#[doc = "`reset()` method sets TXMCASTFRAME_GB to value 0"]
impl crate::Resettable for TxmcastframeGbSpec {}
