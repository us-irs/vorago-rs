#[doc = "Register `IRQ_OUT0` reader"]
pub type R = crate::R<IrqOut0Spec>;
#[doc = "Field `IRQ_OUT0` reader - IRQ_OUT\\[31:0\\]"]
pub type IrqOut0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ_OUT\\[31:0\\]"]
    #[inline(always)]
    pub fn irq_out0(&self) -> IrqOut0R {
        IrqOut0R::new(self.bits)
    }
}
#[doc = "DEBUG IRQ_OUT\\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_out0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqOut0Spec;
impl crate::RegisterSpec for IrqOut0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_out0::R`](R) reader structure"]
impl crate::Readable for IrqOut0Spec {}
#[doc = "`reset()` method sets IRQ_OUT0 to value 0"]
impl crate::Resettable for IrqOut0Spec {}
