#[doc = "Register `IRQ_OUT5` reader"]
pub type R = crate::R<IrqOut5Spec>;
#[doc = "Field `IRQ_OUT5` reader - IRQ_OUT\\[179:160\\]"]
pub type IrqOut5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - IRQ_OUT\\[179:160\\]"]
    #[inline(always)]
    pub fn irq_out5(&self) -> IrqOut5R {
        IrqOut5R::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "DEBUG IRQ_OUT\\[179:160\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_out5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqOut5Spec;
impl crate::RegisterSpec for IrqOut5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_out5::R`](R) reader structure"]
impl crate::Readable for IrqOut5Spec {}
#[doc = "`reset()` method sets IRQ_OUT5 to value 0"]
impl crate::Resettable for IrqOut5Spec {}
