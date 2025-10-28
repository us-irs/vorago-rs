#[doc = "Register `DMA_OPER_MODE` reader"]
pub type R = crate::R<DmaOperModeSpec>;
#[doc = "Register `DMA_OPER_MODE` writer"]
pub type W = crate::W<DmaOperModeSpec>;
#[doc = "Field `SR` reader - Start or Stop Receive"]
pub type SrR = crate::BitReader;
#[doc = "Field `SR` writer - Start or Stop Receive"]
pub type SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSF` reader - Operate on Second Frame"]
pub type OsfR = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on Second Frame"]
pub type OsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - Receive Threshold Control"]
pub type RtcR = crate::FieldReader;
#[doc = "Field `RTC` writer - Receive Threshold Control"]
pub type RtcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DGF` reader - Drop Giant Frames"]
pub type DgfR = crate::BitReader;
#[doc = "Field `DGF` writer - Drop Giant Frames"]
pub type DgfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUF` reader - Forward Undersized Good Frames"]
pub type FufR = crate::BitReader;
#[doc = "Field `FUF` writer - Forward Undersized Good Frames"]
pub type FufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEF` reader - Forward Error Frames"]
pub type FefR = crate::BitReader;
#[doc = "Field `FEF` writer - Forward Error Frames"]
pub type FefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFA` reader - Threshold for Activating Flow Control"]
pub type RfaR = crate::FieldReader;
#[doc = "Field `RFA` writer - Threshold for Activating Flow Control"]
pub type RfaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RFD` reader - Threshold for Deactivating Flow Control"]
pub type RfdR = crate::FieldReader;
#[doc = "Field `RFD` writer - Threshold for Deactivating Flow Control"]
pub type RfdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ST` reader - Start or Stop Transmission Command"]
pub type StR = crate::BitReader;
#[doc = "Field `ST` writer - Start or Stop Transmission Command"]
pub type StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTC` reader - Transmit Threshold Control"]
pub type TtcR = crate::FieldReader;
#[doc = "Field `TTC` writer - Transmit Threshold Control"]
pub type TtcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FTF` reader - Flush Transmit FIFO"]
pub type FtfR = crate::BitReader;
#[doc = "Field `FTF` writer - Flush Transmit FIFO"]
pub type FtfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - Transmit Store and Forward"]
pub type TsfR = crate::BitReader;
#[doc = "Field `TSF` writer - Transmit Store and Forward"]
pub type TsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFF` reader - Disable Flushing of Received Frames"]
pub type DffR = crate::BitReader;
#[doc = "Field `DFF` writer - Disable Flushing of Received Frames"]
pub type DffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - Receive Store and Forward"]
pub type RsfR = crate::BitReader;
#[doc = "Field `RSF` writer - Receive Store and Forward"]
pub type RsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT` reader - Disable Dropping of TCP/IP Checksum Error Frames"]
pub type DtR = crate::BitReader;
#[doc = "Field `DT` writer - Disable Dropping of TCP/IP Checksum Error Frames"]
pub type DtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Start or Stop Receive"]
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on Second Frame"]
    #[inline(always)]
    pub fn osf(&self) -> OsfR {
        OsfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Drop Giant Frames"]
    #[inline(always)]
    pub fn dgf(&self) -> DgfR {
        DgfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames"]
    #[inline(always)]
    pub fn fuf(&self) -> FufR {
        FufR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward Error Frames"]
    #[inline(always)]
    pub fn fef(&self) -> FefR {
        FefR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Threshold for Activating Flow Control"]
    #[inline(always)]
    pub fn rfa(&self) -> RfaR {
        RfaR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Threshold for Deactivating Flow Control"]
    #[inline(always)]
    pub fn rfd(&self) -> RfdR {
        RfdR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control"]
    #[inline(always)]
    pub fn ttc(&self) -> TtcR {
        TtcR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&self) -> FtfR {
        FtfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames"]
    #[inline(always)]
    pub fn dff(&self) -> DffR {
        DffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive Store and Forward"]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start or Stop Receive"]
    #[inline(always)]
    pub fn sr(&mut self) -> SrW<'_, DmaOperModeSpec> {
        SrW::new(self, 1)
    }
    #[doc = "Bit 2 - Operate on Second Frame"]
    #[inline(always)]
    pub fn osf(&mut self) -> OsfW<'_, DmaOperModeSpec> {
        OsfW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, DmaOperModeSpec> {
        RtcW::new(self, 3)
    }
    #[doc = "Bit 5 - Drop Giant Frames"]
    #[inline(always)]
    pub fn dgf(&mut self) -> DgfW<'_, DmaOperModeSpec> {
        DgfW::new(self, 5)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames"]
    #[inline(always)]
    pub fn fuf(&mut self) -> FufW<'_, DmaOperModeSpec> {
        FufW::new(self, 6)
    }
    #[doc = "Bit 7 - Forward Error Frames"]
    #[inline(always)]
    pub fn fef(&mut self) -> FefW<'_, DmaOperModeSpec> {
        FefW::new(self, 7)
    }
    #[doc = "Bits 9:10 - Threshold for Activating Flow Control"]
    #[inline(always)]
    pub fn rfa(&mut self) -> RfaW<'_, DmaOperModeSpec> {
        RfaW::new(self, 9)
    }
    #[doc = "Bits 11:12 - Threshold for Deactivating Flow Control"]
    #[inline(always)]
    pub fn rfd(&mut self) -> RfdW<'_, DmaOperModeSpec> {
        RfdW::new(self, 11)
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn st(&mut self) -> StW<'_, DmaOperModeSpec> {
        StW::new(self, 13)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control"]
    #[inline(always)]
    pub fn ttc(&mut self) -> TtcW<'_, DmaOperModeSpec> {
        TtcW::new(self, 14)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&mut self) -> FtfW<'_, DmaOperModeSpec> {
        FtfW::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TsfW<'_, DmaOperModeSpec> {
        TsfW::new(self, 21)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames"]
    #[inline(always)]
    pub fn dff(&mut self) -> DffW<'_, DmaOperModeSpec> {
        DffW::new(self, 24)
    }
    #[doc = "Bit 25 - Receive Store and Forward"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RsfW<'_, DmaOperModeSpec> {
        RsfW::new(self, 25)
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<'_, DmaOperModeSpec> {
        DtW::new(self, 26)
    }
}
#[doc = "Sets the Receive and Transmit operation mode and command\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_oper_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_oper_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaOperModeSpec;
impl crate::RegisterSpec for DmaOperModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_oper_mode::R`](R) reader structure"]
impl crate::Readable for DmaOperModeSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_oper_mode::W`](W) writer structure"]
impl crate::Writable for DmaOperModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_OPER_MODE to value 0"]
impl crate::Resettable for DmaOperModeSpec {}
