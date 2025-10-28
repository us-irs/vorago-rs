#[doc = "Register `RX1024MAXOCT_GB` reader"]
pub type R = crate::R<Rx1024maxoctGbSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of good and bad frames received with length between 1024 and max size bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`rx1024maxoct_gb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx1024maxoctGbSpec;
impl crate::RegisterSpec for Rx1024maxoctGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx1024maxoct_gb::R`](R) reader structure"]
impl crate::Readable for Rx1024maxoctGbSpec {}
#[doc = "`reset()` method sets RX1024MAXOCT_GB to value 0"]
impl crate::Resettable for Rx1024maxoctGbSpec {}
