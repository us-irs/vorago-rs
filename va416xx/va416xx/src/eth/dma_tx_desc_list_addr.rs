#[doc = "Register `DMA_TX_DESC_LIST_ADDR` reader"]
pub type R = crate::R<DmaTxDescListAddrSpec>;
#[doc = "Register `DMA_TX_DESC_LIST_ADDR` writer"]
pub type W = crate::W<DmaTxDescListAddrSpec>;
#[doc = "Field `TDESLA` reader - Start of Transmit List"]
pub type TdeslaR = crate::FieldReader<u32>;
#[doc = "Field `TDESLA` writer - Start of Transmit List"]
pub type TdeslaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla(&self) -> TdeslaR {
        TdeslaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla(&mut self) -> TdeslaW<'_, DmaTxDescListAddrSpec> {
        TdeslaW::new(self, 0)
    }
}
#[doc = "Points the DMA to the start of the Transmit Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tx_desc_list_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tx_desc_list_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaTxDescListAddrSpec;
impl crate::RegisterSpec for DmaTxDescListAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_tx_desc_list_addr::R`](R) reader structure"]
impl crate::Readable for DmaTxDescListAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_tx_desc_list_addr::W`](W) writer structure"]
impl crate::Writable for DmaTxDescListAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_TX_DESC_LIST_ADDR to value 0"]
impl crate::Resettable for DmaTxDescListAddrSpec {}
