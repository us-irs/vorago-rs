#[doc = "Register `REFRESH_CONFIG` reader"]
pub type R = crate::R<RefreshConfigSpec>;
#[doc = "Register `REFRESH_CONFIG` writer"]
pub type W = crate::W<RefreshConfigSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Register Refresh Control\n\nYou can [`read`](crate::Reg::read) this register and get [`refresh_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refresh_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefreshConfigSpec;
impl crate::RegisterSpec for RefreshConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`refresh_config::R`](R) reader structure"]
impl crate::Readable for RefreshConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`refresh_config::W`](W) writer structure"]
impl crate::Writable for RefreshConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REFRESH_CONFIG to value 0"]
impl crate::Resettable for RefreshConfigSpec {}
