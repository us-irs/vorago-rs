#[doc = "Register `DATAIN` reader"]
pub type R = crate::R<DatainSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Data In Register\n\nYou can [`read`](crate::Reg::read) this register and get [`datain::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatainSpec;
impl crate::RegisterSpec for DatainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datain::R`](R) reader structure"]
impl crate::Readable for DatainSpec {}
#[doc = "`reset()` method sets DATAIN to value 0"]
impl crate::Resettable for DatainSpec {}
