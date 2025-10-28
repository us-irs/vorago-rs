#[doc = "Register `RST_VALUE` reader"]
pub type R = crate::R<RstValueSpec>;
#[doc = "Register `RST_VALUE` writer"]
pub type W = crate::W<RstValueSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The value that counter start from after reaching 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstValueSpec;
impl crate::RegisterSpec for RstValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_value::R`](R) reader structure"]
impl crate::Readable for RstValueSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_value::W`](W) writer structure"]
impl crate::Writable for RstValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RST_VALUE to value 0"]
impl crate::Resettable for RstValueSpec {}
