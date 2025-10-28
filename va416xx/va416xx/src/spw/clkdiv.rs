#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<ClkdivSpec>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<ClkdivSpec>;
#[doc = "Field `CLKDIVRUN` reader - 8-bit Clock divisor value used for the clock-divider when the link-interface is in the run-state"]
pub type ClkdivrunR = crate::FieldReader;
#[doc = "Field `CLKDIVRUN` writer - 8-bit Clock divisor value used for the clock-divider when the link-interface is in the run-state"]
pub type ClkdivrunW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLKDIVSTART` reader - 8-bit Clock divisor value used for the clock-divider during startup"]
pub type ClkdivstartR = crate::FieldReader;
#[doc = "Field `CLKDIVSTART` writer - 8-bit Clock divisor value used for the clock-divider during startup"]
pub type ClkdivstartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit Clock divisor value used for the clock-divider when the link-interface is in the run-state"]
    #[inline(always)]
    pub fn clkdivrun(&self) -> ClkdivrunR {
        ClkdivrunR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit Clock divisor value used for the clock-divider during startup"]
    #[inline(always)]
    pub fn clkdivstart(&self) -> ClkdivstartR {
        ClkdivstartR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit Clock divisor value used for the clock-divider when the link-interface is in the run-state"]
    #[inline(always)]
    pub fn clkdivrun(&mut self) -> ClkdivrunW<'_, ClkdivSpec> {
        ClkdivrunW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit Clock divisor value used for the clock-divider during startup"]
    #[inline(always)]
    pub fn clkdivstart(&mut self) -> ClkdivstartW<'_, ClkdivSpec> {
        ClkdivstartW::new(self, 8)
    }
}
#[doc = "Clock Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkdivSpec;
impl crate::RegisterSpec for ClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for ClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for ClkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKDIV to value 0x0909"]
impl crate::Resettable for ClkdivSpec {
    const RESET_VALUE: u32 = 0x0909;
}
