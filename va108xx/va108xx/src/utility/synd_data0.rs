#[doc = "Register `SYND_DATA0` reader"]
pub type R = crate::R<SyndData0Spec>;
#[doc = "Register `SYND_DATA0` writer"]
pub type W = crate::W<SyndData0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Synd Data 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synd_data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndData0Spec;
impl crate::RegisterSpec for SyndData0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_data0::R`](R) reader structure"]
impl crate::Readable for SyndData0Spec {}
#[doc = "`write(|w| ..)` method takes [`synd_data0::W`](W) writer structure"]
impl crate::Writable for SyndData0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYND_DATA0 to value 0"]
impl crate::Resettable for SyndData0Spec {}
