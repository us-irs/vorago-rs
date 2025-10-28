#[doc = "Register `EF_ID1` reader"]
pub type R = crate::R<EfId1Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "EFuse ID1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_id1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfId1Spec;
impl crate::RegisterSpec for EfId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_id1::R`](R) reader structure"]
impl crate::Readable for EfId1Spec {}
#[doc = "`reset()` method sets EF_ID1 to value 0"]
impl crate::Resettable for EfId1Spec {}
