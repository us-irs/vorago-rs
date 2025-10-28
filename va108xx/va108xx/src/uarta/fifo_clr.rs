#[doc = "Register `FIFO_CLR` writer"]
pub type W = crate::W<FifoClrSpec>;
#[doc = "Field `RXSTS` writer - Clear Rx Status"]
pub type RxstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTS` writer - Clear Tx Status"]
pub type TxstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO` writer - Clear Rx FIFO"]
pub type RxfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO` writer - Clear Tx FIFO"]
pub type TxfifoW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear Rx Status"]
    #[inline(always)]
    pub fn rxsts(&mut self) -> RxstsW<'_, FifoClrSpec> {
        RxstsW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Tx Status"]
    #[inline(always)]
    pub fn txsts(&mut self) -> TxstsW<'_, FifoClrSpec> {
        TxstsW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Rx FIFO"]
    #[inline(always)]
    pub fn rxfifo(&mut self) -> RxfifoW<'_, FifoClrSpec> {
        RxfifoW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Tx FIFO"]
    #[inline(always)]
    pub fn txfifo(&mut self) -> TxfifoW<'_, FifoClrSpec> {
        TxfifoW::new(self, 3)
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
