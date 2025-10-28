#[doc = "Register `CLKDIV1` reader"]
pub type R = crate::R<Clkdiv1Spec>;
#[doc = "Register `CLKDIV1` writer"]
pub type W = crate::W<Clkdiv1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Clock divide value. 0 will disable the clock\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkdiv1Spec;
impl crate::RegisterSpec for Clkdiv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv1::R`](R) reader structure"]
impl crate::Readable for Clkdiv1Spec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv1::W`](W) writer structure"]
impl crate::Writable for Clkdiv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKDIV1 to value 0"]
impl crate::Resettable for Clkdiv1Spec {}
