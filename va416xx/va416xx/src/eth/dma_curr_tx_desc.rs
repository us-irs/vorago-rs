#[doc = "Register `DMA_CURR_TX_DESC` reader"]
pub type R = crate::R<DmaCurrTxDescSpec>;
#[doc = "Register `DMA_CURR_TX_DESC` writer"]
pub type W = crate::W<DmaCurrTxDescSpec>;
#[doc = "Field `CURTDESAPTR` reader - Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CurtdesaptrR = crate::FieldReader<u32>;
#[doc = "Field `CURTDESAPTR` writer - Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CurtdesaptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CurtdesaptrR {
        CurtdesaptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn curtdesaptr(&mut self) -> CurtdesaptrW<'_, DmaCurrTxDescSpec> {
        CurtdesaptrW::new(self, 0)
    }
}
#[doc = "Contains the start address of the current Transmit Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_curr_tx_desc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_curr_tx_desc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCurrTxDescSpec;
impl crate::RegisterSpec for DmaCurrTxDescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_curr_tx_desc::R`](R) reader structure"]
impl crate::Readable for DmaCurrTxDescSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_curr_tx_desc::W`](W) writer structure"]
impl crate::Writable for DmaCurrTxDescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CURR_TX_DESC to value 0"]
impl crate::Resettable for DmaCurrTxDescSpec {}
