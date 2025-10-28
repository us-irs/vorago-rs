#[doc = "Register `TXCARRIERERROR` reader"]
pub type R = crate::R<TxcarriererrorSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of aborted frames because of carrier sense error\n\nYou can [`read`](crate::Reg::read) this register and get [`txcarriererror::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxcarriererrorSpec;
impl crate::RegisterSpec for TxcarriererrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txcarriererror::R`](R) reader structure"]
impl crate::Readable for TxcarriererrorSpec {}
#[doc = "`reset()` method sets TXCARRIERERROR to value 0"]
impl crate::Resettable for TxcarriererrorSpec {}
