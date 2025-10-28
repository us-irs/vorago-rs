#[doc = "Register `RX256TO511OCT_GB` reader"]
pub type R = crate::R<Rx256to511octGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good and bad frames received with length between 256 and 511 bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rx256to511oct_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx256to511octGbSpec;
impl crate::RegisterSpec for Rx256to511octGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx256to511oct_gb::R`](R) reader structure"]
impl crate::Readable for Rx256to511octGbSpec {}
#[doc = "`reset()` method sets RX256TO511OCT_GB to value 0"]
impl crate::Resettable for Rx256to511octGbSpec {}
