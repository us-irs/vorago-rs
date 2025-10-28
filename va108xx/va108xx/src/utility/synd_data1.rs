#[doc = "Register `SYND_DATA1` reader"]
pub type R = crate::R<SyndData1Spec>;
#[doc = "Register `SYND_DATA1` writer"]
pub type W = crate::W<SyndData1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Synd Data 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_data1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synd_data1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndData1Spec;
impl crate::RegisterSpec for SyndData1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_data1::R`](R) reader structure"]
impl crate::Readable for SyndData1Spec {}
#[doc = "`write(|w| ..)` method takes [`synd_data1::W`](W) writer structure"]
impl crate::Writable for SyndData1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYND_DATA1 to value 0"]
impl crate::Resettable for SyndData1Spec {}
