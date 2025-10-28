#[doc = "Register `MAC_FLOW_CTRL` reader"]
pub type R = crate::R<MacFlowCtrlSpec>;
#[doc = "Register `MAC_FLOW_CTRL` writer"]
pub type W = crate::W<MacFlowCtrlSpec>;
#[doc = "Field `FCB_BPA` reader - Flow Control Busy or Backpressure Activate"]
pub type FcbBpaR = crate::BitReader;
#[doc = "Field `FCB_BPA` writer - Flow Control Busy or Backpressure Activate"]
pub type FcbBpaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - Transmit Flow Control Enable"]
pub type TfeR = crate::BitReader;
#[doc = "Field `TFE` writer - Transmit Flow Control Enable"]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFE` reader - Receive Flow Control Enable"]
pub type RfeR = crate::BitReader;
#[doc = "Field `RFE` writer - Receive Flow Control Enable"]
pub type RfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UP` reader - Unicast Pause Frame Detect"]
pub type UpR = crate::BitReader;
#[doc = "Field `UP` writer - Unicast Pause Frame Detect"]
pub type UpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLT` reader - Pause Low Threshold"]
pub type PltR = crate::FieldReader;
#[doc = "Field `PLT` writer - Pause Low Threshold"]
pub type PltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DZPQ` reader - Disable Zero-Quanta Pause"]
pub type DzpqR = crate::BitReader;
#[doc = "Field `DZPQ` writer - Disable Zero-Quanta Pause"]
pub type DzpqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT` reader - Pause time"]
pub type PtR = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pause time"]
pub type PtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FcbBpaR {
        FcbBpaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PltR {
        PltR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn dzpq(&self) -> DzpqR {
        DzpqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause time"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    pub fn fcb_bpa(&mut self) -> FcbBpaW<'_, MacFlowCtrlSpec> {
        FcbBpaW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn tfe(&mut self) -> TfeW<'_, MacFlowCtrlSpec> {
        TfeW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn rfe(&mut self) -> RfeW<'_, MacFlowCtrlSpec> {
        RfeW::new(self, 2)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn up(&mut self) -> UpW<'_, MacFlowCtrlSpec> {
        UpW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn plt(&mut self) -> PltW<'_, MacFlowCtrlSpec> {
        PltW::new(self, 4)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn dzpq(&mut self) -> DzpqW<'_, MacFlowCtrlSpec> {
        DzpqW::new(self, 7)
    }
    #[doc = "Bits 16:31 - Pause time"]
    #[inline(always)]
    pub fn pt(&mut self) -> PtW<'_, MacFlowCtrlSpec> {
        PtW::new(self, 16)
    }
}
#[doc = "Controls the generation of control frames\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_flow_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_flow_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacFlowCtrlSpec;
impl crate::RegisterSpec for MacFlowCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_flow_ctrl::R`](R) reader structure"]
impl crate::Readable for MacFlowCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_flow_ctrl::W`](W) writer structure"]
impl crate::Writable for MacFlowCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_FLOW_CTRL to value 0"]
impl crate::Resettable for MacFlowCtrlSpec {}
