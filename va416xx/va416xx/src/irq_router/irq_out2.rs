#[doc = "Register `IRQ_OUT2` reader"]
pub type R = crate::R<IrqOut2Spec>;
#[doc = "Field `IRQ_OUT2` reader - IRQ_OUT\\[95:64\\]"]
pub type IrqOut2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ_OUT\\[95:64\\]"]
    #[inline(always)]
    pub fn irq_out2(&self) -> IrqOut2R {
        IrqOut2R::new(self.bits)
    }
}
#[doc = "DEBUG IRQ_OUT\\[95:64\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_out2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqOut2Spec;
impl crate::RegisterSpec for IrqOut2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_out2::R`](R) reader structure"]
impl crate::Readable for IrqOut2Spec {}
#[doc = "`reset()` method sets IRQ_OUT2 to value 0"]
impl crate::Resettable for IrqOut2Spec {}
