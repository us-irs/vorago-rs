#[doc = "Register `RX65TO127OCT_GB` reader"]
pub type R = crate::R<Rx65to127octGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good and bad frames received with length between 65 and 127 bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rx65to127oct_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx65to127octGbSpec;
impl crate::RegisterSpec for Rx65to127octGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx65to127oct_gb::R`](R) reader structure"]
impl crate::Readable for Rx65to127octGbSpec {}
#[doc = "`reset()` method sets RX65TO127OCT_GB to value 0"]
impl crate::Resettable for Rx65to127octGbSpec {}
