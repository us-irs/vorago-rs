#[doc = "Register `S0_FIFO_CLR` writer"]
pub type W = crate::W<S0FifoClrSpec>;
#[doc = "Field `RXFIFO` writer - Clear Rx FIFO"]
pub type RxfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO` writer - Clear Tx FIFO"]
pub type TxfifoW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear Rx FIFO"]
    #[inline(always)]
    pub fn rxfifo(&mut self) -> RxfifoW<'_, S0FifoClrSpec> {
        RxfifoW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Tx FIFO"]
    #[inline(always)]
    pub fn txfifo(&mut self) -> TxfifoW<'_, S0FifoClrSpec> {
        TxfifoW::new(self, 1)
    }
}
#[doc = "Slave Clear FIFO Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_fifo_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0FifoClrSpec;
impl crate::RegisterSpec for S0FifoClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`s0_fifo_clr::W`](W) writer structure"]
impl crate::Writable for S0FifoClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S0_FIFO_CLR to value 0"]
impl crate::Resettable for S0FifoClrSpec {}
