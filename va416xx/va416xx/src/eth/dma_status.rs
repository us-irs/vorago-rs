#[doc = "Register `DMA_STATUS` reader"]
pub type R = crate::R<DmaStatusSpec>;
#[doc = "Field `TI` reader - Transmit Interrupt"]
pub type TiR = crate::BitReader;
#[doc = "Field `TPS` reader - Transmit Process Stopped"]
pub type TpsR = crate::BitReader;
#[doc = "Field `TU` reader - Transmit Buffer Unavailable"]
pub type TuR = crate::BitReader;
#[doc = "Field `TJT` reader - Transmit Jabber Timeout"]
pub type TjtR = crate::BitReader;
#[doc = "Field `OVF` reader - Receive Underflow"]
pub type OvfR = crate::BitReader;
#[doc = "Field `UNF` reader - Transmit Underflow"]
pub type UnfR = crate::BitReader;
#[doc = "Field `RI` reader - Receive Interrupt"]
pub type RiR = crate::BitReader;
#[doc = "Field `RU` reader - Receive Buffer Unavailable"]
pub type RuR = crate::BitReader;
#[doc = "Field `RPS` reader - Receive Process Stopped"]
pub type RpsR = crate::BitReader;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout"]
pub type RwtR = crate::BitReader;
#[doc = "Field `ETI` reader - Early Transmit Interrupt"]
pub type EtiR = crate::BitReader;
#[doc = "Field `FBI` reader - Fatal Bus Error Interruptble"]
pub type FbiR = crate::BitReader;
#[doc = "Field `ERI` reader - Early Receive Interrupt"]
pub type EriR = crate::BitReader;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary"]
pub type AisR = crate::BitReader;
#[doc = "Field `NIS` reader - Normal Interrupt Summary"]
pub type NisR = crate::BitReader;
#[doc = "Field `RS` reader - Receive Process State"]
pub type RsR = crate::FieldReader;
#[doc = "Field `TS` reader - Transmit Process State"]
pub type TsR = crate::FieldReader;
#[doc = "Field `EB` reader - Error Bits"]
pub type EbR = crate::FieldReader;
#[doc = "Field `GMI` reader - GMAC MMC Interrupt"]
pub type GmiR = crate::BitReader;
#[doc = "Field `TTI` reader - Timestamp Trigger Interrupt"]
pub type TtiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TpsR {
        TpsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tu(&self) -> TuR {
        TuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn tjt(&self) -> TjtR {
        TjtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Underflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    pub fn unf(&self) -> UnfR {
        UnfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn ru(&self) -> RuR {
        RuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RpsR {
        RpsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RwtR {
        RwtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn eti(&self) -> EtiR {
        EtiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interruptble"]
    #[inline(always)]
    pub fn fbi(&self) -> FbiR {
        FbiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn eri(&self) -> EriR {
        EriR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AisR {
        AisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NisR {
        NisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive Process State"]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error Bits"]
    #[inline(always)]
    pub fn eb(&self) -> EbR {
        EbR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - GMAC MMC Interrupt"]
    #[inline(always)]
    pub fn gmi(&self) -> GmiR {
        GmiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt"]
    #[inline(always)]
    pub fn tti(&self) -> TtiR {
        TtiR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Used to determine the status of the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaStatusSpec;
impl crate::RegisterSpec for DmaStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_status::R`](R) reader structure"]
impl crate::Readable for DmaStatusSpec {}
#[doc = "`reset()` method sets DMA_STATUS to value 0"]
impl crate::Resettable for DmaStatusSpec {}
