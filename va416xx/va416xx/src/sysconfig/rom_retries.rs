#[doc = "Register `ROM_RETRIES` reader"]
pub type R = crate::R<RomRetriesSpec>;
#[doc = "Field `COUNT` reader - Count of ROM block Retries"]
pub type CountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Count of ROM block Retries"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xff) as u8)
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
