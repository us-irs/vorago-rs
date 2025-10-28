#[doc = "Register `TXEXCESSDEF` reader"]
pub type R = crate::R<TxexcessdefSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of frames aborted because of excessive deferral error\n\nYou can [`read`](crate::Reg::read) this register and get [`txexcessdef::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxexcessdefSpec;
impl crate::RegisterSpec for TxexcessdefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txexcessdef::R`](R) reader structure"]
impl crate::Readable for TxexcessdefSpec {}
#[doc = "`reset()` method sets TXEXCESSDEF to value 0"]
impl crate::Resettable for TxexcessdefSpec {}
