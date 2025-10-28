#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CHAN_EN` reader - Enables the channel for data collection"]
pub type ChanEnR = crate::FieldReader<u16>;
#[doc = "Field `CHAN_EN` writer - Enables the channel for data collection"]
pub type ChanEnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CHAN_TAG_EN` reader - Enables the channel tag to be saved with the ADC data"]
pub type ChanTagEnR = crate::BitReader;
#[doc = "Field `CHAN_TAG_EN` writer - Enables the channel tag to be saved with the ADC data"]
pub type ChanTagEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWEEP_EN` reader - ADC data acquisition for all enabled channel"]
pub type SweepEnR = crate::BitReader;
#[doc = "Field `SWEEP_EN` writer - ADC data acquisition for all enabled channel"]
pub type SweepEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_TRIG_EN` reader - Allows the external trigger to start analog acquisition"]
pub type ExtTrigEnR = crate::BitReader;
#[doc = "Field `EXT_TRIG_EN` writer - Allows the external trigger to start analog acquisition"]
pub type ExtTrigEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MANUAL_TRIG` reader - Starts analog acquisition"]
pub type ManualTrigR = crate::BitReader;
#[doc = "Field `MANUAL_TRIG` writer - Starts analog acquisition"]
pub type ManualTrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONV_CNT` reader - Conversion count describes the number of conversions to be applied for triggers/sweeps. (N+1 conversions)"]
pub type ConvCntR = crate::FieldReader;
#[doc = "Field `CONV_CNT` writer - Conversion count describes the number of conversions to be applied for triggers/sweeps. (N+1 conversions)"]
pub type ConvCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - Enables the channel for data collection"]
    #[inline(always)]
    pub fn chan_en(&self) -> ChanEnR {
        ChanEnR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enables the channel tag to be saved with the ADC data"]
    #[inline(always)]
    pub fn chan_tag_en(&self) -> ChanTagEnR {
        ChanTagEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC data acquisition for all enabled channel"]
    #[inline(always)]
    pub fn sweep_en(&self) -> SweepEnR {
        SweepEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Allows the external trigger to start analog acquisition"]
    #[inline(always)]
    pub fn ext_trig_en(&self) -> ExtTrigEnR {
        ExtTrigEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Starts analog acquisition"]
    #[inline(always)]
    pub fn manual_trig(&self) -> ManualTrigR {
        ManualTrigR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Conversion count describes the number of conversions to be applied for triggers/sweeps. (N+1 conversions)"]
    #[inline(always)]
    pub fn conv_cnt(&self) -> ConvCntR {
        ConvCntR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Enables the channel for data collection"]
    #[inline(always)]
    pub fn chan_en(&mut self) -> ChanEnW<'_, CtrlSpec> {
        ChanEnW::new(self, 0)
    }
    #[doc = "Bit 16 - Enables the channel tag to be saved with the ADC data"]
    #[inline(always)]
    pub fn chan_tag_en(&mut self) -> ChanTagEnW<'_, CtrlSpec> {
        ChanTagEnW::new(self, 16)
    }
    #[doc = "Bit 17 - ADC data acquisition for all enabled channel"]
    #[inline(always)]
    pub fn sweep_en(&mut self) -> SweepEnW<'_, CtrlSpec> {
        SweepEnW::new(self, 17)
    }
    #[doc = "Bit 18 - Allows the external trigger to start analog acquisition"]
    #[inline(always)]
    pub fn ext_trig_en(&mut self) -> ExtTrigEnW<'_, CtrlSpec> {
        ExtTrigEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Starts analog acquisition"]
    #[inline(always)]
    pub fn manual_trig(&mut self) -> ManualTrigW<'_, CtrlSpec> {
        ManualTrigW::new(self, 19)
    }
    #[doc = "Bits 20:23 - Conversion count describes the number of conversions to be applied for triggers/sweeps. (N+1 conversions)"]
    #[inline(always)]
    pub fn conv_cnt(&mut self) -> ConvCntW<'_, CtrlSpec> {
        ConvCntW::new(self, 20)
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
