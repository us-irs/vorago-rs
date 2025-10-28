#[doc = "Register `MAC_CONFIG` reader"]
pub type R = crate::R<MacConfigSpec>;
#[doc = "Register `MAC_CONFIG` writer"]
pub type W = crate::W<MacConfigSpec>;
#[doc = "Field `PRELEN` reader - Preamble Length for Transmit frames"]
pub type PrelenR = crate::FieldReader;
#[doc = "Field `PRELEN` writer - Preamble Length for Transmit frames"]
pub type PrelenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RE` reader - Receiver Enable"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Receiver Enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter Enable"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter Enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC` reader - Deferral Check"]
pub type DcR = crate::BitReader;
#[doc = "Field `DC` writer - Deferral Check"]
pub type DcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - Back-Off-Limit"]
pub type BlR = crate::FieldReader;
#[doc = "Field `BL` writer - Back-Off-Limit"]
pub type BlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACS` reader - Automatic Pad, or CRC Stripping"]
pub type AcsR = crate::BitReader;
#[doc = "Field `ACS` writer - Automatic Pad, or CRC Stripping"]
pub type AcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DR` reader - Disable Retry"]
pub type DrR = crate::BitReader;
#[doc = "Field `DR` writer - Disable Retry"]
pub type DrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPC` reader - Checksum Offload"]
pub type IpcR = crate::BitReader;
#[doc = "Field `IPC` writer - Checksum Offload"]
pub type IpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - Duplex Mode"]
pub type DmR = crate::BitReader;
#[doc = "Field `DM` writer - Duplex Mode"]
pub type DmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - Loopback Mode"]
pub type LmR = crate::BitReader;
#[doc = "Field `LM` writer - Loopback Mode"]
pub type LmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRO` reader - Disable Receive Own"]
pub type DroR = crate::BitReader;
#[doc = "Field `DRO` writer - Disable Receive Own"]
pub type DroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - Speed"]
pub type FesR = crate::BitReader;
#[doc = "Field `FES` writer - Speed"]
pub type FesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - Port Select"]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - Port Select"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRS` reader - Disable Carrier Sense During Transmission"]
pub type DcrsR = crate::BitReader;
#[doc = "Field `DCRS` writer - Disable Carrier Sense During Transmission"]
pub type DcrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFG` reader - Inter-Frame Gap"]
pub type IfgR = crate::FieldReader;
#[doc = "Field `IFG` writer - Inter-Frame Gap"]
pub type IfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JE` reader - Jumbo Frame Enable"]
pub type JeR = crate::BitReader;
#[doc = "Field `JE` writer - Jumbo Frame Enable"]
pub type JeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` reader - Frame Burst Enable"]
pub type BeR = crate::BitReader;
#[doc = "Field `BE` writer - Frame Burst Enable"]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JD` reader - Jabber Disable"]
pub type JdR = crate::BitReader;
#[doc = "Field `JD` writer - Jabber Disable"]
pub type JdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD` reader - Watchdog disable"]
pub type WdR = crate::BitReader;
#[doc = "Field `WD` writer - Watchdog disable"]
pub type WdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Preamble Length for Transmit frames"]
    #[inline(always)]
    pub fn prelen(&self) -> PrelenR {
        PrelenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn dc(&self) -> DcR {
        DcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off-Limit"]
    #[inline(always)]
    pub fn bl(&self) -> BlR {
        BlR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic Pad, or CRC Stripping"]
    #[inline(always)]
    pub fn acs(&self) -> AcsR {
        AcsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn ipc(&self) -> IpcR {
        IpcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn lm(&self) -> LmR {
        LmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn dro(&self) -> DroR {
        DroR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn fes(&self) -> FesR {
        FesR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Select"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn dcrs(&self) -> DcrsR {
        DcrsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IfgR {
        IfgR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn je(&self) -> JeR {
        JeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Frame Burst Enable"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&self) -> JdR {
        JdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&self) -> WdR {
        WdR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Preamble Length for Transmit frames"]
    #[inline(always)]
    pub fn prelen(&mut self) -> PrelenW<'_, MacConfigSpec> {
        PrelenW::new(self, 0)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<'_, MacConfigSpec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<'_, MacConfigSpec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn dc(&mut self) -> DcW<'_, MacConfigSpec> {
        DcW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Back-Off-Limit"]
    #[inline(always)]
    pub fn bl(&mut self) -> BlW<'_, MacConfigSpec> {
        BlW::new(self, 5)
    }
    #[doc = "Bit 7 - Automatic Pad, or CRC Stripping"]
    #[inline(always)]
    pub fn acs(&mut self) -> AcsW<'_, MacConfigSpec> {
        AcsW::new(self, 7)
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn dr(&mut self) -> DrW<'_, MacConfigSpec> {
        DrW::new(self, 9)
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn ipc(&mut self) -> IpcW<'_, MacConfigSpec> {
        IpcW::new(self, 10)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&mut self) -> DmW<'_, MacConfigSpec> {
        DmW::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn lm(&mut self) -> LmW<'_, MacConfigSpec> {
        LmW::new(self, 12)
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn dro(&mut self) -> DroW<'_, MacConfigSpec> {
        DroW::new(self, 13)
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn fes(&mut self) -> FesW<'_, MacConfigSpec> {
        FesW::new(self, 14)
    }
    #[doc = "Bit 15 - Port Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, MacConfigSpec> {
        PsW::new(self, 15)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn dcrs(&mut self) -> DcrsW<'_, MacConfigSpec> {
        DcrsW::new(self, 16)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap"]
    #[inline(always)]
    pub fn ifg(&mut self) -> IfgW<'_, MacConfigSpec> {
        IfgW::new(self, 17)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn je(&mut self) -> JeW<'_, MacConfigSpec> {
        JeW::new(self, 20)
    }
    #[doc = "Bit 21 - Frame Burst Enable"]
    #[inline(always)]
    pub fn be(&mut self) -> BeW<'_, MacConfigSpec> {
        BeW::new(self, 21)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn jd(&mut self) -> JdW<'_, MacConfigSpec> {
        JdW::new(self, 22)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&mut self) -> WdW<'_, MacConfigSpec> {
        WdW::new(self, 23)
    }
}
#[doc = "Operation mode register for the MAC\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacConfigSpec;
impl crate::RegisterSpec for MacConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_config::R`](R) reader structure"]
impl crate::Readable for MacConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_config::W`](W) writer structure"]
impl crate::Writable for MacConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_CONFIG to value 0"]
impl crate::Resettable for MacConfigSpec {}
