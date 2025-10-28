#[doc = "Register `TXPAUSEFRAMES` reader"]
pub type R = crate::R<TxpauseframesSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good pause frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`txpauseframes::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxpauseframesSpec;
impl crate::RegisterSpec for TxpauseframesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpauseframes::R`](R) reader structure"]
impl crate::Readable for TxpauseframesSpec {}
#[doc = "`reset()` method sets TXPAUSEFRAMES to value 0"]
impl crate::Resettable for TxpauseframesSpec {}
