#[doc = "Register `INT_RAM_SBE` reader"]
pub type R = crate::R<IntRamSbeSpec>;
#[doc = "Register `INT_RAM_SBE` writer"]
pub type W = crate::W<IntRamSbeSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Internal Memory RAM SBE Interrupt Redirect Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ram_sbe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ram_sbe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRamSbeSpec;
impl crate::RegisterSpec for IntRamSbeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ram_sbe::R`](R) reader structure"]
impl crate::Readable for IntRamSbeSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ram_sbe::W`](W) writer structure"]
impl crate::Writable for IntRamSbeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAM_SBE to value 0xffff_ffff"]
impl crate::Resettable for IntRamSbeSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
