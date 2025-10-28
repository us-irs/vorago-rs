#[doc = "Register `RX128TO255OCT_GB` reader"]
pub type R = crate::R<Rx128to255octGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good and bad frames received with length between 128 and 255 bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rx128to255oct_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx128to255octGbSpec;
impl crate::RegisterSpec for Rx128to255octGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx128to255oct_gb::R`](R) reader structure"]
impl crate::Readable for Rx128to255octGbSpec {}
#[doc = "`reset()` method sets RX128TO255OCT_GB to value 0"]
impl crate::Resettable for Rx128to255octGbSpec {}
