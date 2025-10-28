#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `I2CIDLE` reader - I2C bus is idle"]
pub type I2cidleR = crate::BitReader;
#[doc = "Field `I2CIDLE` writer - I2C bus is idle"]
pub type I2cidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE` reader - I2C controller is Idle"]
pub type IdleR = crate::BitReader;
#[doc = "Field `IDLE` writer - I2C controller is Idle"]
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
#[doc = "Field `RXNEMPTY` reader - RX FIFO is Not Empty"]
pub type RxnemptyR = crate::BitReader;
#[doc = "Field `RXNEMPTY` writer - RX FIFO is Not Empty"]
pub type RxnemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - RX FIFO is Full"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXFULL` writer - RX FIFO is Full"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTRIGGER` reader - RX FIFO Above Trigger Level"]
pub type RxtriggerR = crate::BitReader;
#[doc = "Field `RXTRIGGER` writer - RX FIFO Above Trigger Level"]
pub type RxtriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` reader - TX FIFO is Empty"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `TXEMPTY` writer - TX FIFO is Empty"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXNFULL` reader - TX FIFO is Full"]
pub type TxnfullR = crate::BitReader;
#[doc = "Field `TXNFULL` writer - TX FIFO is Full"]
pub type TxnfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIGGER` reader - TX FIFO Below Trigger Level"]
pub type TxtriggerR = crate::BitReader;
#[doc = "Field `TXTRIGGER` writer - TX FIFO Below Trigger Level"]
pub type TxtriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAW_SDA` reader - I2C Raw SDA value"]
pub type RawSdaR = crate::BitReader;
#[doc = "Field `RAW_SDA` writer - I2C Raw SDA value"]
pub type RawSdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAW_SCL` reader - I2C Raw SCL value"]
pub type RawSclR = crate::BitReader;
#[doc = "Field `RAW_SCL` writer - I2C Raw SCL value"]
pub type RawSclW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C bus is idle"]
    #[inline(always)]
    pub fn i2cidle(&self) -> I2cidleR {
        I2cidleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C controller is Idle"]
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
    #[doc = "Bit 8 - RX FIFO is Not Empty"]
    #[inline(always)]
    pub fn rxnempty(&self) -> RxnemptyR {
        RxnemptyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RX FIFO is Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - RX FIFO Above Trigger Level"]
    #[inline(always)]
    pub fn rxtrigger(&self) -> RxtriggerR {
        RxtriggerR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TX FIFO is Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX FIFO is Full"]
    #[inline(always)]
    pub fn txnfull(&self) -> TxnfullR {
        TxnfullR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - TX FIFO Below Trigger Level"]
    #[inline(always)]
    pub fn txtrigger(&self) -> TxtriggerR {
        TxtriggerR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C Raw SDA value"]
    #[inline(always)]
    pub fn raw_sda(&self) -> RawSdaR {
        RawSdaR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - I2C Raw SCL value"]
    #[inline(always)]
    pub fn raw_scl(&self) -> RawSclR {
        RawSclR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C bus is idle"]
    #[inline(always)]
    pub fn i2cidle(&mut self) -> I2cidleW<'_, StatusSpec> {
        I2cidleW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C controller is Idle"]
    #[inline(always)]
    pub fn idle(&mut self) -> IdleW<'_, StatusSpec> {
        IdleW::new(self, 1)
    }
    #[doc = "Bit 2 - Controller is Waiting"]
    #[inline(always)]
    pub fn waiting(&mut self) -> WaitingW<'_, StatusSpec> {
        WaitingW::new(self, 2)
    }
    #[doc = "Bit 3 - Controller is Stalled"]
    #[inline(always)]
    pub fn stalled(&mut self) -> StalledW<'_, StatusSpec> {
        StalledW::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Arbitration was lost"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ArblostW<'_, StatusSpec> {
        ArblostW::new(self, 4)
    }
    #[doc = "Bit 5 - I2C Address was not Acknowledged"]
    #[inline(always)]
    pub fn nackaddr(&mut self) -> NackaddrW<'_, StatusSpec> {
        NackaddrW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Data was not Acknowledged"]
    #[inline(always)]
    pub fn nackdata(&mut self) -> NackdataW<'_, StatusSpec> {
        NackdataW::new(self, 6)
    }
    #[doc = "Bit 8 - RX FIFO is Not Empty"]
    #[inline(always)]
    pub fn rxnempty(&mut self) -> RxnemptyW<'_, StatusSpec> {
        RxnemptyW::new(self, 8)
    }
    #[doc = "Bit 9 - RX FIFO is Full"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, StatusSpec> {
        RxfullW::new(self, 9)
    }
    #[doc = "Bit 11 - RX FIFO Above Trigger Level"]
    #[inline(always)]
    pub fn rxtrigger(&mut self) -> RxtriggerW<'_, StatusSpec> {
        RxtriggerW::new(self, 11)
    }
    #[doc = "Bit 12 - TX FIFO is Empty"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<'_, StatusSpec> {
        TxemptyW::new(self, 12)
    }
    #[doc = "Bit 13 - TX FIFO is Full"]
    #[inline(always)]
    pub fn txnfull(&mut self) -> TxnfullW<'_, StatusSpec> {
        TxnfullW::new(self, 13)
    }
    #[doc = "Bit 15 - TX FIFO Below Trigger Level"]
    #[inline(always)]
    pub fn txtrigger(&mut self) -> TxtriggerW<'_, StatusSpec> {
        TxtriggerW::new(self, 15)
    }
    #[doc = "Bit 30 - I2C Raw SDA value"]
    #[inline(always)]
    pub fn raw_sda(&mut self) -> RawSdaW<'_, StatusSpec> {
        RawSdaW::new(self, 30)
    }
    #[doc = "Bit 31 - I2C Raw SCL value"]
    #[inline(always)]
    pub fn raw_scl(&mut self) -> RawSclW<'_, StatusSpec> {
        RawSclW::new(self, 31)
    }
}
#[doc = "I2C Controller Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
