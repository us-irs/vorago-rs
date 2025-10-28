#[doc = "Register `TXCOUNT` reader"]
pub type R = crate::R<TxcountSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "TX Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txcount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxcountSpec;
impl crate::RegisterSpec for TxcountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txcount::R`](R) reader structure"]
impl crate::Readable for TxcountSpec {}
#[doc = "`reset()` method sets TXCOUNT to value 0"]
impl crate::Resettable for TxcountSpec {}
