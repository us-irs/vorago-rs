#[doc = "Register `DMA_CURR_RX_DESC` reader"]
pub type R = crate::R<DmaCurrRxDescSpec>;
#[doc = "Register `DMA_CURR_RX_DESC` writer"]
pub type W = crate::W<DmaCurrRxDescSpec>;
#[doc = "Field `CURRDESAPTR` reader - Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CurrdesaptrR = crate::FieldReader<u32>;
#[doc = "Field `CURRDESAPTR` writer - Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CurrdesaptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CurrdesaptrR {
        CurrdesaptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn currdesaptr(&mut self) -> CurrdesaptrW<'_, DmaCurrRxDescSpec> {
        CurrdesaptrW::new(self, 0)
    }
}
#[doc = "Contains the start address of the current Receive Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_curr_rx_desc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_curr_rx_desc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCurrRxDescSpec;
impl crate::RegisterSpec for DmaCurrRxDescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_curr_rx_desc::R`](R) reader structure"]
impl crate::Readable for DmaCurrRxDescSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_curr_rx_desc::W`](W) writer structure"]
impl crate::Writable for DmaCurrRxDescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CURR_RX_DESC to value 0"]
impl crate::Resettable for DmaCurrRxDescSpec {}
