#[doc = "Register `S0_DATA` reader"]
pub type R = crate::R<S0DataSpec>;
#[doc = "Register `S0_DATA` writer"]
pub type W = crate::W<S0DataSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Slave Data Input/Output\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0DataSpec;
impl crate::RegisterSpec for S0DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_data::R`](R) reader structure"]
impl crate::Readable for S0DataSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_data::W`](W) writer structure"]
impl crate::Writable for S0DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S0_DATA to value 0"]
impl crate::Resettable for S0DataSpec {}
