#[doc = "Register `IRQ_EDGE` reader"]
pub type R = crate::R<IrqEdgeSpec>;
#[doc = "Register `IRQ_EDGE` writer"]
pub type W = crate::W<IrqEdgeSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt Both Edge Register (1:Both Edges, 0:Single Edge)\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_edge::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_edge::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEdgeSpec;
impl crate::RegisterSpec for IrqEdgeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_edge::R`](R) reader structure"]
impl crate::Readable for IrqEdgeSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_edge::W`](W) writer structure"]
impl crate::Writable for IrqEdgeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_EDGE to value 0"]
impl crate::Resettable for IrqEdgeSpec {}
