#[doc = "Register `SYND_CHECK_32_44_DATA` reader"]
pub type R = crate::R<SyndCheck32_44DataSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "EDAC Decode Data\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_check_32_44_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndCheck32_44DataSpec;
impl crate::RegisterSpec for SyndCheck32_44DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_check_32_44_data::R`](R) reader structure"]
impl crate::Readable for SyndCheck32_44DataSpec {}
#[doc = "`reset()` method sets SYND_CHECK_32_44_DATA to value 0"]
impl crate::Resettable for SyndCheck32_44DataSpec {}
