#[doc = "Register `PERID` reader"]
pub type R = crate::R<PeridSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeridSpec;
impl crate::RegisterSpec for PeridSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perid::R`](R) reader structure"]
impl crate::Readable for PeridSpec {}
#[doc = "`reset()` method sets PERID to value 0x0011_07e1"]
impl crate::Resettable for PeridSpec {
    const RESET_VALUE: u32 = 0x0011_07e1;
}
