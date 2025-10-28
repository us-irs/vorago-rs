#[doc = "Register `DMA_RX_DESC_LIST_ADDR` reader"]
pub type R = crate::R<DmaRxDescListAddrSpec>;
#[doc = "Register `DMA_RX_DESC_LIST_ADDR` writer"]
pub type W = crate::W<DmaRxDescListAddrSpec>;
#[doc = "Field `RDESLA` reader - Start of Receive List"]
pub type RdeslaR = crate::FieldReader<u32>;
#[doc = "Field `RDESLA` writer - Start of Receive List"]
pub type RdeslaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla(&self) -> RdeslaR {
        RdeslaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla(&mut self) -> RdeslaW<'_, DmaRxDescListAddrSpec> {
        RdeslaW::new(self, 0)
    }
}
#[doc = "Points the DMA to the start of the Receive Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rx_desc_list_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rx_desc_list_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaRxDescListAddrSpec;
impl crate::RegisterSpec for DmaRxDescListAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_rx_desc_list_addr::R`](R) reader structure"]
impl crate::Readable for DmaRxDescListAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_rx_desc_list_addr::W`](W) writer structure"]
impl crate::Writable for DmaRxDescListAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_RX_DESC_LIST_ADDR to value 0"]
impl crate::Resettable for DmaRxDescListAddrSpec {}
