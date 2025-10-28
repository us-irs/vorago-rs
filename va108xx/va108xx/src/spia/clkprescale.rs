#[doc = "Register `CLKPRESCALE` reader"]
pub type R = crate::R<ClkprescaleSpec>;
#[doc = "Register `CLKPRESCALE` writer"]
pub type W = crate::W<ClkprescaleSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Clock Pre Scale divide value\n\nYou can [`read`](crate::Reg::read) this register and get [`clkprescale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkprescale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkprescaleSpec;
impl crate::RegisterSpec for ClkprescaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkprescale::R`](R) reader structure"]
impl crate::Readable for ClkprescaleSpec {}
#[doc = "`write(|w| ..)` method takes [`clkprescale::W`](W) writer structure"]
impl crate::Writable for ClkprescaleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKPRESCALE to value 0"]
impl crate::Resettable for ClkprescaleSpec {}
