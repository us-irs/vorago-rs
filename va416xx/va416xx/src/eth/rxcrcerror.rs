#[doc = "Register `RXCRCERROR` reader"]
pub type R = crate::R<RxcrcerrorSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of frames received with CRC error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcrcerror::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxcrcerrorSpec;
impl crate::RegisterSpec for RxcrcerrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxcrcerror::R`](R) reader structure"]
impl crate::Readable for RxcrcerrorSpec {}
#[doc = "`reset()` method sets RXCRCERROR to value 0"]
impl crate::Resettable for RxcrcerrorSpec {}
