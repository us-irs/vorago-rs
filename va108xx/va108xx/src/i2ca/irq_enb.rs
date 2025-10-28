#[doc = "Register `IRQ_ENB` reader"]
pub type R = crate::R<IrqEnbSpec>;
#[doc = "Register `IRQ_ENB` writer"]
pub type W = crate::W<IrqEnbSpec>;
#[doc = "Field `I2CIDLE` reader - I2C Bus is Idle"]
pub type I2cidleR = crate::BitReader;
#[doc = "Field `I2CIDLE` writer - I2C Bus is Idle"]
pub type I2cidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE` reader - Controller is Idle"]
pub type IdleR = crate::BitReader;
#[doc = "Field `IDLE` writer - Controller is Idle"]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITING` reader - Controller is Waiting"]
pub type WaitingR = crate::BitReader;
#[doc = "Field `WAITING` writer - Controller is Waiting"]
pub type WaitingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLED` reader - Controller is Stalled"]
pub type StalledR = crate::BitReader;
#[doc = "Field `STALLED` writer - Controller is Stalled"]
pub type StalledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` reader - I2C Arbitration was lost"]
pub type ArblostR = crate::BitReader;
#[doc = "Field `ARBLOST` writer - I2C Arbitration was lost"]
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKADDR` reader - I2C Address was not Acknowledged"]
pub type NackaddrR = crate::BitReader;
#[doc = "Field `NACKADDR` writer - I2C Address was not Acknowledged"]
pub type NackaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKDATA` reader - I2C Data was not Acknowledged"]
pub type NackdataR = crate::BitReader;
#[doc = "Field `NACKDATA` writer - I2C Data was not Acknowledged"]
pub type NackdataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKLOTO` reader - I2C Clock Low Timeout"]
pub type ClklotoR = crate::BitReader;
#[doc = "Field `CLKLOTO` writer - I2C Clock Low Timeout"]
pub type ClklotoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOVERFLOW` reader - TX FIFO Overflowed"]
pub type TxoverflowR = crate::BitReader;
#[doc = "Field `TXOVERFLOW` writer - TX FIFO Overflowed"]
pub type TxoverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERFLOW` reader - TX FIFO Overflowed"]
pub type RxoverflowR = crate::BitReader;
#[doc = "Field `RXOVERFLOW` writer - TX FIFO Overflowed"]
pub type RxoverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXREADY` reader - TX FIFO Ready"]
pub type TxreadyR = crate::BitReader;
#[doc = "Field `TXREADY` writer - TX FIFO Ready"]
pub type TxreadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXREADY` reader - RX FIFO Ready"]
pub type RxreadyR = crate::BitReader;
#[doc = "Field `RXREADY` writer - RX FIFO Ready"]
pub type RxreadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` reader - TX FIFO Empty"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `TXEMPTY` writer - TX FIFO Empty"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - RX FIFO Full"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXFULL` writer - RX FIFO Full"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Bus is Idle"]
    #[inline(always)]
    pub fn i2cidle(&self) -> I2cidleR {
        I2cidleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controller is Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controller is Waiting"]
    #[inline(always)]
    pub fn waiting(&self) -> WaitingR {
        WaitingR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controller is Stalled"]
    #[inline(always)]
    pub fn stalled(&self) -> StalledR {
        StalledR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Arbitration was lost"]
    #[inline(always)]
    pub fn arblost(&self) -> ArblostR {
        ArblostR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Address was not Acknowledged"]
    #[inline(always)]
    pub fn nackaddr(&self) -> NackaddrR {
        NackaddrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Data was not Acknowledged"]
    #[inline(always)]
    pub fn nackdata(&self) -> NackdataR {
        NackdataR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout"]
    #[inline(always)]
    pub fn clkloto(&self) -> ClklotoR {
        ClklotoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - TX FIFO Overflowed"]
    #[inline(always)]
    pub fn txoverflow(&self) -> TxoverflowR {
        TxoverflowR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TX FIFO Overflowed"]
    #[inline(always)]
    pub fn rxoverflow(&self) -> RxoverflowR {
        RxoverflowR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TX FIFO Ready"]
    #[inline(always)]
    pub fn txready(&self) -> TxreadyR {
        TxreadyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RX FIFO Ready"]
    #[inline(always)]
    pub fn rxready(&self) -> RxreadyR {
        RxreadyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TX FIFO Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Bus is Idle"]
    #[inline(always)]
    pub fn i2cidle(&mut self) -> I2cidleW<'_, IrqEnbSpec> {
        I2cidleW::new(self, 0)
    }
    #[doc = "Bit 1 - Controller is Idle"]
    #[inline(always)]
    pub fn idle(&mut self) -> IdleW<'_, IrqEnbSpec> {
        IdleW::new(self, 1)
    }
    #[doc = "Bit 2 - Controller is Waiting"]
    #[inline(always)]
    pub fn waiting(&mut self) -> WaitingW<'_, IrqEnbSpec> {
        WaitingW::new(self, 2)
    }
    #[doc = "Bit 3 - Controller is Stalled"]
    #[inline(always)]
    pub fn stalled(&mut self) -> StalledW<'_, IrqEnbSpec> {
        StalledW::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Arbitration was lost"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ArblostW<'_, IrqEnbSpec> {
        ArblostW::new(self, 4)
    }
    #[doc = "Bit 5 - I2C Address was not Acknowledged"]
    #[inline(always)]
    pub fn nackaddr(&mut self) -> NackaddrW<'_, IrqEnbSpec> {
        NackaddrW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Data was not Acknowledged"]
    #[inline(always)]
    pub fn nackdata(&mut self) -> NackdataW<'_, IrqEnbSpec> {
        NackdataW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout"]
    #[inline(always)]
    pub fn clkloto(&mut self) -> ClklotoW<'_, IrqEnbSpec> {
        ClklotoW::new(self, 7)
    }
    #[doc = "Bit 10 - TX FIFO Overflowed"]
    #[inline(always)]
    pub fn txoverflow(&mut self) -> TxoverflowW<'_, IrqEnbSpec> {
        TxoverflowW::new(self, 10)
    }
    #[doc = "Bit 11 - TX FIFO Overflowed"]
    #[inline(always)]
    pub fn rxoverflow(&mut self) -> RxoverflowW<'_, IrqEnbSpec> {
        RxoverflowW::new(self, 11)
    }
    #[doc = "Bit 12 - TX FIFO Ready"]
    #[inline(always)]
    pub fn txready(&mut self) -> TxreadyW<'_, IrqEnbSpec> {
        TxreadyW::new(self, 12)
    }
    #[doc = "Bit 13 - RX FIFO Ready"]
    #[inline(always)]
    pub fn rxready(&mut self) -> RxreadyW<'_, IrqEnbSpec> {
        RxreadyW::new(self, 13)
    }
    #[doc = "Bit 14 - TX FIFO Empty"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<'_, IrqEnbSpec> {
        TxemptyW::new(self, 14)
    }
    #[doc = "Bit 15 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, IrqEnbSpec> {
        RxfullW::new(self, 15)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEnbSpec;
impl crate::RegisterSpec for IrqEnbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_enb::R`](R) reader structure"]
impl crate::Readable for IrqEnbSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_enb::W`](W) writer structure"]
impl crate::Writable for IrqEnbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_ENB to value 0"]
impl crate::Resettable for IrqEnbSpec {}
