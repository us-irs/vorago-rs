#[doc = "Register `TX65TO127OCT_GB` reader"]
pub type R = crate::R<Tx65to127octGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Good and bad Frames transmitted with length 65 to 127\n\nYou can [`read`](crate::Reg::read) this register and get [`tx65to127oct_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx65to127octGbSpec;
impl crate::RegisterSpec for Tx65to127octGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx65to127oct_gb::R`](R) reader structure"]
impl crate::Readable for Tx65to127octGbSpec {}
#[doc = "`reset()` method sets TX65TO127OCT_GB to value 0"]
impl crate::Resettable for Tx65to127octGbSpec {}
