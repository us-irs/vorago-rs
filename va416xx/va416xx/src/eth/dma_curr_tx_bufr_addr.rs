#[doc = "Register `DMA_CURR_TX_BUFR_ADDR` reader"]
pub type R = crate::R<DmaCurrTxBufrAddrSpec>;
#[doc = "Register `DMA_CURR_TX_BUFR_ADDR` writer"]
pub type W = crate::W<DmaCurrTxBufrAddrSpec>;
#[doc = "Field `CURTBUFAPTR` reader - Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CurtbufaptrR = crate::FieldReader<u32>;
#[doc = "Field `CURTBUFAPTR` writer - Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CurtbufaptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CurtbufaptrR {
        CurtbufaptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn curtbufaptr(&mut self) -> CurtbufaptrW<'_, DmaCurrTxBufrAddrSpec> {
        CurtbufaptrW::new(self, 0)
    }
}
#[doc = "Contains the start address of the current Receive Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_curr_tx_bufr_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_curr_tx_bufr_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCurrTxBufrAddrSpec;
impl crate::RegisterSpec for DmaCurrTxBufrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_curr_tx_bufr_addr::R`](R) reader structure"]
impl crate::Readable for DmaCurrTxBufrAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_curr_tx_bufr_addr::W`](W) writer structure"]
impl crate::Writable for DmaCurrTxBufrAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CURR_TX_BUFR_ADDR to value 0"]
impl crate::Resettable for DmaCurrTxBufrAddrSpec {}
