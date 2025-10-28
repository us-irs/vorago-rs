#[doc = "Register `CLKSCALE` reader"]
pub type R = crate::R<ClkscaleSpec>;
#[doc = "Register `CLKSCALE` writer"]
pub type W = crate::W<ClkscaleSpec>;
#[doc = "Field `FRAC` reader - Fractional Divide (64ths)"]
pub type FracR = crate::FieldReader;
#[doc = "Field `FRAC` writer - Fractional Divide (64ths)"]
pub type FracW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `INT` reader - Integer Divide"]
pub type IntR = crate::FieldReader<u32>;
#[doc = "Field `INT` writer - Integer Divide"]
pub type IntW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `RESET` writer - Reset Baud Counter"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Fractional Divide (64ths)"]
    #[inline(always)]
    pub fn frac(&self) -> FracR {
        FracR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:23 - Integer Divide"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits >> 6) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fractional Divide (64ths)"]
    #[inline(always)]
    pub fn frac(&mut self) -> FracW<'_, ClkscaleSpec> {
        FracW::new(self, 0)
    }
    #[doc = "Bits 6:23 - Integer Divide"]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<'_, ClkscaleSpec> {
        IntW::new(self, 6)
    }
    #[doc = "Bit 31 - Reset Baud Counter"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, ClkscaleSpec> {
        ResetW::new(self, 31)
    }
}
#[doc = "Clock Scale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkscale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkscale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkscaleSpec;
impl crate::RegisterSpec for ClkscaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkscale::R`](R) reader structure"]
impl crate::Readable for ClkscaleSpec {}
#[doc = "`write(|w| ..)` method takes [`clkscale::W`](W) writer structure"]
impl crate::Writable for ClkscaleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKSCALE to value 0"]
impl crate::Resettable for ClkscaleSpec {}
