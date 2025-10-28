#[doc = "Register `S0_IRQ_ENB` reader"]
pub type R = crate::R<S0IrqEnbSpec>;
#[doc = "Register `S0_IRQ_ENB` writer"]
pub type W = crate::W<S0IrqEnbSpec>;
#[doc = "Field `COMPLETED` reader - Controller Complted a Transaction"]
pub type CompletedR = crate::BitReader;
#[doc = "Field `COMPLETED` writer - Controller Complted a Transaction"]
pub type CompletedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE` reader - Controller is Idle"]
pub type IdleR = crate::BitReader;
#[doc = "Field `IDLE` writer - Controller is Idle"]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITING` reader - Controller is Waiting"]
pub type WaitingR = crate::BitReader;
#[doc = "Field `WAITING` writer - Controller is Waiting"]
pub type WaitingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTALLED` reader - Controller is Tx Stalled"]
pub type TxstalledR = crate::BitReader;
#[doc = "Field `TXSTALLED` writer - Controller is Tx Stalled"]
pub type TxstalledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALLED` reader - Controller is Rx Stalled"]
pub type RxstalledR = crate::BitReader;
#[doc = "Field `RXSTALLED` writer - Controller is Rx Stalled"]
pub type RxstalledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESSMATCH` reader - I2C Address Match"]
pub type AddressmatchR = crate::BitReader;
#[doc = "Field `ADDRESSMATCH` writer - I2C Address Match"]
pub type AddressmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKDATA` reader - I2C Data was not Acknowledged"]
pub type NackdataR = crate::BitReader;
#[doc = "Field `NACKDATA` writer - I2C Data was not Acknowledged"]
pub type NackdataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDATAFIRST` reader - Pending Data is first Byte following Address"]
pub type RxdatafirstR = crate::BitReader;
#[doc = "Field `RXDATAFIRST` writer - Pending Data is first Byte following Address"]
pub type RxdatafirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_START` reader - I2C Start Condition"]
pub type I2cStartR = crate::BitReader;
#[doc = "Field `I2C_START` writer - I2C Start Condition"]
pub type I2cStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_STOP` reader - I2C Stop Condition"]
pub type I2cStopR = crate::BitReader;
#[doc = "Field `I2C_STOP` writer - I2C Stop Condition"]
pub type I2cStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERFLOW` reader - TX FIFO Underflowed"]
pub type TxunderflowR = crate::BitReader;
#[doc = "Field `TXUNDERFLOW` writer - TX FIFO Underflowed"]
pub type TxunderflowW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - Controller Complted a Transaction"]
    #[inline(always)]
    pub fn completed(&self) -> CompletedR {
        CompletedR::new((self.bits & 1) != 0)
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
    #[doc = "Bit 3 - Controller is Tx Stalled"]
    #[inline(always)]
    pub fn txstalled(&self) -> TxstalledR {
        TxstalledR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Controller is Rx Stalled"]
    #[inline(always)]
    pub fn rxstalled(&self) -> RxstalledR {
        RxstalledR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Address Match"]
    #[inline(always)]
    pub fn addressmatch(&self) -> AddressmatchR {
        AddressmatchR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Data was not Acknowledged"]
    #[inline(always)]
    pub fn nackdata(&self) -> NackdataR {
        NackdataR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending Data is first Byte following Address"]
    #[inline(always)]
    pub fn rxdatafirst(&self) -> RxdatafirstR {
        RxdatafirstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Start Condition"]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2cStartR {
        I2cStartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Stop Condition"]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2cStopR {
        I2cStopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TX FIFO Underflowed"]
    #[inline(always)]
    pub fn txunderflow(&self) -> TxunderflowR {
        TxunderflowR::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bit 0 - Controller Complted a Transaction"]
    #[inline(always)]
    pub fn completed(&mut self) -> CompletedW<'_, S0IrqEnbSpec> {
        CompletedW::new(self, 0)
    }
    #[doc = "Bit 1 - Controller is Idle"]
    #[inline(always)]
    pub fn idle(&mut self) -> IdleW<'_, S0IrqEnbSpec> {
        IdleW::new(self, 1)
    }
    #[doc = "Bit 2 - Controller is Waiting"]
    #[inline(always)]
    pub fn waiting(&mut self) -> WaitingW<'_, S0IrqEnbSpec> {
        WaitingW::new(self, 2)
    }
    #[doc = "Bit 3 - Controller is Tx Stalled"]
    #[inline(always)]
    pub fn txstalled(&mut self) -> TxstalledW<'_, S0IrqEnbSpec> {
        TxstalledW::new(self, 3)
    }
    #[doc = "Bit 4 - Controller is Rx Stalled"]
    #[inline(always)]
    pub fn rxstalled(&mut self) -> RxstalledW<'_, S0IrqEnbSpec> {
        RxstalledW::new(self, 4)
    }
    #[doc = "Bit 5 - I2C Address Match"]
    #[inline(always)]
    pub fn addressmatch(&mut self) -> AddressmatchW<'_, S0IrqEnbSpec> {
        AddressmatchW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Data was not Acknowledged"]
    #[inline(always)]
    pub fn nackdata(&mut self) -> NackdataW<'_, S0IrqEnbSpec> {
        NackdataW::new(self, 6)
    }
    #[doc = "Bit 7 - Pending Data is first Byte following Address"]
    #[inline(always)]
    pub fn rxdatafirst(&mut self) -> RxdatafirstW<'_, S0IrqEnbSpec> {
        RxdatafirstW::new(self, 7)
    }
    #[doc = "Bit 8 - I2C Start Condition"]
    #[inline(always)]
    pub fn i2c_start(&mut self) -> I2cStartW<'_, S0IrqEnbSpec> {
        I2cStartW::new(self, 8)
    }
    #[doc = "Bit 9 - I2C Stop Condition"]
    #[inline(always)]
    pub fn i2c_stop(&mut self) -> I2cStopW<'_, S0IrqEnbSpec> {
        I2cStopW::new(self, 9)
    }
    #[doc = "Bit 10 - TX FIFO Underflowed"]
    #[inline(always)]
    pub fn txunderflow(&mut self) -> TxunderflowW<'_, S0IrqEnbSpec> {
        TxunderflowW::new(self, 10)
    }
    #[doc = "Bit 11 - TX FIFO Overflowed"]
    #[inline(always)]
    pub fn rxoverflow(&mut self) -> RxoverflowW<'_, S0IrqEnbSpec> {
        RxoverflowW::new(self, 11)
    }
    #[doc = "Bit 12 - TX FIFO Ready"]
    #[inline(always)]
    pub fn txready(&mut self) -> TxreadyW<'_, S0IrqEnbSpec> {
        TxreadyW::new(self, 12)
    }
    #[doc = "Bit 13 - RX FIFO Ready"]
    #[inline(always)]
    pub fn rxready(&mut self) -> RxreadyW<'_, S0IrqEnbSpec> {
        RxreadyW::new(self, 13)
    }
    #[doc = "Bit 14 - TX FIFO Empty"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<'_, S0IrqEnbSpec> {
        TxemptyW::new(self, 14)
    }
    #[doc = "Bit 15 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, S0IrqEnbSpec> {
        RxfullW::new(self, 15)
    }
}
#[doc = "Slave Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_irq_enb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_irq_enb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0IrqEnbSpec;
impl crate::RegisterSpec for S0IrqEnbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_irq_enb::R`](R) reader structure"]
impl crate::Readable for S0IrqEnbSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_irq_enb::W`](W) writer structure"]
impl crate::Writable for S0IrqEnbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S0_IRQ_ENB to value 0"]
impl crate::Resettable for S0IrqEnbSpec {}
