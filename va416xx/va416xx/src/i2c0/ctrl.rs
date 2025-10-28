#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CLKENABLED` reader - I2C CLK Enabled"]
pub type ClkenabledR = crate::BitReader;
#[doc = "Field `CLKENABLED` writer - I2C CLK Enabled"]
pub type ClkenabledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - I2C Activated"]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - I2C Activated"]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - I2C Active"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - I2C Active"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEMD` reader - TX FIFIO Empty Mode"]
pub type TxfemdR = crate::BitReader;
#[doc = "Field `TXFEMD` writer - TX FIFIO Empty Mode"]
pub type TxfemdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFMD` reader - RX FIFO Full Mode"]
pub type RxffmdR = crate::BitReader;
#[doc = "Field `RXFFMD` writer - RX FIFO Full Mode"]
pub type RxffmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGFILTER` reader - Enable Input Analog Glitch Filter"]
pub type AlgfilterR = crate::BitReader;
#[doc = "Field `ALGFILTER` writer - Enable Input Analog Glitch Filter"]
pub type AlgfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLGFILTER` reader - Enable Input Digital Glitch Filter"]
pub type DlgfilterR = crate::BitReader;
#[doc = "Field `DLGFILTER` writer - Enable Input Digital Glitch Filter"]
pub type DlgfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - Enable LoopBack Mode"]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Enable LoopBack Mode"]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMCONFIGENB` reader - Enable Timing Config Register"]
pub type TmconfigenbR = crate::BitReader;
#[doc = "Field `TMCONFIGENB` writer - Enable Timing Config Register"]
pub type TmconfigenbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C CLK Enabled"]
    #[inline(always)]
    pub fn clkenabled(&self) -> ClkenabledR {
        ClkenabledR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Activated"]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Active"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FIFIO Empty Mode"]
    #[inline(always)]
    pub fn txfemd(&self) -> TxfemdR {
        TxfemdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Full Mode"]
    #[inline(always)]
    pub fn rxffmd(&self) -> RxffmdR {
        RxffmdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Input Analog Glitch Filter"]
    #[inline(always)]
    pub fn algfilter(&self) -> AlgfilterR {
        AlgfilterR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Input Digital Glitch Filter"]
    #[inline(always)]
    pub fn dlgfilter(&self) -> DlgfilterR {
        DlgfilterR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable LoopBack Mode"]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Timing Config Register"]
    #[inline(always)]
    pub fn tmconfigenb(&self) -> TmconfigenbR {
        TmconfigenbR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C CLK Enabled"]
    #[inline(always)]
    pub fn clkenabled(&mut self) -> ClkenabledW<'_, CtrlSpec> {
        ClkenabledW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Activated"]
    #[inline(always)]
    pub fn enabled(&mut self) -> EnabledW<'_, CtrlSpec> {
        EnabledW::new(self, 1)
    }
    #[doc = "Bit 2 - I2C Active"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlSpec> {
        EnableW::new(self, 2)
    }
    #[doc = "Bit 3 - TX FIFIO Empty Mode"]
    #[inline(always)]
    pub fn txfemd(&mut self) -> TxfemdW<'_, CtrlSpec> {
        TxfemdW::new(self, 3)
    }
    #[doc = "Bit 4 - RX FIFO Full Mode"]
    #[inline(always)]
    pub fn rxffmd(&mut self) -> RxffmdW<'_, CtrlSpec> {
        RxffmdW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Input Analog Glitch Filter"]
    #[inline(always)]
    pub fn algfilter(&mut self) -> AlgfilterW<'_, CtrlSpec> {
        AlgfilterW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Input Digital Glitch Filter"]
    #[inline(always)]
    pub fn dlgfilter(&mut self) -> DlgfilterW<'_, CtrlSpec> {
        DlgfilterW::new(self, 6)
    }
    #[doc = "Bit 8 - Enable LoopBack Mode"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LoopbackW<'_, CtrlSpec> {
        LoopbackW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Timing Config Register"]
    #[inline(always)]
    pub fn tmconfigenb(&mut self) -> TmconfigenbW<'_, CtrlSpec> {
        TmconfigenbW::new(self, 9)
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
