#[doc = "Register `S0_TXCOUNT` reader"]
pub type R = crate::R<S0TxcountSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
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
