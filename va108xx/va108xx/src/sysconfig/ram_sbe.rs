#[doc = "Register `RAM_SBE` reader"]
pub type R = crate::R<RamSbeSpec>;
#[doc = "Register `RAM_SBE` writer"]
pub type W = crate::W<RamSbeSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Count of RAM EDAC Single Bit Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_sbe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_sbe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamSbeSpec;
impl crate::RegisterSpec for RamSbeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram_sbe::R`](R) reader structure"]
impl crate::Readable for RamSbeSpec {}
#[doc = "`write(|w| ..)` method takes [`ram_sbe::W`](W) writer structure"]
impl crate::Writable for RamSbeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAM_SBE to value 0"]
impl crate::Resettable for RamSbeSpec {}
