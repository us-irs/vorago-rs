#[doc = "Register `TXMCASTFRAMES_G` reader"]
pub type R = crate::R<TxmcastframesGSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Good Multicast Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txmcastframes_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxmcastframesGSpec;
impl crate::RegisterSpec for TxmcastframesGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmcastframes_g::R`](R) reader structure"]
impl crate::Readable for TxmcastframesGSpec {}
#[doc = "`reset()` method sets TXMCASTFRAMES_G to value 0"]
impl crate::Resettable for TxmcastframesGSpec {}
