#[doc = "Register `TXSINGLECOL_G` reader"]
pub type R = crate::R<TxsinglecolGSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of successfully transmitted frames after a single collision\n\nYou can [`read`](crate::Reg::read) this register and get [`txsinglecol_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxsinglecolGSpec;
impl crate::RegisterSpec for TxsinglecolGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txsinglecol_g::R`](R) reader structure"]
impl crate::Readable for TxsinglecolGSpec {}
#[doc = "`reset()` method sets TXSINGLECOL_G to value 0"]
impl crate::Resettable for TxsinglecolGSpec {}
