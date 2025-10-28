#[doc = "Register `CNT_VALUE` reader"]
pub type R = crate::R<CntValueSpec>;
#[doc = "Register `CNT_VALUE` writer"]
pub type W = crate::W<CntValueSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The current value of the counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntValueSpec;
impl crate::RegisterSpec for CntValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt_value::R`](R) reader structure"]
impl crate::Readable for CntValueSpec {}
#[doc = "`write(|w| ..)` method takes [`cnt_value::W`](W) writer structure"]
impl crate::Writable for CntValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNT_VALUE to value 0"]
impl crate::Resettable for CntValueSpec {}
