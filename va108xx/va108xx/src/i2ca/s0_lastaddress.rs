#[doc = "Register `S0_LASTADDRESS` reader"]
pub type R = crate::R<S0LastaddressSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Slave I2C Last Address value\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_lastaddress::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0LastaddressSpec;
impl crate::RegisterSpec for S0LastaddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_lastaddress::R`](R) reader structure"]
impl crate::Readable for S0LastaddressSpec {}
#[doc = "`reset()` method sets S0_LASTADDRESS to value 0"]
impl crate::Resettable for S0LastaddressSpec {}
