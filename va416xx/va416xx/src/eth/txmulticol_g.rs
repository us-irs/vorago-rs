#[doc = "Register `TXMULTICOL_G` reader"]
pub type R = crate::R<TxmulticolGSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of successfully transmitted frames after multiple collisions\n\nYou can [`read`](crate::Reg::read) this register and get [`txmulticol_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxmulticolGSpec;
impl crate::RegisterSpec for TxmulticolGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmulticol_g::R`](R) reader structure"]
impl crate::Readable for TxmulticolGSpec {}
#[doc = "`reset()` method sets TXMULTICOL_G to value 0"]
impl crate::Resettable for TxmulticolGSpec {}
