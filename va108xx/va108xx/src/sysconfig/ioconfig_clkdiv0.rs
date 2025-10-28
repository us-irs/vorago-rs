#[doc = "Register `IOCONFIG_CLKDIV0` reader"]
pub type R = crate::R<IoconfigClkdiv0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "IO Configuration Clock Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ioconfig_clkdiv0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoconfigClkdiv0Spec;
impl crate::RegisterSpec for IoconfigClkdiv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioconfig_clkdiv0::R`](R) reader structure"]
impl crate::Readable for IoconfigClkdiv0Spec {}
#[doc = "`reset()` method sets IOCONFIG_CLKDIV0 to value 0"]
impl crate::Resettable for IoconfigClkdiv0Spec {}
