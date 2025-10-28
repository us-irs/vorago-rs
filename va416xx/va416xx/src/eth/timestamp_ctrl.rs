#[doc = "Register `TIMESTAMP_CTRL` reader"]
pub type R = crate::R<TimestampCtrlSpec>;
#[doc = "Register `TIMESTAMP_CTRL` writer"]
pub type W = crate::W<TimestampCtrlSpec>;
#[doc = "Field `TSENA` reader - Timestamp Enable"]
pub type TsenaR = crate::BitReader;
#[doc = "Field `TSENA` writer - Timestamp Enable"]
pub type TsenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCFUPDT` reader - Timestamp Fine or Coarse Update"]
pub type TscfupdtR = crate::BitReader;
#[doc = "Field `TSCFUPDT` writer - Timestamp Fine or Coarse Update"]
pub type TscfupdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSINIT` reader - Timestamp Initialize"]
pub type TsinitR = crate::BitReader;
#[doc = "Field `TSINIT` writer - Timestamp Initialize"]
pub type TsinitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUPDT` reader - Timestamp Update"]
pub type TsupdtR = crate::BitReader;
#[doc = "Field `TSUPDT` writer - Timestamp Update"]
pub type TsupdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTRIG` reader - Timestamp Interrupt Trigger Enable"]
pub type TstrigR = crate::BitReader;
#[doc = "Field `TSTRIG` writer - Timestamp Interrupt Trigger Enable"]
pub type TstrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSADDRREG` reader - Addend Reg Update"]
pub type TsaddrregR = crate::BitReader;
#[doc = "Field `TSADDRREG` writer - Addend Reg Update"]
pub type TsaddrregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENALL` reader - Enable Timestamp for All Frames"]
pub type TsenallR = crate::BitReader;
#[doc = "Field `TSENALL` writer - Enable Timestamp for All Frames"]
pub type TsenallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCTRLSSR` reader - Timestamp Digital or Binary Rollover Control"]
pub type TsctrlssrR = crate::BitReader;
#[doc = "Field `TSCTRLSSR` writer - Timestamp Digital or Binary Rollover Control"]
pub type TsctrlssrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSVER2ENA` reader - Enable PTP packet Processing for Version 2 Format"]
pub type Tsver2enaR = crate::BitReader;
#[doc = "Field `TSVER2ENA` writer - Enable PTP packet Processing for Version 2 Format"]
pub type Tsver2enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPENA` reader - Enable Processing of PTP over Ethernet Frames"]
pub type TsipenaR = crate::BitReader;
#[doc = "Field `TSIPENA` writer - Enable Processing of PTP over Ethernet Frames"]
pub type TsipenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV6ENA` reader - Enable Processing of PTP Frames Sent over IPv6-UDP"]
pub type Tsipv6enaR = crate::BitReader;
#[doc = "Field `TSIPV6ENA` writer - Enable Processing of PTP Frames Sent over IPv6-UDP"]
pub type Tsipv6enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV4ENA` reader - Enable Processing of PTP Frames Sent over IPv4-UDP"]
pub type Tsipv4enaR = crate::BitReader;
#[doc = "Field `TSIPV4ENA` writer - Enable Processing of PTP Frames Sent over IPv4-UDP"]
pub type Tsipv4enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEVNTENA` reader - Enable Timestamp Snapshot for Event Messages"]
pub type TsevntenaR = crate::BitReader;
#[doc = "Field `TSEVNTENA` writer - Enable Timestamp Snapshot for Event Messages"]
pub type TsevntenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSMSTRENA` reader - Enable Snapshot for Messages Relevant to Master"]
pub type TsmstrenaR = crate::BitReader;
#[doc = "Field `TSMSTRENA` writer - Enable Snapshot for Messages Relevant to Master"]
pub type TsmstrenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAPTYPSEL` reader - Select PTP packets for Taking Snapshots"]
pub type SnaptypselR = crate::FieldReader;
#[doc = "Field `SNAPTYPSEL` writer - Select PTP packets for Taking Snapshots"]
pub type SnaptypselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSENMACADDR` reader - Enable MAC address for PTP Frame Filtering"]
pub type TsenmacaddrR = crate::BitReader;
#[doc = "Field `TSENMACADDR` writer - Enable MAC address for PTP Frame Filtering"]
pub type TsenmacaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSFC` reader - Auxiliary Snapshot FIFO Clear"]
pub type AtsfcR = crate::BitReader;
#[doc = "Field `ATSFC` writer - Auxiliary Snapshot FIFO Clear"]
pub type AtsfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN0` reader - Auxiliary Snapshot 0 Enable"]
pub type Atsen0R = crate::BitReader;
#[doc = "Field `ATSEN0` writer - Auxiliary Snapshot 0 Enable"]
pub type Atsen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN1` reader - Auxiliary Snapshot 1 Enable"]
pub type Atsen1R = crate::BitReader;
#[doc = "Field `ATSEN1` writer - Auxiliary Snapshot 1 Enable"]
pub type Atsen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN2` reader - Auxiliary Snapshot 2 Enable"]
pub type Atsen2R = crate::BitReader;
#[doc = "Field `ATSEN2` writer - Auxiliary Snapshot 2 Enable"]
pub type Atsen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN3` reader - Auxiliary Snapshot 3 Enable"]
pub type Atsen3R = crate::BitReader;
#[doc = "Field `ATSEN3` writer - Auxiliary Snapshot 3 Enable"]
pub type Atsen3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn tsena(&self) -> TsenaR {
        TsenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TscfupdtR {
        TscfupdtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn tsinit(&self) -> TsinitR {
        TsinitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn tsupdt(&self) -> TsupdtR {
        TsupdtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn tstrig(&self) -> TstrigR {
        TstrigR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Addend Reg Update"]
    #[inline(always)]
    pub fn tsaddrreg(&self) -> TsaddrregR {
        TsaddrregR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Frames"]
    #[inline(always)]
    pub fn tsenall(&self) -> TsenallR {
        TsenallR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TsctrlssrR {
        TsctrlssrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable PTP packet Processing for Version 2 Format"]
    #[inline(always)]
    pub fn tsver2ena(&self) -> Tsver2enaR {
        Tsver2enaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Frames"]
    #[inline(always)]
    pub fn tsipena(&self) -> TsipenaR {
        TsipenaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent over IPv6-UDP"]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> Tsipv6enaR {
        Tsipv6enaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> Tsipv4enaR {
        Tsipv4enaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn tsevntena(&self) -> TsevntenaR {
        TsevntenaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TsmstrenaR {
        TsmstrenaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SnaptypselR {
        SnaptypselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TsenmacaddrR {
        TsenmacaddrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Auxiliary Snapshot FIFO Clear"]
    #[inline(always)]
    pub fn atsfc(&self) -> AtsfcR {
        AtsfcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Auxiliary Snapshot 0 Enable"]
    #[inline(always)]
    pub fn atsen0(&self) -> Atsen0R {
        Atsen0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Auxiliary Snapshot 1 Enable"]
    #[inline(always)]
    pub fn atsen1(&self) -> Atsen1R {
        Atsen1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Auxiliary Snapshot 2 Enable"]
    #[inline(always)]
    pub fn atsen2(&self) -> Atsen2R {
        Atsen2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Auxiliary Snapshot 3 Enable"]
    #[inline(always)]
    pub fn atsen3(&self) -> Atsen3R {
        Atsen3R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn tsena(&mut self) -> TsenaW<'_, TimestampCtrlSpec> {
        TsenaW::new(self, 0)
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn tscfupdt(&mut self) -> TscfupdtW<'_, TimestampCtrlSpec> {
        TscfupdtW::new(self, 1)
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn tsinit(&mut self) -> TsinitW<'_, TimestampCtrlSpec> {
        TsinitW::new(self, 2)
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn tsupdt(&mut self) -> TsupdtW<'_, TimestampCtrlSpec> {
        TsupdtW::new(self, 3)
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn tstrig(&mut self) -> TstrigW<'_, TimestampCtrlSpec> {
        TstrigW::new(self, 4)
    }
    #[doc = "Bit 5 - Addend Reg Update"]
    #[inline(always)]
    pub fn tsaddrreg(&mut self) -> TsaddrregW<'_, TimestampCtrlSpec> {
        TsaddrregW::new(self, 5)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Frames"]
    #[inline(always)]
    pub fn tsenall(&mut self) -> TsenallW<'_, TimestampCtrlSpec> {
        TsenallW::new(self, 8)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn tsctrlssr(&mut self) -> TsctrlssrW<'_, TimestampCtrlSpec> {
        TsctrlssrW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable PTP packet Processing for Version 2 Format"]
    #[inline(always)]
    pub fn tsver2ena(&mut self) -> Tsver2enaW<'_, TimestampCtrlSpec> {
        Tsver2enaW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Frames"]
    #[inline(always)]
    pub fn tsipena(&mut self) -> TsipenaW<'_, TimestampCtrlSpec> {
        TsipenaW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent over IPv6-UDP"]
    #[inline(always)]
    pub fn tsipv6ena(&mut self) -> Tsipv6enaW<'_, TimestampCtrlSpec> {
        Tsipv6enaW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn tsipv4ena(&mut self) -> Tsipv4enaW<'_, TimestampCtrlSpec> {
        Tsipv4enaW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn tsevntena(&mut self) -> TsevntenaW<'_, TimestampCtrlSpec> {
        TsevntenaW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn tsmstrena(&mut self) -> TsmstrenaW<'_, TimestampCtrlSpec> {
        TsmstrenaW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn snaptypsel(&mut self) -> SnaptypselW<'_, TimestampCtrlSpec> {
        SnaptypselW::new(self, 16)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn tsenmacaddr(&mut self) -> TsenmacaddrW<'_, TimestampCtrlSpec> {
        TsenmacaddrW::new(self, 18)
    }
    #[doc = "Bit 24 - Auxiliary Snapshot FIFO Clear"]
    #[inline(always)]
    pub fn atsfc(&mut self) -> AtsfcW<'_, TimestampCtrlSpec> {
        AtsfcW::new(self, 24)
    }
    #[doc = "Bit 25 - Auxiliary Snapshot 0 Enable"]
    #[inline(always)]
    pub fn atsen0(&mut self) -> Atsen0W<'_, TimestampCtrlSpec> {
        Atsen0W::new(self, 25)
    }
    #[doc = "Bit 26 - Auxiliary Snapshot 1 Enable"]
    #[inline(always)]
    pub fn atsen1(&mut self) -> Atsen1W<'_, TimestampCtrlSpec> {
        Atsen1W::new(self, 26)
    }
    #[doc = "Bit 27 - Auxiliary Snapshot 2 Enable"]
    #[inline(always)]
    pub fn atsen2(&mut self) -> Atsen2W<'_, TimestampCtrlSpec> {
        Atsen2W::new(self, 27)
    }
    #[doc = "Bit 28 - Auxiliary Snapshot 3 Enable"]
    #[inline(always)]
    pub fn atsen3(&mut self) -> Atsen3W<'_, TimestampCtrlSpec> {
        Atsen3W::new(self, 28)
    }
}
#[doc = "Controls the IEEE 1588 timestamp generation and update logic\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timestamp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimestampCtrlSpec;
impl crate::RegisterSpec for TimestampCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp_ctrl::R`](R) reader structure"]
impl crate::Readable for TimestampCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`timestamp_ctrl::W`](W) writer structure"]
impl crate::Writable for TimestampCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMESTAMP_CTRL to value 0"]
impl crate::Resettable for TimestampCtrlSpec {}
