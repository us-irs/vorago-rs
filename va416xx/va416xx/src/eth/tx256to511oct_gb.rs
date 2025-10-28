#[doc = "Register `TX256TO511OCT_GB` reader"]
pub type R = crate::R<Tx256to511octGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Good and bad Frames transmitted with length 256 to 511\n\nYou can [`read`](crate::Reg::read) this register and get [`tx256to511oct_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx256to511octGbSpec;
impl crate::RegisterSpec for Tx256to511octGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx256to511oct_gb::R`](R) reader structure"]
impl crate::Readable for Tx256to511octGbSpec {}
#[doc = "`reset()` method sets TX256TO511OCT_GB to value 0"]
impl crate::Resettable for Tx256to511octGbSpec {}
