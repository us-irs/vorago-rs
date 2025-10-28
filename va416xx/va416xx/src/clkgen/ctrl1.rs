#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `SYS_CLK_LOST_DET_REARM` reader - Resets/Rearms the SYS_CLK lost detection feature"]
pub type SysClkLostDetRearmR = crate::BitReader;
#[doc = "Field `SYS_CLK_LOST_DET_REARM` writer - Resets/Rearms the SYS_CLK lost detection feature"]
pub type SysClkLostDetRearmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_LCK_DET_REARM` reader - Resets/Rearms the PLL lock detect circuit"]
pub type PllLckDetRearmR = crate::BitReader;
#[doc = "Field `PLL_LCK_DET_REARM` writer - Resets/Rearms the PLL lock detect circuit"]
pub type PllLckDetRearmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_LOST_LOCK_DET_EN` reader - Enables the PLL lock lost detection circuit"]
pub type PllLostLockDetEnR = crate::BitReader;
#[doc = "Field `PLL_LOST_LOCK_DET_EN` writer - Enables the PLL lock lost detection circuit"]
pub type PllLostLockDetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_EN` reader - Enables the crystal oscillator"]
pub type XtalEnR = crate::BitReader;
#[doc = "Field `XTAL_EN` writer - Enables the crystal oscillator"]
pub type XtalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_N_EN` reader - Enables XTAL_N output"]
pub type XtalNEnR = crate::BitReader;
#[doc = "Field `XTAL_N_EN` writer - Enables XTAL_N output"]
pub type XtalNEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_CLK_DIV_SEL` reader - Clock divider select for ADC"]
pub type AdcClkDivSelR = crate::FieldReader;
#[doc = "Field `ADC_CLK_DIV_SEL` writer - Clock divider select for ADC"]
pub type AdcClkDivSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Resets/Rearms the SYS_CLK lost detection feature"]
    #[inline(always)]
    pub fn sys_clk_lost_det_rearm(&self) -> SysClkLostDetRearmR {
        SysClkLostDetRearmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resets/Rearms the PLL lock detect circuit"]
    #[inline(always)]
    pub fn pll_lck_det_rearm(&self) -> PllLckDetRearmR {
        PllLckDetRearmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the PLL lock lost detection circuit"]
    #[inline(always)]
    pub fn pll_lost_lock_det_en(&self) -> PllLostLockDetEnR {
        PllLostLockDetEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the crystal oscillator"]
    #[inline(always)]
    pub fn xtal_en(&self) -> XtalEnR {
        XtalEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables XTAL_N output"]
    #[inline(always)]
    pub fn xtal_n_en(&self) -> XtalNEnR {
        XtalNEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Clock divider select for ADC"]
    #[inline(always)]
    pub fn adc_clk_div_sel(&self) -> AdcClkDivSelR {
        AdcClkDivSelR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Resets/Rearms the SYS_CLK lost detection feature"]
    #[inline(always)]
    pub fn sys_clk_lost_det_rearm(&mut self) -> SysClkLostDetRearmW<'_, Ctrl1Spec> {
        SysClkLostDetRearmW::new(self, 0)
    }
    #[doc = "Bit 1 - Resets/Rearms the PLL lock detect circuit"]
    #[inline(always)]
    pub fn pll_lck_det_rearm(&mut self) -> PllLckDetRearmW<'_, Ctrl1Spec> {
        PllLckDetRearmW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables the PLL lock lost detection circuit"]
    #[inline(always)]
    pub fn pll_lost_lock_det_en(&mut self) -> PllLostLockDetEnW<'_, Ctrl1Spec> {
        PllLostLockDetEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables the crystal oscillator"]
    #[inline(always)]
    pub fn xtal_en(&mut self) -> XtalEnW<'_, Ctrl1Spec> {
        XtalEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables XTAL_N output"]
    #[inline(always)]
    pub fn xtal_n_en(&mut self) -> XtalNEnW<'_, Ctrl1Spec> {
        XtalNEnW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Clock divider select for ADC"]
    #[inline(always)]
    pub fn adc_clk_div_sel(&mut self) -> AdcClkDivSelW<'_, Ctrl1Spec> {
        AdcClkDivSelW::new(self, 5)
    }
}
#[doc = "Clock Generation Module Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {}
