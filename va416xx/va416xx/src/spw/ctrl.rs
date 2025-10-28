#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `LD` reader - Disable the SpaceWire CODEC"]
pub type LdR = crate::BitReader;
#[doc = "Field `LD` writer - Disable the SpaceWire CODEC"]
pub type LdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS` reader - Start the link"]
pub type LsR = crate::BitReader;
#[doc = "Field `LS` writer - Start the link"]
pub type LsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AS` reader - Automatically start the link when a NULL has been received"]
pub type AsR = crate::BitReader;
#[doc = "Field `AS` writer - Automatically start the link when a NULL has been received"]
pub type AsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - If set, an interrupt is generated when one or both of bit 8 to 9 is set and its corresponding event occurs"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - If set, an interrupt is generated when one or both of bit 8 to 9 is set and its corresponding event occurs"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TI` reader - The host can generate a tick by writing a one to this field"]
pub type TiR = crate::BitReader;
#[doc = "Field `TI` writer - The host can generate a tick by writing a one to this field"]
pub type TiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM` reader - Enable Promiscuous mode"]
pub type PmR = crate::BitReader;
#[doc = "Field `PM` writer - Enable Promiscuous mode"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS` reader - Make complete reset of the SpaceWire node. Self-clearing"]
pub type RsR = crate::BitReader;
#[doc = "Field `RS` writer - Make complete reset of the SpaceWire node. Self-clearing"]
pub type RsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TQ` reader - Generate interrupt when a valid time-code is received"]
pub type TqR = crate::BitReader;
#[doc = "Field `TQ` writer - Generate interrupt when a valid time-code is received"]
pub type TqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LI` reader - Generate interrupt when link error occurs"]
pub type LiR = crate::BitReader;
#[doc = "Field `LI` writer - Generate interrupt when link error occurs"]
pub type LiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TT` reader - Enable time-code transmissions"]
pub type TtR = crate::BitReader;
#[doc = "Field `TT` writer - Enable time-code transmissions"]
pub type TtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR` reader - Enable time-code receptions"]
pub type TrR = crate::BitReader;
#[doc = "Field `TR` writer - Enable time-code receptions"]
pub type TrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TF` reader - Time-code Flag Filter"]
pub type TfR = crate::BitReader;
#[doc = "Field `TF` writer - Time-code Flag Filter"]
pub type TfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TL` reader - Transmitter Enable Lock Control"]
pub type TlR = crate::BitReader;
#[doc = "Field `TL` writer - Transmitter Enable Lock Control"]
pub type TlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - SpW Plug-and-Play Enable"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - SpW Plug-and-Play Enable"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - Enable RMAP command handler"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Enable RMAP command handler"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - If set only one RMAP buffer is used"]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - If set only one RMAP buffer is used"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNPA` reader - SpW Plug-and-Play Available"]
pub type PnpaR = crate::FieldReader;
#[doc = "Field `NP` reader - Disable port force"]
pub type NpR = crate::BitReader;
#[doc = "Field `NP` writer - Disable port force"]
pub type NpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - Selects the active port when the no port force bit is zero"]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - Selects the active port when the no port force bit is zero"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LE` reader - Loop-back Enable"]
pub type LeR = crate::BitReader;
#[doc = "Field `LE` writer - Loop-back Enable"]
pub type LeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID` reader - Interrupt distribution available"]
pub type IdR = crate::BitReader;
#[doc = "Field `CC` reader - CCSDS/CCITT CRC-16"]
pub type CcR = crate::BitReader;
#[doc = "Field `PO` reader - The number of available SpaceWire ports minus one"]
pub type PoR = crate::BitReader;
#[doc = "Field `NCH` reader - Number of DMA Channels minus one"]
pub type NchR = crate::FieldReader;
#[doc = "Field `RC` reader - Reads as 1 if RMAP CRC is enabled in the core"]
pub type RcR = crate::BitReader;
#[doc = "Field `RX` reader - Reads as 1 if unaligned writes are available for the receiver"]
pub type RxR = crate::BitReader;
#[doc = "Field `RA` reader - Reads as 1 if the RMAP command handler is available"]
pub type RaR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Disable the SpaceWire CODEC"]
    #[inline(always)]
    pub fn ld(&self) -> LdR {
        LdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start the link"]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatically start the link when a NULL has been received"]
    #[inline(always)]
    pub fn as_(&self) -> AsR {
        AsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If set, an interrupt is generated when one or both of bit 8 to 9 is set and its corresponding event occurs"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The host can generate a tick by writing a one to this field"]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Make complete reset of the SpaceWire node. Self-clearing"]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Generate interrupt when a valid time-code is received"]
    #[inline(always)]
    pub fn tq(&self) -> TqR {
        TqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generate interrupt when link error occurs"]
    #[inline(always)]
    pub fn li(&self) -> LiR {
        LiR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable time-code transmissions"]
    #[inline(always)]
    pub fn tt(&self) -> TtR {
        TtR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable time-code receptions"]
    #[inline(always)]
    pub fn tr(&self) -> TrR {
        TrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Time-code Flag Filter"]
    #[inline(always)]
    pub fn tf(&self) -> TfR {
        TfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmitter Enable Lock Control"]
    #[inline(always)]
    pub fn tl(&self) -> TlR {
        TlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - SpW Plug-and-Play Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable RMAP command handler"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If set only one RMAP buffer is used"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - SpW Plug-and-Play Available"]
    #[inline(always)]
    pub fn pnpa(&self) -> PnpaR {
        PnpaR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Disable port force"]
    #[inline(always)]
    pub fn np(&self) -> NpR {
        NpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Selects the active port when the no port force bit is zero"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Loop-back Enable"]
    #[inline(always)]
    pub fn le(&self) -> LeR {
        LeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt distribution available"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CCSDS/CCITT CRC-16"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The number of available SpaceWire ports minus one"]
    #[inline(always)]
    pub fn po(&self) -> PoR {
        PoR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Number of DMA Channels minus one"]
    #[inline(always)]
    pub fn nch(&self) -> NchR {
        NchR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Reads as 1 if RMAP CRC is enabled in the core"]
    #[inline(always)]
    pub fn rc(&self) -> RcR {
        RcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reads as 1 if unaligned writes are available for the receiver"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reads as 1 if the RMAP command handler is available"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable the SpaceWire CODEC"]
    #[inline(always)]
    pub fn ld(&mut self) -> LdW<'_, CtrlSpec> {
        LdW::new(self, 0)
    }
    #[doc = "Bit 1 - Start the link"]
    #[inline(always)]
    pub fn ls(&mut self) -> LsW<'_, CtrlSpec> {
        LsW::new(self, 1)
    }
    #[doc = "Bit 2 - Automatically start the link when a NULL has been received"]
    #[inline(always)]
    pub fn as_(&mut self) -> AsW<'_, CtrlSpec> {
        AsW::new(self, 2)
    }
    #[doc = "Bit 3 - If set, an interrupt is generated when one or both of bit 8 to 9 is set and its corresponding event occurs"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, CtrlSpec> {
        IeW::new(self, 3)
    }
    #[doc = "Bit 4 - The host can generate a tick by writing a one to this field"]
    #[inline(always)]
    pub fn ti(&mut self) -> TiW<'_, CtrlSpec> {
        TiW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<'_, CtrlSpec> {
        PmW::new(self, 5)
    }
    #[doc = "Bit 6 - Make complete reset of the SpaceWire node. Self-clearing"]
    #[inline(always)]
    pub fn rs(&mut self) -> RsW<'_, CtrlSpec> {
        RsW::new(self, 6)
    }
    #[doc = "Bit 8 - Generate interrupt when a valid time-code is received"]
    #[inline(always)]
    pub fn tq(&mut self) -> TqW<'_, CtrlSpec> {
        TqW::new(self, 8)
    }
    #[doc = "Bit 9 - Generate interrupt when link error occurs"]
    #[inline(always)]
    pub fn li(&mut self) -> LiW<'_, CtrlSpec> {
        LiW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable time-code transmissions"]
    #[inline(always)]
    pub fn tt(&mut self) -> TtW<'_, CtrlSpec> {
        TtW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable time-code receptions"]
    #[inline(always)]
    pub fn tr(&mut self) -> TrW<'_, CtrlSpec> {
        TrW::new(self, 11)
    }
    #[doc = "Bit 12 - Time-code Flag Filter"]
    #[inline(always)]
    pub fn tf(&mut self) -> TfW<'_, CtrlSpec> {
        TfW::new(self, 12)
    }
    #[doc = "Bit 13 - Transmitter Enable Lock Control"]
    #[inline(always)]
    pub fn tl(&mut self) -> TlW<'_, CtrlSpec> {
        TlW::new(self, 13)
    }
    #[doc = "Bit 15 - SpW Plug-and-Play Enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, CtrlSpec> {
        PeW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable RMAP command handler"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<'_, CtrlSpec> {
        ReW::new(self, 16)
    }
    #[doc = "Bit 17 - If set only one RMAP buffer is used"]
    #[inline(always)]
    pub fn rd(&mut self) -> RdW<'_, CtrlSpec> {
        RdW::new(self, 17)
    }
    #[doc = "Bit 20 - Disable port force"]
    #[inline(always)]
    pub fn np(&mut self) -> NpW<'_, CtrlSpec> {
        NpW::new(self, 20)
    }
    #[doc = "Bit 21 - Selects the active port when the no port force bit is zero"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, CtrlSpec> {
        PsW::new(self, 21)
    }
    #[doc = "Bit 22 - Loop-back Enable"]
    #[inline(always)]
    pub fn le(&mut self) -> LeW<'_, CtrlSpec> {
        LeW::new(self, 22)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0xa201_0004"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0xa201_0004;
}
