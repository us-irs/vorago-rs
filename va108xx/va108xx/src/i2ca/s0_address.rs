#[doc = "Register `S0_ADDRESS` reader"]
pub type R = crate::R<S0AddressSpec>;
#[doc = "Register `S0_ADDRESS` writer"]
pub type W = crate::W<S0AddressSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Slave I2C Address Value\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0AddressSpec;
impl crate::RegisterSpec for S0AddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_address::R`](R) reader structure"]
impl crate::Readable for S0AddressSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_address::W`](W) writer structure"]
impl crate::Writable for S0AddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S0_ADDRESS to value 0"]
impl crate::Resettable for S0AddressSpec {}
