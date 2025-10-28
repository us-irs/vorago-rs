#[doc = "Register `DATAOUT` writer"]
pub type W = crate::W<DataoutSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DataoutSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Data Out Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataout::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataoutSpec;
impl crate::RegisterSpec for DataoutSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dataout::W`](W) writer structure"]
impl crate::Writable for DataoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAOUT to value 0"]
impl crate::Resettable for DataoutSpec {}
