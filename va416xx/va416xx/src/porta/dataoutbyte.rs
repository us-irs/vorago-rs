#[doc = "Register `DATAOUTBYTE[%s]` writer"]
pub type W = crate::W<DataoutbyteSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DataoutbyteSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Data Out Register by Byte\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataoutbyte::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataoutbyteSpec;
impl crate::RegisterSpec for DataoutbyteSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`dataoutbyte::W`](W) writer structure"]
impl crate::Writable for DataoutbyteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAOUTBYTE[%s] to value 0"]
impl crate::Resettable for DataoutbyteSpec {}
