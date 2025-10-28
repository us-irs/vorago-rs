#[doc = "Register `S0_CTRL` reader"]
pub type R = crate::R<S0CtrlSpec>;
#[doc = "Register `S0_CTRL` writer"]
pub type W = crate::W<S0CtrlSpec>;
#[doc = "Field `CLKENABLED` reader - I2C Enabled"]
pub type ClkenabledR = crate::BitReader;
#[doc = "Field `CLKENABLED` writer - I2C Enabled"]
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
impl R {
    #[doc = "Bit 0 - I2C Enabled"]
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
}
impl W {
    #[doc = "Bit 0 - I2C Enabled"]
    #[inline(always)]
    pub fn clkenabled(&mut self) -> ClkenabledW<'_, S0CtrlSpec> {
        ClkenabledW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Activated"]
    #[inline(always)]
    pub fn enabled(&mut self) -> EnabledW<'_, S0CtrlSpec> {
        EnabledW::new(self, 1)
    }
    #[doc = "Bit 2 - I2C Active"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, S0CtrlSpec> {
        EnableW::new(self, 2)
    }
    #[doc = "Bit 3 - TX FIFIO Empty Mode"]
    #[inline(always)]
    pub fn txfemd(&mut self) -> TxfemdW<'_, S0CtrlSpec> {
        TxfemdW::new(self, 3)
    }
    #[doc = "Bit 4 - RX FIFO Full Mode"]
    #[inline(always)]
    pub fn rxffmd(&mut self) -> RxffmdW<'_, S0CtrlSpec> {
        RxffmdW::new(self, 4)
    }
}
#[doc = "Slave Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0CtrlSpec;
impl crate::RegisterSpec for S0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_ctrl::R`](R) reader structure"]
impl crate::Readable for S0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_ctrl::W`](W) writer structure"]
impl crate::Writable for S0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S0_CTRL to value 0"]
impl crate::Resettable for S0CtrlSpec {}
