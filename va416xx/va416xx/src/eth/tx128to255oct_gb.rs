#[doc = "Register `TX128TO255OCT_GB` reader"]
pub type R = crate::R<Tx128to255octGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Good and bad Frames transmitted with length 128 to 255\n\nYou can [`read`](crate::Reg::read) this register and get [`tx128to255oct_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx128to255octGbSpec;
impl crate::RegisterSpec for Tx128to255octGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx128to255oct_gb::R`](R) reader structure"]
impl crate::Readable for Tx128to255octGbSpec {}
#[doc = "`reset()` method sets TX128TO255OCT_GB to value 0"]
impl crate::Resettable for Tx128to255octGbSpec {}
