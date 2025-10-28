#[doc = "Register `MAC_DEBUG` reader"]
pub type R = crate::R<MacDebugSpec>;
#[doc = "Field `RPESTS` reader - MAC GMII or MII Receive Protocol Engine Status"]
pub type RpestsR = crate::BitReader;
#[doc = "Field `RFCFCSTS` reader - MAC Receive Frame FIFO Controller Status"]
pub type RfcfcstsR = crate::FieldReader;
#[doc = "Field `RWCSTS` reader - MTL Rx FIFO Write Controller Active Status"]
pub type RwcstsR = crate::BitReader;
#[doc = "Field `RRCSTS` reader - MTL RxFIFO Read Controller State"]
pub type RrcstsR = crate::FieldReader;
#[doc = "Field `RXFSTS` reader - MTL RxFIFO Fill-Level Status"]
pub type RxfstsR = crate::FieldReader;
#[doc = "Field `TPESTS` reader - MAC GMII or MII Transmit Protocol Engine Status"]
pub type TpestsR = crate::BitReader;
#[doc = "Field `TFCSTS` reader - PAC Transmit Frame Controller Status"]
pub type TfcstsR = crate::FieldReader;
#[doc = "Field `TXPAUSED` reader - MAC Transmitter in Pause"]
pub type TxpausedR = crate::BitReader;
#[doc = "Field `TRCSTS` reader - MTL Tx FIFO Read Controller Status"]
pub type TrcstsR = crate::FieldReader;
#[doc = "Field `TWCSTS` reader - MTL Tx FIFO Write Controller Status"]
pub type TwcstsR = crate::BitReader;
#[doc = "Field `TXFSTS` reader - MTL Tx FIFO Not Empty Status"]
pub type TxfstsR = crate::BitReader;
#[doc = "Field `TXSTSFSTS` reader - MTL TxStatus FIFO Full Status"]
pub type TxstsfstsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MAC GMII or MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub fn rpests(&self) -> RpestsR {
        RpestsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Frame FIFO Controller Status"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RfcfcstsR {
        RfcfcstsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - MTL Rx FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RwcstsR {
        RwcstsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - MTL RxFIFO Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RrcstsR {
        RrcstsR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9 - MTL RxFIFO Fill-Level Status"]
    #[inline(always)]
    pub fn rxfsts(&self) -> RxfstsR {
        RxfstsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - MAC GMII or MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub fn tpests(&self) -> TpestsR {
        TpestsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - PAC Transmit Frame Controller Status"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TfcstsR {
        TfcstsR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - MAC Transmitter in Pause"]
    #[inline(always)]
    pub fn txpaused(&self) -> TxpausedR {
        TxpausedR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - MTL Tx FIFO Read Controller Status"]
    #[inline(always)]
    pub fn trcsts(&self) -> TrcstsR {
        TrcstsR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - MTL Tx FIFO Write Controller Status"]
    #[inline(always)]
    pub fn twcsts(&self) -> TwcstsR {
        TwcstsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - MTL Tx FIFO Not Empty Status"]
    #[inline(always)]
    pub fn txfsts(&self) -> TxfstsR {
        TxfstsR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MTL TxStatus FIFO Full Status"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TxstsfstsR {
        TxstsfstsR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Gives the status of the various internal blocks for debugging\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_debug::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacDebugSpec;
impl crate::RegisterSpec for MacDebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_debug::R`](R) reader structure"]
impl crate::Readable for MacDebugSpec {}
#[doc = "`reset()` method sets MAC_DEBUG to value 0"]
impl crate::Resettable for MacDebugSpec {}
