#[doc = "Register `IOCONFIG_CLKDIV%s` reader"]
pub type R = crate::R<IoconfigClkdivSpec>;
#[doc = "Register `IOCONFIG_CLKDIV%s` writer"]
pub type W = crate::W<IoconfigClkdivSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "IO Configuration Clock Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ioconfig_clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioconfig_clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoconfigClkdivSpec;
impl crate::RegisterSpec for IoconfigClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioconfig_clkdiv::R`](R) reader structure"]
impl crate::Readable for IoconfigClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`ioconfig_clkdiv::W`](W) writer structure"]
impl crate::Writable for IoconfigClkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOCONFIG_CLKDIV%s to value 0"]
impl crate::Resettable for IoconfigClkdivSpec {}
