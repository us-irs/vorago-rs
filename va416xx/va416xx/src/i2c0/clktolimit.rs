#[doc = "Register `CLKTOLIMIT` reader"]
pub type R = crate::R<ClktolimitSpec>;
#[doc = "Register `CLKTOLIMIT` writer"]
pub type W = crate::W<ClktolimitSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Clock Low Timeout Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clktolimit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clktolimit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClktolimitSpec;
impl crate::RegisterSpec for ClktolimitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clktolimit::R`](R) reader structure"]
impl crate::Readable for ClktolimitSpec {}
#[doc = "`write(|w| ..)` method takes [`clktolimit::W`](W) writer structure"]
impl crate::Writable for ClktolimitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKTOLIMIT to value 0"]
impl crate::Resettable for ClktolimitSpec {}
