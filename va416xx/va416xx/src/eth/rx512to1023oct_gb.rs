#[doc = "Register `RX512TO1023OCT_GB` reader"]
pub type R = crate::R<Rx512to1023octGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good and bad frames received with length between 512 and 1023 bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rx512to1023oct_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx512to1023octGbSpec;
impl crate::RegisterSpec for Rx512to1023octGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx512to1023oct_gb::R`](R) reader structure"]
impl crate::Readable for Rx512to1023octGbSpec {}
#[doc = "`reset()` method sets RX512TO1023OCT_GB to value 0"]
impl crate::Resettable for Rx512to1023octGbSpec {}
