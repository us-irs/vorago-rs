#[doc = "Register `IRQ_OUT1` reader"]
pub type R = crate::R<IrqOut1Spec>;
#[doc = "Field `IRQ_OUT1` reader - IRQ_OUT\\[63:32\\]"]
pub type IrqOut1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ_OUT\\[63:32\\]"]
    #[inline(always)]
    pub fn irq_out1(&self) -> IrqOut1R {
        IrqOut1R::new(self.bits)
    }
}
#[doc = "DEBUG IRQ_OUT\\[63:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_out1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqOut1Spec;
impl crate::RegisterSpec for IrqOut1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_out1::R`](R) reader structure"]
impl crate::Readable for IrqOut1Spec {}
#[doc = "`reset()` method sets IRQ_OUT1 to value 0"]
impl crate::Resettable for IrqOut1Spec {}
