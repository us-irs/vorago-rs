#[doc = "Register `TXBCASTFRAMES_G` reader"]
pub type R = crate::R<TxbcastframesGSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Good Broadcast Frames Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcastframes_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcastframesGSpec;
impl crate::RegisterSpec for TxbcastframesGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcastframes_g::R`](R) reader structure"]
impl crate::Readable for TxbcastframesGSpec {}
#[doc = "`reset()` method sets TXBCASTFRAMES_G to value 0"]
impl crate::Resettable for TxbcastframesGSpec {}
