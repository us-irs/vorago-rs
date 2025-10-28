#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `TFE` reader - Transmit FIFO empty"]
pub type TfeR = crate::BitReader;
#[doc = "Field `TNF` reader - Transmit FIFO not full"]
pub type TnfR = crate::BitReader;
#[doc = "Field `RNE` reader - Receive FIFO not empty"]
pub type RneR = crate::BitReader;
#[doc = "Field `RFF` reader - Receive FIFO Full"]
pub type RffR = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `RXDATAFIRST` reader - Pending Data is first Byte in BLOCKMODE"]
pub type RxdatafirstR = crate::BitReader;
#[doc = "Field `RXTRIGGER` reader - RX FIFO Above Trigger Level"]
pub type RxtriggerR = crate::BitReader;
#[doc = "Field `TXTRIGGER` reader - TX FIFO Below Trigger Level"]
pub type TxtriggerR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO not full"]
    #[inline(always)]
    pub fn tnf(&self) -> TnfR {
        TnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO not empty"]
    #[inline(always)]
    pub fn rne(&self) -> RneR {
        RneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Full"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending Data is first Byte in BLOCKMODE"]
    #[inline(always)]
    pub fn rxdatafirst(&self) -> RxdatafirstR {
        RxdatafirstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX FIFO Above Trigger Level"]
    #[inline(always)]
    pub fn rxtrigger(&self) -> RxtriggerR {
        RxtriggerR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX FIFO Below Trigger Level"]
    #[inline(always)]
    pub fn txtrigger(&self) -> TxtriggerR {
        TxtriggerR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
