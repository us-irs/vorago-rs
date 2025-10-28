#[doc = "Register `TXSTATUS` reader"]
pub type R = crate::R<TxstatusSpec>;
#[doc = "Field `WRRDY` reader - Write Fifo NOT Full"]
pub type WrrdyR = crate::BitReader;
#[doc = "Field `WRBUSY` reader - Write Fifo Full"]
pub type WrbusyR = crate::BitReader;
#[doc = "Field `TXBUSY` reader - TX Busy Transmitting"]
pub type TxbusyR = crate::BitReader;
#[doc = "Field `WRLOST` reader - Write Data Lost (Fifo Overflow)"]
pub type WrlostR = crate::BitReader;
#[doc = "Field `TXCTSN` reader - TX CTSn Input Value"]
pub type TxctsnR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write Fifo NOT Full"]
    #[inline(always)]
    pub fn wrrdy(&self) -> WrrdyR {
        WrrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Fifo Full"]
    #[inline(always)]
    pub fn wrbusy(&self) -> WrbusyR {
        WrbusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX Busy Transmitting"]
    #[inline(always)]
    pub fn txbusy(&self) -> TxbusyR {
        TxbusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write Data Lost (Fifo Overflow)"]
    #[inline(always)]
    pub fn wrlost(&self) -> WrlostR {
        WrlostR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - TX CTSn Input Value"]
    #[inline(always)]
    pub fn txctsn(&self) -> TxctsnR {
        TxctsnR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxstatusSpec;
impl crate::RegisterSpec for TxstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txstatus::R`](R) reader structure"]
impl crate::Readable for TxstatusSpec {}
#[doc = "`reset()` method sets TXSTATUS to value 0"]
impl crate::Resettable for TxstatusSpec {}
