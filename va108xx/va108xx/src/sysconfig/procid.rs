#[doc = "Register `PROCID` reader"]
pub type R = crate::R<ProcidSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Processor ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`procid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProcidSpec;
impl crate::RegisterSpec for ProcidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`procid::R`](R) reader structure"]
impl crate::Readable for ProcidSpec {}
#[doc = "`reset()` method sets PROCID to value 0x0400_17e3"]
impl crate::Resettable for ProcidSpec {
    const RESET_VALUE: u32 = 0x0400_17e3;
}
