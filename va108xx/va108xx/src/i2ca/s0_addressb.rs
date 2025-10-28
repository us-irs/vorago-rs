#[doc = "Register `S0_ADDRESSB` reader"]
pub type R = crate::R<S0AddressbSpec>;
#[doc = "Register `S0_ADDRESSB` writer"]
pub type W = crate::W<S0AddressbSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Slave I2C Address B Value\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_addressb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_addressb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0AddressbSpec;
impl crate::RegisterSpec for S0AddressbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_addressb::R`](R) reader structure"]
impl crate::Readable for S0AddressbSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_addressb::W`](W) writer structure"]
impl crate::Writable for S0AddressbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S0_ADDRESSB to value 0"]
impl crate::Resettable for S0AddressbSpec {}
