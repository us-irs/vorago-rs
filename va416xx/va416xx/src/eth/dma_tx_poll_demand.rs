#[doc = "Register `DMA_TX_POLL_DEMAND` reader"]
pub type R = crate::R<DmaTxPollDemandSpec>;
#[doc = "Register `DMA_TX_POLL_DEMAND` writer"]
pub type W = crate::W<DmaTxPollDemandSpec>;
#[doc = "Field `TPD` reader - Transmit Poll Demand (Read Only and Write Trigger)"]
pub type TpdR = crate::FieldReader<u32>;
#[doc = "Field `TPD` writer - Transmit Poll Demand (Read Only and Write Trigger)"]
pub type TpdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Poll Demand (Read Only and Write Trigger)"]
    #[inline(always)]
    pub fn tpd(&self) -> TpdR {
        TpdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Poll Demand (Read Only and Write Trigger)"]
    #[inline(always)]
    pub fn tpd(&mut self) -> TpdW<'_, DmaTxPollDemandSpec> {
        TpdW::new(self, 0)
    }
}
#[doc = "Used by the host to instruct the DMA to poll the transmit Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tx_poll_demand::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tx_poll_demand::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaTxPollDemandSpec;
impl crate::RegisterSpec for DmaTxPollDemandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_tx_poll_demand::R`](R) reader structure"]
impl crate::Readable for DmaTxPollDemandSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_tx_poll_demand::W`](W) writer structure"]
impl crate::Writable for DmaTxPollDemandSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_TX_POLL_DEMAND to value 0"]
impl crate::Resettable for DmaTxPollDemandSpec {}
