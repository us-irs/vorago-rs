#[doc = "Register `TMCONFIG` reader"]
pub type R = crate::R<TmconfigSpec>;
#[doc = "Register `TMCONFIG` writer"]
pub type W = crate::W<TmconfigSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timing Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmconfigSpec;
impl crate::RegisterSpec for TmconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmconfig::R`](R) reader structure"]
impl crate::Readable for TmconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`tmconfig::W`](W) writer structure"]
impl crate::Writable for TmconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMCONFIG to value 0"]
impl crate::Resettable for TmconfigSpec {}
