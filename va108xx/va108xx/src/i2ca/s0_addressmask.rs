#[doc = "Register `S0_ADDRESSMASK` reader"]
pub type R = crate::R<S0AddressmaskSpec>;
#[doc = "Register `S0_ADDRESSMASK` writer"]
pub type W = crate::W<S0AddressmaskSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Slave I2C Address Mask value\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_addressmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_addressmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0AddressmaskSpec;
impl crate::RegisterSpec for S0AddressmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_addressmask::R`](R) reader structure"]
impl crate::Readable for S0AddressmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_addressmask::W`](W) writer structure"]
impl crate::Writable for S0AddressmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S0_ADDRESSMASK to value 0"]
impl crate::Resettable for S0AddressmaskSpec {}
