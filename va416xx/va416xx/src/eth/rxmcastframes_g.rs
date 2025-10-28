#[doc = "Register `RXMCASTFRAMES_G` reader"]
pub type R = crate::R<RxmcastframesGSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good multicast frames received\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmcastframes_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxmcastframesGSpec;
impl crate::RegisterSpec for RxmcastframesGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmcastframes_g::R`](R) reader structure"]
impl crate::Readable for RxmcastframesGSpec {}
#[doc = "`reset()` method sets RXMCASTFRAMES_G to value 0"]
impl crate::Resettable for RxmcastframesGSpec {}
