#[doc = "Register `RXCOUNT` reader"]
pub type R = crate::R<RxcountSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "RX Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxcountSpec;
impl crate::RegisterSpec for RxcountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxcount::R`](R) reader structure"]
impl crate::Readable for RxcountSpec {}
#[doc = "`reset()` method sets RXCOUNT to value 0"]
impl crate::Resettable for RxcountSpec {}
