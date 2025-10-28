#[doc = "Register `S0_MAXWORDS` reader"]
pub type R = crate::R<S0MaxwordsSpec>;
#[doc = "Register `S0_MAXWORDS` writer"]
pub type W = crate::W<S0MaxwordsSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Slave MaxWords Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_maxwords::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_maxwords::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0MaxwordsSpec;
impl crate::RegisterSpec for S0MaxwordsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_maxwords::R`](R) reader structure"]
impl crate::Readable for S0MaxwordsSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_maxwords::W`](W) writer structure"]
impl crate::Writable for S0MaxwordsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S0_MAXWORDS to value 0"]
impl crate::Resettable for S0MaxwordsSpec {}
