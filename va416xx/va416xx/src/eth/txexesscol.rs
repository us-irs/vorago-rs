#[doc = "Register `TXEXESSCOL` reader"]
pub type R = crate::R<TxexesscolSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of aborted frames because of excessive collision errors\n\nYou can [`read`](crate::Reg::read) this register and get [`txexesscol::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxexesscolSpec;
impl crate::RegisterSpec for TxexesscolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txexesscol::R`](R) reader structure"]
impl crate::Readable for TxexesscolSpec {}
#[doc = "`reset()` method sets TXEXESSCOL to value 0"]
impl crate::Resettable for TxexesscolSpec {}
