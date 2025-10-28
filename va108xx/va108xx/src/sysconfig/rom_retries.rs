#[doc = "Register `ROM_RETRIES` reader"]
pub type R = crate::R<RomRetriesSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "ROM BOOT Retry count\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_retries::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomRetriesSpec;
impl crate::RegisterSpec for RomRetriesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_retries::R`](R) reader structure"]
impl crate::Readable for RomRetriesSpec {}
#[doc = "`reset()` method sets ROM_RETRIES to value 0"]
impl crate::Resettable for RomRetriesSpec {}
