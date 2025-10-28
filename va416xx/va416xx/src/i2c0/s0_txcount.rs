#[doc = "Register `S0_TXCOUNT` reader"]
pub type R = crate::R<S0TxcountSpec>;
#[doc = "Field `VALUE` reader - Count value"]
pub type ValueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Count value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "Slave TX Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_txcount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0TxcountSpec;
impl crate::RegisterSpec for S0TxcountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_txcount::R`](R) reader structure"]
impl crate::Readable for S0TxcountSpec {}
#[doc = "`reset()` method sets S0_TXCOUNT to value 0"]
impl crate::Resettable for S0TxcountSpec {}
