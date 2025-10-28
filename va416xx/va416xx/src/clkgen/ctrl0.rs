#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<Ctrl0Spec>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<Ctrl0Spec>;
#[doc = "Field `REF_CLK_SEL` reader - PLL Reference Clock Select"]
pub type RefClkSelR = crate::FieldReader;
#[doc = "Field `REF_CLK_SEL` writer - PLL Reference Clock Select"]
pub type RefClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKSEL_SYS` reader - Input clock select to PLL"]
pub type ClkselSysR = crate::FieldReader;
#[doc = "Field `CLKSEL_SYS` writer - Input clock select to PLL"]
pub type ClkselSysW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL_INTFB` reader - PLL Symbol; select internal feedback path when high rather than FCLK"]
pub type PllIntfbR = crate::BitReader;
#[doc = "Field `PLL_INTFB` writer - PLL Symbol; select internal feedback path when high rather than FCLK"]
pub type PllIntfbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_PWDN` reader - PLL Symbol; power down when high"]
pub type PllPwdnR = crate::BitReader;
#[doc = "Field `PLL_PWDN` writer - PLL Symbol; power down when high"]
pub type PllPwdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_BYPASS` reader - PLL Symbol; reference-to-output bypass when high"]
pub type PllBypassR = crate::BitReader;
#[doc = "Field `PLL_BYPASS` writer - PLL Symbol; reference-to-output bypass when high"]
pub type PllBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_TEST` reader - PLL Symbol; Reference-to-counters-to-output bypass when high"]
pub type PllTestR = crate::BitReader;
#[doc = "Field `PLL_TEST` writer - PLL Symbol; Reference-to-counters-to-output bypass when high"]
pub type PllTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_BWADJ` reader - PLL Symbol; selects the values 1-64 for the bandwidth divider"]
pub type PllBwadjR = crate::FieldReader;
#[doc = "Field `PLL_BWADJ` writer - PLL Symbol; selects the values 1-64 for the bandwidth divider"]
pub type PllBwadjW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PLL_CLKOD` reader - PLL Symbol; selects the values 1-16 for the post VCO divider"]
pub type PllClkodR = crate::FieldReader;
#[doc = "Field `PLL_CLKOD` writer - PLL Symbol; selects the values 1-16 for the post VCO divider"]
pub type PllClkodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLL_CLKF` reader - PLL Symbol; selects the values 1-64 for the multiplication factor"]
pub type PllClkfR = crate::FieldReader;
#[doc = "Field `PLL_CLKF` writer - PLL Symbol; selects the values 1-64 for the multiplication factor"]
pub type PllClkfW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PLL_CLKR` reader - PLL Symbol; selects the values 1-16 for the reference divider"]
pub type PllClkrR = crate::FieldReader;
#[doc = "Field `PLL_CLKR` writer - PLL Symbol; selects the values 1-16 for the reference divider"]
pub type PllClkrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLK_DIV_SEL` reader - Selects the PLL out divider to divide by 1/2/4/8"]
pub type ClkDivSelR = crate::FieldReader;
#[doc = "Field `CLK_DIV_SEL` writer - Selects the PLL out divider to divide by 1/2/4/8"]
pub type ClkDivSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL_RESET` reader - Writing this bit to 1 puts the PLL into reset"]
pub type PllResetR = crate::BitReader;
#[doc = "Field `PLL_RESET` writer - Writing this bit to 1 puts the PLL into reset"]
pub type PllResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_CLK_LOST_DET_EN` reader - Enable the circuit that detects loss of SYS_CLK"]
pub type SysClkLostDetEnR = crate::BitReader;
#[doc = "Field `SYS_CLK_LOST_DET_EN` writer - Enable the circuit that detects loss of SYS_CLK"]
pub type SysClkLostDetEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - PLL Reference Clock Select"]
    #[inline(always)]
    pub fn ref_clk_sel(&self) -> RefClkSelR {
        RefClkSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input clock select to PLL"]
    #[inline(always)]
    pub fn clksel_sys(&self) -> ClkselSysR {
        ClkselSysR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PLL Symbol; select internal feedback path when high rather than FCLK"]
    #[inline(always)]
    pub fn pll_intfb(&self) -> PllIntfbR {
        PllIntfbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL Symbol; power down when high"]
    #[inline(always)]
    pub fn pll_pwdn(&self) -> PllPwdnR {
        PllPwdnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL Symbol; reference-to-output bypass when high"]
    #[inline(always)]
    pub fn pll_bypass(&self) -> PllBypassR {
        PllBypassR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL Symbol; Reference-to-counters-to-output bypass when high"]
    #[inline(always)]
    pub fn pll_test(&self) -> PllTestR {
        PllTestR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - PLL Symbol; selects the values 1-64 for the bandwidth divider"]
    #[inline(always)]
    pub fn pll_bwadj(&self) -> PllBwadjR {
        PllBwadjR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:17 - PLL Symbol; selects the values 1-16 for the post VCO divider"]
    #[inline(always)]
    pub fn pll_clkod(&self) -> PllClkodR {
        PllClkodR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:23 - PLL Symbol; selects the values 1-64 for the multiplication factor"]
    #[inline(always)]
    pub fn pll_clkf(&self) -> PllClkfR {
        PllClkfR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - PLL Symbol; selects the values 1-16 for the reference divider"]
    #[inline(always)]
    pub fn pll_clkr(&self) -> PllClkrR {
        PllClkrR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Selects the PLL out divider to divide by 1/2/4/8"]
    #[inline(always)]
    pub fn clk_div_sel(&self) -> ClkDivSelR {
        ClkDivSelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Writing this bit to 1 puts the PLL into reset"]
    #[inline(always)]
    pub fn pll_reset(&self) -> PllResetR {
        PllResetR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable the circuit that detects loss of SYS_CLK"]
    #[inline(always)]
    pub fn sys_clk_lost_det_en(&self) -> SysClkLostDetEnR {
        SysClkLostDetEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL Reference Clock Select"]
    #[inline(always)]
    pub fn ref_clk_sel(&mut self) -> RefClkSelW<'_, Ctrl0Spec> {
        RefClkSelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input clock select to PLL"]
    #[inline(always)]
    pub fn clksel_sys(&mut self) -> ClkselSysW<'_, Ctrl0Spec> {
        ClkselSysW::new(self, 2)
    }
    #[doc = "Bit 4 - PLL Symbol; select internal feedback path when high rather than FCLK"]
    #[inline(always)]
    pub fn pll_intfb(&mut self) -> PllIntfbW<'_, Ctrl0Spec> {
        PllIntfbW::new(self, 4)
    }
    #[doc = "Bit 5 - PLL Symbol; power down when high"]
    #[inline(always)]
    pub fn pll_pwdn(&mut self) -> PllPwdnW<'_, Ctrl0Spec> {
        PllPwdnW::new(self, 5)
    }
    #[doc = "Bit 6 - PLL Symbol; reference-to-output bypass when high"]
    #[inline(always)]
    pub fn pll_bypass(&mut self) -> PllBypassW<'_, Ctrl0Spec> {
        PllBypassW::new(self, 6)
    }
    #[doc = "Bit 7 - PLL Symbol; Reference-to-counters-to-output bypass when high"]
    #[inline(always)]
    pub fn pll_test(&mut self) -> PllTestW<'_, Ctrl0Spec> {
        PllTestW::new(self, 7)
    }
    #[doc = "Bits 8:13 - PLL Symbol; selects the values 1-64 for the bandwidth divider"]
    #[inline(always)]
    pub fn pll_bwadj(&mut self) -> PllBwadjW<'_, Ctrl0Spec> {
        PllBwadjW::new(self, 8)
    }
    #[doc = "Bits 14:17 - PLL Symbol; selects the values 1-16 for the post VCO divider"]
    #[inline(always)]
    pub fn pll_clkod(&mut self) -> PllClkodW<'_, Ctrl0Spec> {
        PllClkodW::new(self, 14)
    }
    #[doc = "Bits 18:23 - PLL Symbol; selects the values 1-64 for the multiplication factor"]
    #[inline(always)]
    pub fn pll_clkf(&mut self) -> PllClkfW<'_, Ctrl0Spec> {
        PllClkfW::new(self, 18)
    }
    #[doc = "Bits 24:27 - PLL Symbol; selects the values 1-16 for the reference divider"]
    #[inline(always)]
    pub fn pll_clkr(&mut self) -> PllClkrW<'_, Ctrl0Spec> {
        PllClkrW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Selects the PLL out divider to divide by 1/2/4/8"]
    #[inline(always)]
    pub fn clk_div_sel(&mut self) -> ClkDivSelW<'_, Ctrl0Spec> {
        ClkDivSelW::new(self, 28)
    }
    #[doc = "Bit 30 - Writing this bit to 1 puts the PLL into reset"]
    #[inline(always)]
    pub fn pll_reset(&mut self) -> PllResetW<'_, Ctrl0Spec> {
        PllResetW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable the circuit that detects loss of SYS_CLK"]
    #[inline(always)]
    pub fn sys_clk_lost_det_en(&mut self) -> SysClkLostDetEnW<'_, Ctrl0Spec> {
        SysClkLostDetEnW::new(self, 31)
    }
}
#[doc = "Clock Generation Module Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl0Spec;
impl crate::RegisterSpec for Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for Ctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL0 to value 0x30"]
impl crate::Resettable for Ctrl0Spec {
    const RESET_VALUE: u32 = 0x30;
}
