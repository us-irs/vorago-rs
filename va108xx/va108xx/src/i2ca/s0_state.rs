#[doc = "Register `S0_STATE` reader"]
pub type R = crate::R<S0StateSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Internal STATE of I2C Slave Controller\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0StateSpec;
impl crate::RegisterSpec for S0StateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_state::R`](R) reader structure"]
impl crate::Readable for S0StateSpec {}
#[doc = "`reset()` method sets S0_STATE to value 0"]
impl crate::Resettable for S0StateSpec {}
