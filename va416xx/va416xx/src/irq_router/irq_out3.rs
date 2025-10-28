#[doc = "Register `IRQ_OUT3` reader"]
pub type R = crate::R<IrqOut3Spec>;
#[doc = "Field `IRQ_OUT3` reader - IRQ_OUT\\[127:96\\]"]
pub type IrqOut3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ_OUT\\[127:96\\]"]
    #[inline(always)]
    pub fn irq_out3(&self) -> IrqOut3R {
        IrqOut3R::new(self.bits)
    }
}
#[doc = "DEBUG IRQ_OUT\\[127:96\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_out3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqOut3Spec;
impl crate::RegisterSpec for IrqOut3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_out3::R`](R) reader structure"]
impl crate::Readable for IrqOut3Spec {}
#[doc = "`reset()` method sets IRQ_OUT3 to value 0"]
impl crate::Resettable for IrqOut3Spec {}
