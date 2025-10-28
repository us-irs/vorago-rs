#[doc = "Register `CLKDIV0` reader"]
pub type R = crate::R<Clkdiv0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Clock divide value. 0 will disable the clock\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkdiv0Spec;
impl crate::RegisterSpec for Clkdiv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv0::R`](R) reader structure"]
impl crate::Readable for Clkdiv0Spec {}
#[doc = "`reset()` method sets CLKDIV0 to value 0"]
impl crate::Resettable for Clkdiv0Spec {}
