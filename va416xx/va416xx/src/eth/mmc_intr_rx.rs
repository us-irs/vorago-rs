#[doc = "Register `MMC_INTR_RX` reader"]
pub type R = crate::R<MmcIntrRxSpec>;
#[doc = "Register `MMC_INTR_RX` writer"]
pub type W = crate::W<MmcIntrRxSpec>;
#[doc = "Field `RXGBFRMIS` reader - MMC Receive Good Bad Frame Counter Interrupt Status"]
pub type RxgbfrmisR = crate::BitReader;
#[doc = "Field `RXGBFRMIS` writer - MMC Receive Good Bad Frame Counter Interrupt Status"]
pub type RxgbfrmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXGBOCTIS` reader - MMC Receive Good Bad Octet Counter Interrupt Status"]
pub type RxgboctisR = crate::BitReader;
#[doc = "Field `RXGBOCTIS` writer - MMC Receive Good Bad Octet Counter Interrupt Status"]
pub type RxgboctisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXGOCTIS` reader - MMC Receive Good Octet Counter Interrupt Status"]
pub type RxgoctisR = crate::BitReader;
#[doc = "Field `RXGOCTIS` writer - MMC Receive Good Octet Counter Interrupt Status"]
pub type RxgoctisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBCGFIS` reader - MMC Receive Broadcast Good Frame Counter Interrupt Status"]
pub type RxbcgfisR = crate::BitReader;
#[doc = "Field `RXBCGFIS` writer - MMC Receive Broadcast Good Frame Counter Interrupt Status"]
pub type RxbcgfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMCGFIS` reader - MMC Receive Multicast Good Frame Counter Interrupt Status"]
pub type RxmcgfisR = crate::BitReader;
#[doc = "Field `RXMCGFIS` writer - MMC Receive Multicast Good Frame Counter Interrupt Status"]
pub type RxmcgfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCRCERFIS` reader - MMC Receive CRC Error Frame Counter Interrupt Status"]
pub type RxcrcerfisR = crate::BitReader;
#[doc = "Field `RXCRCERFIS` writer - MMC Receive CRC Error Frame Counter Interrupt Status"]
pub type RxcrcerfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXALGNERFIS` reader - MMC Receive Alignment Error Frame Counter Interrupt Status"]
pub type RxalgnerfisR = crate::BitReader;
#[doc = "Field `RXALGNERFIS` writer - MMC Receive Alignment Error Frame Counter Interrupt Status"]
pub type RxalgnerfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRUNTFIS` reader - MMC Receive Runt Frame Counter Interrupt Status"]
pub type RxruntfisR = crate::BitReader;
#[doc = "Field `RXRUNTFIS` writer - MMC Receive Runt Frame Counter Interrupt Status"]
pub type RxruntfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXJABERFIS` reader - MMC Receive Jabber Error Frame Counter Interrupt Status"]
pub type RxjaberfisR = crate::BitReader;
#[doc = "Field `RXJABERFIS` writer - MMC Receive Jabber Error Frame Counter Interrupt Status"]
pub type RxjaberfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUSIZEGFIS` reader - MMC Receive Undersize Good Frame Counter Interrupt Status"]
pub type RxusizegfisR = crate::BitReader;
#[doc = "Field `RXUSIZEGFIS` writer - MMC Receive Undersize Good Frame Counter Interrupt Status"]
pub type RxusizegfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOSIZEGFIS` reader - MMC Receive Oversize Good Frame Counter Interrupt Status"]
pub type RxosizegfisR = crate::BitReader;
#[doc = "Field `RXOSIZEGFIS` writer - MMC Receive Oversize Good Frame Counter Interrupt Status"]
pub type RxosizegfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX64OCTGBFIS` reader - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx64octgbfisR = crate::BitReader;
#[doc = "Field `RX64OCTGBFIS` writer - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx64octgbfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX65T127OCTGBFIS` reader - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx65t127octgbfisR = crate::BitReader;
#[doc = "Field `RX65T127OCTGBFIS` writer - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx65t127octgbfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX128T255OCTGBFIS` reader - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx128t255octgbfisR = crate::BitReader;
#[doc = "Field `RX128T255OCTGBFIS` writer - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx128t255octgbfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX256T511OCTGBFIS` reader - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx256t511octgbfisR = crate::BitReader;
#[doc = "Field `RX256T511OCTGBFIS` writer - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx256t511octgbfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX512T1023OCTGBFIS` reader - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx512t1023octgbfisR = crate::BitReader;
#[doc = "Field `RX512T1023OCTGBFIS` writer - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
pub type Rx512t1023octgbfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX1024TMAXOCTGBFIS` reader - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status."]
pub type Rx1024tmaxoctgbfisR = crate::BitReader;
#[doc = "Field `RX1024TMAXOCTGBFIS` writer - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status."]
pub type Rx1024tmaxoctgbfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUCGFIS` reader - MMC Receive Unicast Good Frame Counter Interrupt Status"]
pub type RxucgfisR = crate::BitReader;
#[doc = "Field `RXUCGFIS` writer - MMC Receive Unicast Good Frame Counter Interrupt Status"]
pub type RxucgfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLENERFIS` reader - MMC Receive Length Error Frame Counter Interrupt Status"]
pub type RxlenerfisR = crate::BitReader;
#[doc = "Field `RXLENERFIS` writer - MMC Receive Length Error Frame Counter Interrupt Status"]
pub type RxlenerfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXORANGEFIS` reader - MMC Receive Out Of Range Error Frame Counter Interrupt Status."]
pub type RxorangefisR = crate::BitReader;
#[doc = "Field `RXORANGEFIS` writer - MMC Receive Out Of Range Error Frame Counter Interrupt Status."]
pub type RxorangefisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPAUSFIS` reader - MMC Receive Pause Frame Counter Interrupt Status"]
pub type RxpausfisR = crate::BitReader;
#[doc = "Field `RXPAUSFIS` writer - MMC Receive Pause Frame Counter Interrupt Status"]
pub type RxpausfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFOVFIS` reader - MMC Receive FIFO Overflow Frame Counter Interrupt Status"]
pub type RxfovfisR = crate::BitReader;
#[doc = "Field `RXFOVFIS` writer - MMC Receive FIFO Overflow Frame Counter Interrupt Status"]
pub type RxfovfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXVLANGBFIS` reader - MMC Receive VLAN Good Bad Frame Counter Interrupt Status"]
pub type RxvlangbfisR = crate::BitReader;
#[doc = "Field `RXVLANGBFIS` writer - MMC Receive VLAN Good Bad Frame Counter Interrupt Status"]
pub type RxvlangbfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXWDOGFIS` reader - MMC Receive Watchdog Error Frame Counter Interrupt Status"]
pub type RxwdogfisR = crate::BitReader;
#[doc = "Field `RXWDOGFIS` writer - MMC Receive Watchdog Error Frame Counter Interrupt Status"]
pub type RxwdogfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRCVERRFIS` reader - MMC Receive Error Frame Counter Interrupt Status"]
pub type RxrcverrfisR = crate::BitReader;
#[doc = "Field `RXRCVERRFIS` writer - MMC Receive Error Frame Counter Interrupt Status"]
pub type RxrcverrfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCTRLFIS` reader - MMC Receive Control Frame Counter Interrupt Status"]
pub type RxctrlfisR = crate::BitReader;
#[doc = "Field `RXCTRLFIS` writer - MMC Receive Control Frame Counter Interrupt Status"]
pub type RxctrlfisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgbfrmis(&self) -> RxgbfrmisR {
        RxgbfrmisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgboctis(&self) -> RxgboctisR {
        RxgboctisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgoctis(&self) -> RxgoctisR {
        RxgoctisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxbcgfis(&self) -> RxbcgfisR {
        RxbcgfisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxmcgfis(&self) -> RxmcgfisR {
        RxmcgfisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxcrcerfis(&self) -> RxcrcerfisR {
        RxcrcerfisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxalgnerfis(&self) -> RxalgnerfisR {
        RxalgnerfisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxruntfis(&self) -> RxruntfisR {
        RxruntfisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxjaberfis(&self) -> RxjaberfisR {
        RxjaberfisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxusizegfis(&self) -> RxusizegfisR {
        RxusizegfisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxosizegfis(&self) -> RxosizegfisR {
        RxosizegfisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx64octgbfis(&self) -> Rx64octgbfisR {
        Rx64octgbfisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx65t127octgbfis(&self) -> Rx65t127octgbfisR {
        Rx65t127octgbfisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx128t255octgbfis(&self) -> Rx128t255octgbfisR {
        Rx128t255octgbfisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx256t511octgbfis(&self) -> Rx256t511octgbfisR {
        Rx256t511octgbfisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx512t1023octgbfis(&self) -> Rx512t1023octgbfisR {
        Rx512t1023octgbfisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfis(&self) -> Rx1024tmaxoctgbfisR {
        Rx1024tmaxoctgbfisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxucgfis(&self) -> RxucgfisR {
        RxucgfisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxlenerfis(&self) -> RxlenerfisR {
        RxlenerfisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn rxorangefis(&self) -> RxorangefisR {
        RxorangefisR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxpausfis(&self) -> RxpausfisR {
        RxpausfisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxfovfis(&self) -> RxfovfisR {
        RxfovfisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxvlangbfis(&self) -> RxvlangbfisR {
        RxvlangbfisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxwdogfis(&self) -> RxwdogfisR {
        RxwdogfisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxrcverrfis(&self) -> RxrcverrfisR {
        RxrcverrfisR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxctrlfis(&self) -> RxctrlfisR {
        RxctrlfisR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgbfrmis(&mut self) -> RxgbfrmisW<'_, MmcIntrRxSpec> {
        RxgbfrmisW::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgboctis(&mut self) -> RxgboctisW<'_, MmcIntrRxSpec> {
        RxgboctisW::new(self, 1)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgoctis(&mut self) -> RxgoctisW<'_, MmcIntrRxSpec> {
        RxgoctisW::new(self, 2)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxbcgfis(&mut self) -> RxbcgfisW<'_, MmcIntrRxSpec> {
        RxbcgfisW::new(self, 3)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxmcgfis(&mut self) -> RxmcgfisW<'_, MmcIntrRxSpec> {
        RxmcgfisW::new(self, 4)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxcrcerfis(&mut self) -> RxcrcerfisW<'_, MmcIntrRxSpec> {
        RxcrcerfisW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxalgnerfis(&mut self) -> RxalgnerfisW<'_, MmcIntrRxSpec> {
        RxalgnerfisW::new(self, 6)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxruntfis(&mut self) -> RxruntfisW<'_, MmcIntrRxSpec> {
        RxruntfisW::new(self, 7)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxjaberfis(&mut self) -> RxjaberfisW<'_, MmcIntrRxSpec> {
        RxjaberfisW::new(self, 8)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxusizegfis(&mut self) -> RxusizegfisW<'_, MmcIntrRxSpec> {
        RxusizegfisW::new(self, 9)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxosizegfis(&mut self) -> RxosizegfisW<'_, MmcIntrRxSpec> {
        RxosizegfisW::new(self, 10)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx64octgbfis(&mut self) -> Rx64octgbfisW<'_, MmcIntrRxSpec> {
        Rx64octgbfisW::new(self, 11)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx65t127octgbfis(&mut self) -> Rx65t127octgbfisW<'_, MmcIntrRxSpec> {
        Rx65t127octgbfisW::new(self, 12)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx128t255octgbfis(&mut self) -> Rx128t255octgbfisW<'_, MmcIntrRxSpec> {
        Rx128t255octgbfisW::new(self, 13)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx256t511octgbfis(&mut self) -> Rx256t511octgbfisW<'_, MmcIntrRxSpec> {
        Rx256t511octgbfisW::new(self, 14)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx512t1023octgbfis(&mut self) -> Rx512t1023octgbfisW<'_, MmcIntrRxSpec> {
        Rx512t1023octgbfisW::new(self, 15)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfis(&mut self) -> Rx1024tmaxoctgbfisW<'_, MmcIntrRxSpec> {
        Rx1024tmaxoctgbfisW::new(self, 16)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxucgfis(&mut self) -> RxucgfisW<'_, MmcIntrRxSpec> {
        RxucgfisW::new(self, 17)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxlenerfis(&mut self) -> RxlenerfisW<'_, MmcIntrRxSpec> {
        RxlenerfisW::new(self, 18)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn rxorangefis(&mut self) -> RxorangefisW<'_, MmcIntrRxSpec> {
        RxorangefisW::new(self, 19)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxpausfis(&mut self) -> RxpausfisW<'_, MmcIntrRxSpec> {
        RxpausfisW::new(self, 20)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxfovfis(&mut self) -> RxfovfisW<'_, MmcIntrRxSpec> {
        RxfovfisW::new(self, 21)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxvlangbfis(&mut self) -> RxvlangbfisW<'_, MmcIntrRxSpec> {
        RxvlangbfisW::new(self, 22)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxwdogfis(&mut self) -> RxwdogfisW<'_, MmcIntrRxSpec> {
        RxwdogfisW::new(self, 23)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxrcverrfis(&mut self) -> RxrcverrfisW<'_, MmcIntrRxSpec> {
        RxrcverrfisW::new(self, 24)
    }
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxctrlfis(&mut self) -> RxctrlfisW<'_, MmcIntrRxSpec> {
        RxctrlfisW::new(self, 25)
    }
}
#[doc = "MMC Receive Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_intr_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_intr_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcIntrRxSpec;
impl crate::RegisterSpec for MmcIntrRxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_intr_rx::R`](R) reader structure"]
impl crate::Readable for MmcIntrRxSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_intr_rx::W`](W) writer structure"]
impl crate::Writable for MmcIntrRxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMC_INTR_RX to value 0"]
impl crate::Resettable for MmcIntrRxSpec {}
