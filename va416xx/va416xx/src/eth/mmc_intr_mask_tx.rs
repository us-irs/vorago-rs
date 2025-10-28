#[doc = "Register `MMC_INTR_MASK_TX` reader"]
pub type R = crate::R<MmcIntrMaskTxSpec>;
#[doc = "Register `MMC_INTR_MASK_TX` writer"]
pub type W = crate::W<MmcIntrMaskTxSpec>;
#[doc = "Field `TXGBOCTIM` reader - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
pub type TxgboctimR = crate::BitReader;
#[doc = "Field `TXGBOCTIM` writer - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
pub type TxgboctimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXGBFRMIM` reader - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
pub type TxgbfrmimR = crate::BitReader;
#[doc = "Field `TXGBFRMIM` writer - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
pub type TxgbfrmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBCGFIM` reader - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
pub type TxbcgfimR = crate::BitReader;
#[doc = "Field `TXBCGFIM` writer - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
pub type TxbcgfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMCGFIM` reader - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
pub type TxmcgfimR = crate::BitReader;
#[doc = "Field `TXMCGFIM` writer - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
pub type TxmcgfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX64OCTGBFIM` reader - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Tx64octgbfimR = crate::BitReader;
#[doc = "Field `TX64OCTGBFIM` writer - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Tx64octgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX65T127OCTGBFIM` reader - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Tx65t127octgbfimR = crate::BitReader;
#[doc = "Field `TX65T127OCTGBFIM` writer - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Tx65t127octgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX128T255OCTGBFIM` reader - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Tx128t255octgbfimR = crate::BitReader;
#[doc = "Field `TX128T255OCTGBFIM` writer - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Tx128t255octgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX256T511OCTGBFIM` reader - MC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Tx256t511octgbfimR = crate::BitReader;
#[doc = "Field `TX256T511OCTGBFIM` writer - MC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Tx256t511octgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX512T1023OCTGBFIM` reader - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Tx512t1023octgbfimR = crate::BitReader;
#[doc = "Field `TX512T1023OCTGBFIM` writer - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Tx512t1023octgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX1024TMAXOCTGBFIM` reader - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
pub type Tx1024tmaxoctgbfimR = crate::BitReader;
#[doc = "Field `TX1024TMAXOCTGBFIM` writer - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
pub type Tx1024tmaxoctgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUCGBFIM` reader - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
pub type TxucgbfimR = crate::BitReader;
#[doc = "Field `TXUCGBFIM` writer - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
pub type TxucgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMCGBFIM` reader - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
pub type TxmcgbfimR = crate::BitReader;
#[doc = "Field `TXMCGBFIM` writer - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
pub type TxmcgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBCGBFIM` reader - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
pub type TxbcgbfimR = crate::BitReader;
#[doc = "Field `TXBCGBFIM` writer - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
pub type TxbcgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUFLOWERFIM` reader - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
pub type TxuflowerfimR = crate::BitReader;
#[doc = "Field `TXUFLOWERFIM` writer - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
pub type TxuflowerfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSCOLGFIM` reader - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
pub type TxscolgfimR = crate::BitReader;
#[doc = "Field `TXSCOLGFIM` writer - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
pub type TxscolgfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMCOLGFIM` reader - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
pub type TxmcolgfimR = crate::BitReader;
#[doc = "Field `TXMCOLGFIM` writer - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
pub type TxmcolgfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDEFFIM` reader - MMC Transmit Deferred Frame Counter Interrupt Mask"]
pub type TxdeffimR = crate::BitReader;
#[doc = "Field `TXDEFFIM` writer - MMC Transmit Deferred Frame Counter Interrupt Mask"]
pub type TxdeffimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXLATCOLFIM` reader - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
pub type TxlatcolfimR = crate::BitReader;
#[doc = "Field `TXLATCOLFIM` writer - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
pub type TxlatcolfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEXCOLFIM` reader - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
pub type TxexcolfimR = crate::BitReader;
#[doc = "Field `TXEXCOLFIM` writer - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
pub type TxexcolfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCARERFIM` reader - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
pub type TxcarerfimR = crate::BitReader;
#[doc = "Field `TXCARERFIM` writer - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
pub type TxcarerfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXGOCTIM` reader - MMC Transmit Good Octet Counter Interrupt Mask"]
pub type TxgoctimR = crate::BitReader;
#[doc = "Field `TXGOCTIM` writer - MMC Transmit Good Octet Counter Interrupt Mask"]
pub type TxgoctimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXGFRMIM` reader - MMC Transmit Good Frame Counter Interrupt Mask"]
pub type TxgfrmimR = crate::BitReader;
#[doc = "Field `TXGFRMIM` writer - MMC Transmit Good Frame Counter Interrupt Mask"]
pub type TxgfrmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEXDEFFIM` reader - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
pub type TxexdeffimR = crate::BitReader;
#[doc = "Field `TXEXDEFFIM` writer - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
pub type TxexdeffimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPAUSFIM` reader - MMC Transmit Pause Frame Counter Interrupt Mask"]
pub type TxpausfimR = crate::BitReader;
#[doc = "Field `TXPAUSFIM` writer - MMC Transmit Pause Frame Counter Interrupt Mask"]
pub type TxpausfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXVLANGFIM` reader - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
pub type TxvlangfimR = crate::BitReader;
#[doc = "Field `TXVLANGFIM` writer - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
pub type TxvlangfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOSIZEGFIM` reader - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
pub type TxosizegfimR = crate::BitReader;
#[doc = "Field `TXOSIZEGFIM` writer - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
pub type TxosizegfimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgboctim(&self) -> TxgboctimR {
        TxgboctimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgbfrmim(&self) -> TxgbfrmimR {
        TxgbfrmimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgfim(&self) -> TxbcgfimR {
        TxbcgfimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgfim(&self) -> TxmcgfimR {
        TxmcgfimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx64octgbfim(&self) -> Tx64octgbfimR {
        Tx64octgbfimR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx65t127octgbfim(&self) -> Tx65t127octgbfimR {
        Tx65t127octgbfimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx128t255octgbfim(&self) -> Tx128t255octgbfimR {
        Tx128t255octgbfimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx256t511octgbfim(&self) -> Tx256t511octgbfimR {
        Tx256t511octgbfimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx512t1023octgbfim(&self) -> Tx512t1023octgbfimR {
        Tx512t1023octgbfimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfim(&self) -> Tx1024tmaxoctgbfimR {
        Tx1024tmaxoctgbfimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txucgbfim(&self) -> TxucgbfimR {
        TxucgbfimR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgbfim(&self) -> TxmcgbfimR {
        TxmcgbfimR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgbfim(&self) -> TxbcgbfimR {
        TxbcgbfimR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txuflowerfim(&self) -> TxuflowerfimR {
        TxuflowerfimR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgfim(&self) -> TxscolgfimR {
        TxscolgfimR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgfim(&self) -> TxmcolgfimR {
        TxmcolgfimR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txdeffim(&self) -> TxdeffimR {
        TxdeffimR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txlatcolfim(&self) -> TxlatcolfimR {
        TxlatcolfimR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexcolfim(&self) -> TxexcolfimR {
        TxexcolfimR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txcarerfim(&self) -> TxcarerfimR {
        TxcarerfimR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgoctim(&self) -> TxgoctimR {
        TxgoctimR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgfrmim(&self) -> TxgfrmimR {
        TxgfrmimR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexdeffim(&self) -> TxexdeffimR {
        TxexdeffimR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txpausfim(&self) -> TxpausfimR {
        TxpausfimR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txvlangfim(&self) -> TxvlangfimR {
        TxvlangfimR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txosizegfim(&self) -> TxosizegfimR {
        TxosizegfimR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgboctim(&mut self) -> TxgboctimW<'_, MmcIntrMaskTxSpec> {
        TxgboctimW::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgbfrmim(&mut self) -> TxgbfrmimW<'_, MmcIntrMaskTxSpec> {
        TxgbfrmimW::new(self, 1)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgfim(&mut self) -> TxbcgfimW<'_, MmcIntrMaskTxSpec> {
        TxbcgfimW::new(self, 2)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgfim(&mut self) -> TxmcgfimW<'_, MmcIntrMaskTxSpec> {
        TxmcgfimW::new(self, 3)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx64octgbfim(&mut self) -> Tx64octgbfimW<'_, MmcIntrMaskTxSpec> {
        Tx64octgbfimW::new(self, 4)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx65t127octgbfim(&mut self) -> Tx65t127octgbfimW<'_, MmcIntrMaskTxSpec> {
        Tx65t127octgbfimW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx128t255octgbfim(&mut self) -> Tx128t255octgbfimW<'_, MmcIntrMaskTxSpec> {
        Tx128t255octgbfimW::new(self, 6)
    }
    #[doc = "Bit 7 - MC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx256t511octgbfim(&mut self) -> Tx256t511octgbfimW<'_, MmcIntrMaskTxSpec> {
        Tx256t511octgbfimW::new(self, 7)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx512t1023octgbfim(&mut self) -> Tx512t1023octgbfimW<'_, MmcIntrMaskTxSpec> {
        Tx512t1023octgbfimW::new(self, 8)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfim(&mut self) -> Tx1024tmaxoctgbfimW<'_, MmcIntrMaskTxSpec> {
        Tx1024tmaxoctgbfimW::new(self, 9)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txucgbfim(&mut self) -> TxucgbfimW<'_, MmcIntrMaskTxSpec> {
        TxucgbfimW::new(self, 10)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcgbfim(&mut self) -> TxmcgbfimW<'_, MmcIntrMaskTxSpec> {
        TxmcgbfimW::new(self, 11)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txbcgbfim(&mut self) -> TxbcgbfimW<'_, MmcIntrMaskTxSpec> {
        TxbcgbfimW::new(self, 12)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txuflowerfim(&mut self) -> TxuflowerfimW<'_, MmcIntrMaskTxSpec> {
        TxuflowerfimW::new(self, 13)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgfim(&mut self) -> TxscolgfimW<'_, MmcIntrMaskTxSpec> {
        TxscolgfimW::new(self, 14)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgfim(&mut self) -> TxmcolgfimW<'_, MmcIntrMaskTxSpec> {
        TxmcolgfimW::new(self, 15)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txdeffim(&mut self) -> TxdeffimW<'_, MmcIntrMaskTxSpec> {
        TxdeffimW::new(self, 16)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txlatcolfim(&mut self) -> TxlatcolfimW<'_, MmcIntrMaskTxSpec> {
        TxlatcolfimW::new(self, 17)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexcolfim(&mut self) -> TxexcolfimW<'_, MmcIntrMaskTxSpec> {
        TxexcolfimW::new(self, 18)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txcarerfim(&mut self) -> TxcarerfimW<'_, MmcIntrMaskTxSpec> {
        TxcarerfimW::new(self, 19)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgoctim(&mut self) -> TxgoctimW<'_, MmcIntrMaskTxSpec> {
        TxgoctimW::new(self, 20)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgfrmim(&mut self) -> TxgfrmimW<'_, MmcIntrMaskTxSpec> {
        TxgfrmimW::new(self, 21)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txexdeffim(&mut self) -> TxexdeffimW<'_, MmcIntrMaskTxSpec> {
        TxexdeffimW::new(self, 22)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txpausfim(&mut self) -> TxpausfimW<'_, MmcIntrMaskTxSpec> {
        TxpausfimW::new(self, 23)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txvlangfim(&mut self) -> TxvlangfimW<'_, MmcIntrMaskTxSpec> {
        TxvlangfimW::new(self, 24)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txosizegfim(&mut self) -> TxosizegfimW<'_, MmcIntrMaskTxSpec> {
        TxosizegfimW::new(self, 25)
    }
}
#[doc = "MMC Transmit Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_intr_mask_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_intr_mask_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcIntrMaskTxSpec;
impl crate::RegisterSpec for MmcIntrMaskTxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_intr_mask_tx::R`](R) reader structure"]
impl crate::Readable for MmcIntrMaskTxSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_intr_mask_tx::W`](W) writer structure"]
impl crate::Writable for MmcIntrMaskTxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMC_INTR_MASK_TX to value 0"]
impl crate::Resettable for MmcIntrMaskTxSpec {}
