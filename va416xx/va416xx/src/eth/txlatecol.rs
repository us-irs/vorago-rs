#[doc = "Register `TXLATECOL` reader"]
pub type R = crate::R<TxlatecolSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of aborted frames because of late collision error\n\nYou can [`read`](crate::Reg::read) this register and get [`txlatecol::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxlatecolSpec;
impl crate::RegisterSpec for TxlatecolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlatecol::R`](R) reader structure"]
impl crate::Readable for TxlatecolSpec {}
#[doc = "`reset()` method sets TXLATECOL to value 0"]
impl crate::Resettable for TxlatecolSpec {}
