#[doc = "Register `DATAMASKBYTE[%s]` reader"]
pub type R = crate::R<DatamaskbyteSpec>;
#[doc = "Register `DATAMASKBYTE[%s]` writer"]
pub type W = crate::W<DatamaskbyteSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data Out Register by Byte\n\nYou can [`read`](crate::Reg::read) this register and get [`datamaskbyte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datamaskbyte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatamaskbyteSpec;
impl crate::RegisterSpec for DatamaskbyteSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`datamaskbyte::R`](R) reader structure"]
impl crate::Readable for DatamaskbyteSpec {}
#[doc = "`write(|w| ..)` method takes [`datamaskbyte::W`](W) writer structure"]
impl crate::Writable for DatamaskbyteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAMASKBYTE[%s] to value 0"]
impl crate::Resettable for DatamaskbyteSpec {}
