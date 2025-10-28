#[doc = "Register `SYND_SYND` reader"]
pub type R = crate::R<SyndSyndSpec>;
#[doc = "Register `SYND_SYND` writer"]
pub type W = crate::W<SyndSyndSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Synd Parity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_synd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synd_synd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndSyndSpec;
impl crate::RegisterSpec for SyndSyndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_synd::R`](R) reader structure"]
impl crate::Readable for SyndSyndSpec {}
#[doc = "`write(|w| ..)` method takes [`synd_synd::W`](W) writer structure"]
impl crate::Writable for SyndSyndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYND_SYND to value 0"]
impl crate::Resettable for SyndSyndSpec {}
