#[doc = "Register `IRQ_END` reader"]
pub type R = crate::R<IrqEndSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Masked Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_end::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEndSpec;
impl crate::RegisterSpec for IrqEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_end::R`](R) reader structure"]
impl crate::Readable for IrqEndSpec {}
#[doc = "`reset()` method sets IRQ_END to value 0"]
impl crate::Resettable for IrqEndSpec {}
