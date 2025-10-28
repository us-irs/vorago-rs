#[doc = "Register `IRQ_RAW` reader"]
pub type R = crate::R<IrqRawSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Raw Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqRawSpec;
impl crate::RegisterSpec for IrqRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_raw::R`](R) reader structure"]
impl crate::Readable for IrqRawSpec {}
#[doc = "`reset()` method sets IRQ_RAW to value 0"]
impl crate::Resettable for IrqRawSpec {}
