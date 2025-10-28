#[doc = "Register `TX1024MAXOCT_GB` reader"]
pub type R = crate::R<Tx1024maxoctGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Good and bad Frames transmitted with length 1024 to max bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`tx1024maxoct_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx1024maxoctGbSpec;
impl crate::RegisterSpec for Tx1024maxoctGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx1024maxoct_gb::R`](R) reader structure"]
impl crate::Readable for Tx1024maxoctGbSpec {}
#[doc = "`reset()` method sets TX1024MAXOCT_GB to value 0"]
impl crate::Resettable for Tx1024maxoctGbSpec {}
