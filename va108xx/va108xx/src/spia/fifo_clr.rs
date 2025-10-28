#[doc = "Register `FIFO_CLR` writer"]
pub type W = crate::W<FifoClrSpec>;
#[doc = "Field `RXFIFO` writer - Clear Rx FIFO"]
pub type RxfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO` writer - Clear Tx FIFO"]
pub type TxfifoW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear Rx FIFO"]
    #[inline(always)]
    pub fn rxfifo(&mut self) -> RxfifoW<'_, FifoClrSpec> {
        RxfifoW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Tx FIFO"]
    #[inline(always)]
    pub fn txfifo(&mut self) -> TxfifoW<'_, FifoClrSpec> {
        TxfifoW::new(self, 1)
    }
}
#[doc = "Clear FIFO Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoClrSpec;
impl crate::RegisterSpec for FifoClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fifo_clr::W`](W) writer structure"]
impl crate::Writable for FifoClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO_CLR to value 0"]
impl crate::Resettable for FifoClrSpec {}
