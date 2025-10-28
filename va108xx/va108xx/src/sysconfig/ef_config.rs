#[doc = "Register `EF_CONFIG` reader"]
pub type R = crate::R<EfConfigSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "EFuse Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_config::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfConfigSpec;
impl crate::RegisterSpec for EfConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_config::R`](R) reader structure"]
impl crate::Readable for EfConfigSpec {}
#[doc = "`reset()` method sets EF_CONFIG to value 0"]
impl crate::Resettable for EfConfigSpec {}
