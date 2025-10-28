#[doc = "Register `IRQ_EVT` reader"]
pub type R = crate::R<IrqEvtSpec>;
#[doc = "Register `IRQ_EVT` writer"]
pub type W = crate::W<IrqEvtSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt Event Register (1:HighLevel/L->H Edge, 0:LowLevel/H->L Edge)\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_evt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_evt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEvtSpec;
impl crate::RegisterSpec for IrqEvtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_evt::R`](R) reader structure"]
impl crate::Readable for IrqEvtSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_evt::W`](W) writer structure"]
impl crate::Writable for IrqEvtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_EVT to value 0"]
impl crate::Resettable for IrqEvtSpec {}
