#[doc = "Register `RXCTRLFRAMES_G` reader"]
pub type R = crate::R<RxctrlframesGSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of received good control frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctrlframes_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxctrlframesGSpec;
impl crate::RegisterSpec for RxctrlframesGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctrlframes_g::R`](R) reader structure"]
impl crate::Readable for RxctrlframesGSpec {}
#[doc = "`reset()` method sets RXCTRLFRAMES_G to value 0"]
impl crate::Resettable for RxctrlframesGSpec {}
