#[doc = "Register `DMA_RX_INTR_WDOG_TIMER` reader"]
pub type R = crate::R<DmaRxIntrWdogTimerSpec>;
#[doc = "Register `DMA_RX_INTR_WDOG_TIMER` writer"]
pub type W = crate::W<DmaRxIntrWdogTimerSpec>;
#[doc = "Field `RIWT` reader - These bits indicate the number of system clock cycles x 256 for which the watchdog timer is set."]
pub type RiwtR = crate::FieldReader;
#[doc = "Field `RIWT` writer - These bits indicate the number of system clock cycles x 256 for which the watchdog timer is set."]
pub type RiwtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - These bits indicate the number of system clock cycles x 256 for which the watchdog timer is set."]
    #[inline(always)]
    pub fn riwt(&self) -> RiwtR {
        RiwtR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits indicate the number of system clock cycles x 256 for which the watchdog timer is set."]
    #[inline(always)]
    pub fn riwt(&mut self) -> RiwtW<'_, DmaRxIntrWdogTimerSpec> {
        RiwtW::new(self, 0)
    }
}
#[doc = "Watchdog timeout for Receive Interrupt from DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rx_intr_wdog_timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rx_intr_wdog_timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaRxIntrWdogTimerSpec;
impl crate::RegisterSpec for DmaRxIntrWdogTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_rx_intr_wdog_timer::R`](R) reader structure"]
impl crate::Readable for DmaRxIntrWdogTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_rx_intr_wdog_timer::W`](W) writer structure"]
impl crate::Writable for DmaRxIntrWdogTimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_RX_INTR_WDOG_TIMER to value 0"]
impl crate::Resettable for DmaRxIntrWdogTimerSpec {}
