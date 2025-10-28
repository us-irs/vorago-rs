#[doc = "Register `IRQ_OUT4` reader"]
pub type R = crate::R<IrqOut4Spec>;
#[doc = "Field `IRQ_OUT4` reader - IRQ_OUT\\[159:128\\]"]
pub type IrqOut4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ_OUT\\[159:128\\]"]
    #[inline(always)]
    pub fn irq_out4(&self) -> IrqOut4R {
        IrqOut4R::new(self.bits)
    }
}
#[doc = "DEBUG IRQ_OUT\\[159:128\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_out4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqOut4Spec;
impl crate::RegisterSpec for IrqOut4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_out4::R`](R) reader structure"]
impl crate::Readable for IrqOut4Spec {}
#[doc = "`reset()` method sets IRQ_OUT4 to value 0"]
impl crate::Resettable for IrqOut4Spec {}
