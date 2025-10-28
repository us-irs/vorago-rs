#[doc = "Register `NMI` reader"]
pub type R = crate::R<NmiSpec>;
#[doc = "Field `ACTIVE` reader - Active"]
pub type ActiveR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Active"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
}
#[doc = "NMI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiSpec;
impl crate::RegisterSpec for NmiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmi::R`](R) reader structure"]
impl crate::Readable for NmiSpec {}
#[doc = "`reset()` method sets NMI to value 0"]
impl crate::Resettable for NmiSpec {}
