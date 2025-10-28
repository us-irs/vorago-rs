#[doc = "Register `RXSTATUS` reader"]
pub type R = crate::R<RxstatusSpec>;
#[doc = "Field `RDAVL` reader - Read Data Available"]
pub type RdavlR = crate::BitReader;
#[doc = "Field `RDNFULL` reader - Read Fifo NOT Full"]
pub type RdnfullR = crate::BitReader;
#[doc = "Field `RXBUSY` reader - RX Busy Receiving"]
pub type RxbusyR = crate::BitReader;
#[doc = "Field `RXTO` reader - RX Receive Timeout"]
pub type RxtoR = crate::BitReader;
#[doc = "Field `RXOVR` reader - Read Fifo Overflow"]
pub type RxovrR = crate::BitReader;
#[doc = "Field `RXFRM` reader - RX Framing Error"]
pub type RxfrmR = crate::BitReader;
#[doc = "Field `RXPAR` reader - RX Parity Error"]
pub type RxparR = crate::BitReader;
#[doc = "Field `RXBRK` reader - RX Break Error"]
pub type RxbrkR = crate::BitReader;
#[doc = "Field `RXBUSYBRK` reader - RX Busy Receiving Break"]
pub type RxbusybrkR = crate::BitReader;
#[doc = "Field `RXADDR9` reader - Address Match for 9 bit mode"]
pub type Rxaddr9R = crate::BitReader;
#[doc = "Field `RXRTSN` reader - RX RTSn Output Value"]
pub type RxrtsnR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read Data Available"]
    #[inline(always)]
    pub fn rdavl(&self) -> RdavlR {
        RdavlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Fifo NOT Full"]
    #[inline(always)]
    pub fn rdnfull(&self) -> RdnfullR {
        RdnfullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Busy Receiving"]
    #[inline(always)]
    pub fn rxbusy(&self) -> RxbusyR {
        RxbusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX Receive Timeout"]
    #[inline(always)]
    pub fn rxto(&self) -> RxtoR {
        RxtoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read Fifo Overflow"]
    #[inline(always)]
    pub fn rxovr(&self) -> RxovrR {
        RxovrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX Framing Error"]
    #[inline(always)]
    pub fn rxfrm(&self) -> RxfrmR {
        RxfrmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX Parity Error"]
    #[inline(always)]
    pub fn rxpar(&self) -> RxparR {
        RxparR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Break Error"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RxbrkR {
        RxbrkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX Busy Receiving Break"]
    #[inline(always)]
    pub fn rxbusybrk(&self) -> RxbusybrkR {
        RxbusybrkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Address Match for 9 bit mode"]
    #[inline(always)]
    pub fn rxaddr9(&self) -> Rxaddr9R {
        Rxaddr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - RX RTSn Output Value"]
    #[inline(always)]
    pub fn rxrtsn(&self) -> RxrtsnR {
        RxrtsnR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxstatusSpec;
impl crate::RegisterSpec for RxstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxstatus::R`](R) reader structure"]
impl crate::Readable for RxstatusSpec {}
#[doc = "`reset()` method sets RXSTATUS to value 0"]
impl crate::Resettable for RxstatusSpec {}
