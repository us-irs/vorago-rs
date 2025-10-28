#[doc = "Register `S0_RXCOUNT` reader"]
pub type R = crate::R<S0RxcountSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Slave RX Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_rxcount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0RxcountSpec;
impl crate::RegisterSpec for S0RxcountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_rxcount::R`](R) reader structure"]
impl crate::Readable for S0RxcountSpec {}
#[doc = "`reset()` method sets S0_RXCOUNT to value 0"]
impl crate::Resettable for S0RxcountSpec {}
