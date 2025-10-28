#[doc = "Register `IRQ_SEN` reader"]
pub type R = crate::R<IrqSenSpec>;
#[doc = "Register `IRQ_SEN` writer"]
pub type W = crate::W<IrqSenSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt Sense Register (1:Level Sensitive, 0:Edge Sensitive)\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_sen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_sen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqSenSpec;
impl crate::RegisterSpec for IrqSenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_sen::R`](R) reader structure"]
impl crate::Readable for IrqSenSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_sen::W`](W) writer structure"]
impl crate::Writable for IrqSenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_SEN to value 0"]
impl crate::Resettable for IrqSenSpec {}
