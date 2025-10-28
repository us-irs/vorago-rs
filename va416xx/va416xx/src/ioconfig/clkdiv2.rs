#[doc = "Register `CLKDIV2` reader"]
pub type R = crate::R<Clkdiv2Spec>;
#[doc = "Register `CLKDIV2` writer"]
pub type W = crate::W<Clkdiv2Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Clock divide value. 0 will disable the clock\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkdiv2Spec;
impl crate::RegisterSpec for Clkdiv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv2::R`](R) reader structure"]
impl crate::Readable for Clkdiv2Spec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv2::W`](W) writer structure"]
impl crate::Writable for Clkdiv2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKDIV2 to value 0"]
impl crate::Resettable for Clkdiv2Spec {}
