#[doc = "Register `DATAMASK` reader"]
pub type R = crate::R<DatamaskSpec>;
#[doc = "Register `DATAMASK` writer"]
pub type W = crate::W<DatamaskSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`datamask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datamask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatamaskSpec;
impl crate::RegisterSpec for DatamaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datamask::R`](R) reader structure"]
impl crate::Readable for DatamaskSpec {}
#[doc = "`write(|w| ..)` method takes [`datamask::W`](W) writer structure"]
impl crate::Writable for DatamaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAMASK to value 0"]
impl crate::Resettable for DatamaskSpec {}
