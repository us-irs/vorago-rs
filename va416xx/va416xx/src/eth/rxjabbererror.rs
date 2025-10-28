#[doc = "Register `RXJABBERERROR` reader"]
pub type R = crate::R<RxjabbererrorSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of giant frames received with length greater than 1518 bytes and with CRC error\n\nYou can [`read`](crate::Reg::read) this register and get [`rxjabbererror::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxjabbererrorSpec;
impl crate::RegisterSpec for RxjabbererrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxjabbererror::R`](R) reader structure"]
impl crate::Readable for RxjabbererrorSpec {}
#[doc = "`reset()` method sets RXJABBERERROR to value 0"]
impl crate::Resettable for RxjabbererrorSpec {}
