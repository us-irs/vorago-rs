#[doc = "Register `EF_ID` reader"]
pub type R = crate::R<EfIdSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "EFuse ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfIdSpec;
impl crate::RegisterSpec for EfIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_id::R`](R) reader structure"]
impl crate::Readable for EfIdSpec {}
#[doc = "`reset()` method sets EF_ID to value 0"]
impl crate::Resettable for EfIdSpec {}
