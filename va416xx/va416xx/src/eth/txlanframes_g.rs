#[doc = "Register `TXLANFRAMES_G` reader"]
pub type R = crate::R<TxlanframesGSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good VLAN frames transmitted\n\nYou can [`read`](crate::Reg::read) this register and get [`txlanframes_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxlanframesGSpec;
impl crate::RegisterSpec for TxlanframesGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txlanframes_g::R`](R) reader structure"]
impl crate::Readable for TxlanframesGSpec {}
#[doc = "`reset()` method sets TXLANFRAMES_G to value 0"]
impl crate::Resettable for TxlanframesGSpec {}
