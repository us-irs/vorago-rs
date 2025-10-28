#[doc = "Register `RX64OCTETS_GB` reader"]
pub type R = crate::R<Rx64octetsGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good and bad frames received with length 64 bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rx64octets_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx64octetsGbSpec;
impl crate::RegisterSpec for Rx64octetsGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx64octets_gb::R`](R) reader structure"]
impl crate::Readable for Rx64octetsGbSpec {}
#[doc = "`reset()` method sets RX64OCTETS_GB to value 0"]
impl crate::Resettable for Rx64octetsGbSpec {}
