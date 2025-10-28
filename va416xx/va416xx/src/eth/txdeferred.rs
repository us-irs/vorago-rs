#[doc = "Register `TXDEFERRED` reader"]
pub type R = crate::R<TxdeferredSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of successfully transmitted frames after a deferral\n\nYou can [`read`](crate::Reg::read) this register and get [`txdeferred::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdeferredSpec;
impl crate::RegisterSpec for TxdeferredSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdeferred::R`](R) reader structure"]
impl crate::Readable for TxdeferredSpec {}
#[doc = "`reset()` method sets TXDEFERRED to value 0"]
impl crate::Resettable for TxdeferredSpec {}
