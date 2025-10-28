#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `I2C_IDLE` reader - I2C bus is Idle"]
pub type I2cIdleR = crate::BitReader;
#[doc = "Field `IDLE` reader - Controller is Idle"]
pub type IdleR = crate::BitReader;
#[doc = "Field `WAITING` reader - Controller is Waiting"]
pub type WaitingR = crate::BitReader;
#[doc = "Field `STALLED` reader - Controller is Stalled"]
pub type StalledR = crate::BitReader;
#[doc = "Field `ARBLOST` reader - I2C Arbitration was lost"]
pub type ArblostR = crate::BitReader;
#[doc = "Field `NACKADDR` reader - I2C Address was not Acknowledged"]
pub type NackaddrR = crate::BitReader;
#[doc = "Field `NACKDATA` reader - I2C Data was not Acknowledged"]
pub type NackdataR = crate::BitReader;
#[doc = "Field `RXNEMPTY` reader - RX FIFO is Not Empty"]
pub type RxnemptyR = crate::BitReader;
#[doc = "Field `RXFULL` reader - RX FIFO is Full"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXTRIGGER` reader - RX FIFO Above Trigger Level"]
pub type RxtriggerR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - TX FIFO is Empty"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `TXNFULL` reader - TX FIFO is Full"]
pub type TxnfullR = crate::BitReader;
#[doc = "Field `TXTRIGGER` reader - TX FIFO Below Trigger Level"]
pub type TxtriggerR = crate::BitReader;
#[doc = "Field `RAW_SDA` reader - I2C Raw SDA value"]
pub type RawSdaR = crate::BitReader;
#[doc = "Field `RAW_SCL` reader - I2C Raw SCL value"]
pub type RawSclR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I2C bus is Idle"]
    #[inline(always)]
    pub fn i2c_idle(&self) -> I2cIdleR {
        I2cIdleR::new((self.bits & 1) != 0)
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
#[doc = "I2C Controller Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
