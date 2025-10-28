#[doc = "Register `TX64OCT_GB` reader"]
pub type R = crate::R<Tx64octGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Good and bad Frames transmitted with length 64\n\nYou can [`read`](crate::Reg::read) this register and get [`tx64oct_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx64octGbSpec;
impl crate::RegisterSpec for Tx64octGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx64oct_gb::R`](R) reader structure"]
impl crate::Readable for Tx64octGbSpec {}
#[doc = "`reset()` method sets TX64OCT_GB to value 0"]
impl crate::Resettable for Tx64octGbSpec {}
