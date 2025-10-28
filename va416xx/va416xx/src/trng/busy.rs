#[doc = "Register `BUSY` reader"]
pub type R = crate::R<BusySpec>;
#[doc = "Field `BUSY` reader - Reflects the status of the rng_busy signal"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reflects the status of the rng_busy signal"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
}
#[doc = "Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusySpec;
impl crate::RegisterSpec for BusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busy::R`](R) reader structure"]
impl crate::Readable for BusySpec {}
#[doc = "`reset()` method sets BUSY to value 0"]
impl crate::Resettable for BusySpec {}
