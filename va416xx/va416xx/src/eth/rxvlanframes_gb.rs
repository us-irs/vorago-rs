#[doc = "Register `RXVLANFRAMES_GB` reader"]
pub type R = crate::R<RxvlanframesGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good and bad VLAN frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`rxvlanframes_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxvlanframesGbSpec;
impl crate::RegisterSpec for RxvlanframesGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxvlanframes_gb::R`](R) reader structure"]
impl crate::Readable for RxvlanframesGbSpec {}
#[doc = "`reset()` method sets RXVLANFRAMES_GB to value 0"]
impl crate::Resettable for RxvlanframesGbSpec {}
