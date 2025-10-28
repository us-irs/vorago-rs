#[doc = "Register `TXUNDERERR` reader"]
pub type R = crate::R<TxundererrSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC number of frames aborted because of frame underflow error\n\nYou can [`read`](crate::Reg::read) this register and get [`txundererr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxundererrSpec;
impl crate::RegisterSpec for TxundererrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txundererr::R`](R) reader structure"]
impl crate::Readable for TxundererrSpec {}
#[doc = "`reset()` method sets TXUNDERERR to value 0"]
impl crate::Resettable for TxundererrSpec {}
