#[doc = "Register `RXOUTRANGETYPE` reader"]
pub type R = crate::R<RxoutrangetypeSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of frames received with length field not equal to the valid frame size\n\nYou can [`read`](crate::Reg::read) this register and get [`rxoutrangetype::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxoutrangetypeSpec;
impl crate::RegisterSpec for RxoutrangetypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxoutrangetype::R`](R) reader structure"]
impl crate::Readable for RxoutrangetypeSpec {}
#[doc = "`reset()` method sets RXOUTRANGETYPE to value 0"]
impl crate::Resettable for RxoutrangetypeSpec {}
