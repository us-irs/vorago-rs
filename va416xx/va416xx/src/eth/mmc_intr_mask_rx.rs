#[doc = "Register `MMC_INTR_MASK_RX` reader"]
pub type R = crate::R<MmcIntrMaskRxSpec>;
#[doc = "Register `MMC_INTR_MASK_RX` writer"]
pub type W = crate::W<MmcIntrMaskRxSpec>;
#[doc = "Field `RXGBFRMIM` reader - MMC Receive Good Bad Frame Counter Interrupt Mask"]
pub type RxgbfrmimR = crate::BitReader;
#[doc = "Field `RXGBFRMIM` writer - MMC Receive Good Bad Frame Counter Interrupt Mask"]
pub type RxgbfrmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXGBOCTIM` reader - MMC Receive Good Bad Octet Counter Interrupt Mask."]
pub type RxgboctimR = crate::BitReader;
#[doc = "Field `RXGBOCTIM` writer - MMC Receive Good Bad Octet Counter Interrupt Mask."]
pub type RxgboctimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXGOCTIM` reader - MMC Receive Good Octet Counter Interrupt Mask"]
pub type RxgoctimR = crate::BitReader;
#[doc = "Field `RXGOCTIM` writer - MMC Receive Good Octet Counter Interrupt Mask"]
pub type RxgoctimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBCGFIM` reader - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
pub type RxbcgfimR = crate::BitReader;
#[doc = "Field `RXBCGFIM` writer - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
pub type RxbcgfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMCGFIM` reader - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
pub type RxmcgfimR = crate::BitReader;
#[doc = "Field `RXMCGFIM` writer - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
pub type RxmcgfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCRCERFIM` reader - MMC Receive CRC Error Frame Counter Interrupt Mask"]
pub type RxcrcerfimR = crate::BitReader;
#[doc = "Field `RXCRCERFIM` writer - MMC Receive CRC Error Frame Counter Interrupt Mask"]
pub type RxcrcerfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXALGNERFIM` reader - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
pub type RxalgnerfimR = crate::BitReader;
#[doc = "Field `RXALGNERFIM` writer - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
pub type RxalgnerfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRUNTFIM` reader - MMC Receive Runt Frame Counter Interrupt Mask"]
pub type RxruntfimR = crate::BitReader;
#[doc = "Field `RXRUNTFIM` writer - MMC Receive Runt Frame Counter Interrupt Mask"]
pub type RxruntfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXJABERFIM` reader - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
pub type RxjaberfimR = crate::BitReader;
#[doc = "Field `RXJABERFIM` writer - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
pub type RxjaberfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUSIZEGFIM` reader - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
pub type RxusizegfimR = crate::BitReader;
#[doc = "Field `RXUSIZEGFIM` writer - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
pub type RxusizegfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOSIZEGFIM` reader - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
pub type RxosizegfimR = crate::BitReader;
#[doc = "Field `RXOSIZEGFIM` writer - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
pub type RxosizegfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX64OCTGBFIM` reader - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Rx64octgbfimR = crate::BitReader;
#[doc = "Field `RX64OCTGBFIM` writer - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Rx64octgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX65T127OCTGBFIM` reader - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Rx65t127octgbfimR = crate::BitReader;
#[doc = "Field `RX65T127OCTGBFIM` writer - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Rx65t127octgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX128T255OCTGBFIM` reader - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Rx128t255octgbfimR = crate::BitReader;
#[doc = "Field `RX128T255OCTGBFIM` writer - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Rx128t255octgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX256T511OCTGBFIM` reader - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Rx256t511octgbfimR = crate::BitReader;
#[doc = "Field `RX256T511OCTGBFIM` writer - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Rx256t511octgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX512T1023OCTGBFIM` reader - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Rx512t1023octgbfimR = crate::BitReader;
#[doc = "Field `RX512T1023OCTGBFIM` writer - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
pub type Rx512t1023octgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX1024TMAXOCTGBFIM` reader - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask."]
pub type Rx1024tmaxoctgbfimR = crate::BitReader;
#[doc = "Field `RX1024TMAXOCTGBFIM` writer - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask."]
pub type Rx1024tmaxoctgbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUCGFIM` reader - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
pub type RxucgfimR = crate::BitReader;
#[doc = "Field `RXUCGFIM` writer - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
pub type RxucgfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLENERFIM` reader - MMC Receive Length Error Frame Counter Interrupt Mask"]
pub type RxlenerfimR = crate::BitReader;
#[doc = "Field `RXLENERFIM` writer - MMC Receive Length Error Frame Counter Interrupt Mask"]
pub type RxlenerfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXORANGEFIM` reader - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
pub type RxorangefimR = crate::BitReader;
#[doc = "Field `RXORANGEFIM` writer - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
pub type RxorangefimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPAUSFIM` reader - MMC Receive Pause Frame Counter Interrupt Mask"]
pub type RxpausfimR = crate::BitReader;
#[doc = "Field `RXPAUSFIM` writer - MMC Receive Pause Frame Counter Interrupt Mask"]
pub type RxpausfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFOVFIM` reader - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
pub type RxfovfimR = crate::BitReader;
#[doc = "Field `RXFOVFIM` writer - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
pub type RxfovfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXVLANGBFIM` reader - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
pub type RxvlangbfimR = crate::BitReader;
#[doc = "Field `RXVLANGBFIM` writer - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
pub type RxvlangbfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXWDOGFIM` reader - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
pub type RxwdogfimR = crate::BitReader;
#[doc = "Field `RXWDOGFIM` writer - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
pub type RxwdogfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRCVERRFIM` reader - MMC Receive Error Frame Counter Interrupt Mask"]
pub type RxrcverrfimR = crate::BitReader;
#[doc = "Field `RXRCVERRFIM` writer - MMC Receive Error Frame Counter Interrupt Mask"]
pub type RxrcverrfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCTRLFIM` reader - MMC Receive Control Frame Counter Interrupt Mask"]
pub type RxctrlfimR = crate::BitReader;
#[doc = "Field `RXCTRLFIM` writer - MMC Receive Control Frame Counter Interrupt Mask"]
pub type RxctrlfimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgbfrmim(&self) -> RxgbfrmimR {
        RxgbfrmimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Mask."]
    #[inline(always)]
    pub fn rxgboctim(&self) -> RxgboctimR {
        RxgboctimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgoctim(&self) -> RxgoctimR {
        RxgoctimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxbcgfim(&self) -> RxbcgfimR {
        RxbcgfimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxmcgfim(&self) -> RxmcgfimR {
        RxmcgfimR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxcrcerfim(&self) -> RxcrcerfimR {
        RxcrcerfimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxalgnerfim(&self) -> RxalgnerfimR {
        RxalgnerfimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxruntfim(&self) -> RxruntfimR {
        RxruntfimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxjaberfim(&self) -> RxjaberfimR {
        RxjaberfimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxusizegfim(&self) -> RxusizegfimR {
        RxusizegfimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxosizegfim(&self) -> RxosizegfimR {
        RxosizegfimR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx64octgbfim(&self) -> Rx64octgbfimR {
        Rx64octgbfimR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx65t127octgbfim(&self) -> Rx65t127octgbfimR {
        Rx65t127octgbfimR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx128t255octgbfim(&self) -> Rx128t255octgbfimR {
        Rx128t255octgbfimR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx256t511octgbfim(&self) -> Rx256t511octgbfimR {
        Rx256t511octgbfimR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx512t1023octgbfim(&self) -> Rx512t1023octgbfimR {
        Rx512t1023octgbfimR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask."]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfim(&self) -> Rx1024tmaxoctgbfimR {
        Rx1024tmaxoctgbfimR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxucgfim(&self) -> RxucgfimR {
        RxucgfimR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxlenerfim(&self) -> RxlenerfimR {
        RxlenerfimR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxorangefim(&self) -> RxorangefimR {
        RxorangefimR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxpausfim(&self) -> RxpausfimR {
        RxpausfimR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxfovfim(&self) -> RxfovfimR {
        RxfovfimR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxvlangbfim(&self) -> RxvlangbfimR {
        RxvlangbfimR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxwdogfim(&self) -> RxwdogfimR {
        RxwdogfimR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxrcverrfim(&self) -> RxrcverrfimR {
        RxrcverrfimR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxctrlfim(&self) -> RxctrlfimR {
        RxctrlfimR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgbfrmim(&mut self) -> RxgbfrmimW<'_, MmcIntrMaskRxSpec> {
        RxgbfrmimW::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Mask."]
    #[inline(always)]
    pub fn rxgboctim(&mut self) -> RxgboctimW<'_, MmcIntrMaskRxSpec> {
        RxgboctimW::new(self, 1)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxgoctim(&mut self) -> RxgoctimW<'_, MmcIntrMaskRxSpec> {
        RxgoctimW::new(self, 2)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxbcgfim(&mut self) -> RxbcgfimW<'_, MmcIntrMaskRxSpec> {
        RxbcgfimW::new(self, 3)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxmcgfim(&mut self) -> RxmcgfimW<'_, MmcIntrMaskRxSpec> {
        RxmcgfimW::new(self, 4)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxcrcerfim(&mut self) -> RxcrcerfimW<'_, MmcIntrMaskRxSpec> {
        RxcrcerfimW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxalgnerfim(&mut self) -> RxalgnerfimW<'_, MmcIntrMaskRxSpec> {
        RxalgnerfimW::new(self, 6)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxruntfim(&mut self) -> RxruntfimW<'_, MmcIntrMaskRxSpec> {
        RxruntfimW::new(self, 7)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxjaberfim(&mut self) -> RxjaberfimW<'_, MmcIntrMaskRxSpec> {
        RxjaberfimW::new(self, 8)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxusizegfim(&mut self) -> RxusizegfimW<'_, MmcIntrMaskRxSpec> {
        RxusizegfimW::new(self, 9)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxosizegfim(&mut self) -> RxosizegfimW<'_, MmcIntrMaskRxSpec> {
        RxosizegfimW::new(self, 10)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx64octgbfim(&mut self) -> Rx64octgbfimW<'_, MmcIntrMaskRxSpec> {
        Rx64octgbfimW::new(self, 11)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx65t127octgbfim(&mut self) -> Rx65t127octgbfimW<'_, MmcIntrMaskRxSpec> {
        Rx65t127octgbfimW::new(self, 12)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx128t255octgbfim(&mut self) -> Rx128t255octgbfimW<'_, MmcIntrMaskRxSpec> {
        Rx128t255octgbfimW::new(self, 13)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx256t511octgbfim(&mut self) -> Rx256t511octgbfimW<'_, MmcIntrMaskRxSpec> {
        Rx256t511octgbfimW::new(self, 14)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rx512t1023octgbfim(&mut self) -> Rx512t1023octgbfimW<'_, MmcIntrMaskRxSpec> {
        Rx512t1023octgbfimW::new(self, 15)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask."]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfim(&mut self) -> Rx1024tmaxoctgbfimW<'_, MmcIntrMaskRxSpec> {
        Rx1024tmaxoctgbfimW::new(self, 16)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxucgfim(&mut self) -> RxucgfimW<'_, MmcIntrMaskRxSpec> {
        RxucgfimW::new(self, 17)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxlenerfim(&mut self) -> RxlenerfimW<'_, MmcIntrMaskRxSpec> {
        RxlenerfimW::new(self, 18)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxorangefim(&mut self) -> RxorangefimW<'_, MmcIntrMaskRxSpec> {
        RxorangefimW::new(self, 19)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxpausfim(&mut self) -> RxpausfimW<'_, MmcIntrMaskRxSpec> {
        RxpausfimW::new(self, 20)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxfovfim(&mut self) -> RxfovfimW<'_, MmcIntrMaskRxSpec> {
        RxfovfimW::new(self, 21)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxvlangbfim(&mut self) -> RxvlangbfimW<'_, MmcIntrMaskRxSpec> {
        RxvlangbfimW::new(self, 22)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxwdogfim(&mut self) -> RxwdogfimW<'_, MmcIntrMaskRxSpec> {
        RxwdogfimW::new(self, 23)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxrcverrfim(&mut self) -> RxrcverrfimW<'_, MmcIntrMaskRxSpec> {
        RxrcverrfimW::new(self, 24)
    }
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxctrlfim(&mut self) -> RxctrlfimW<'_, MmcIntrMaskRxSpec> {
        RxctrlfimW::new(self, 25)
    }
}
#[doc = "MMC Receive Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_intr_mask_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_intr_mask_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcIntrMaskRxSpec;
impl crate::RegisterSpec for MmcIntrMaskRxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_intr_mask_rx::R`](R) reader structure"]
impl crate::Readable for MmcIntrMaskRxSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_intr_mask_rx::W`](W) writer structure"]
impl crate::Writable for MmcIntrMaskRxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMC_INTR_MASK_RX to value 0"]
impl crate::Resettable for MmcIntrMaskRxSpec {}
