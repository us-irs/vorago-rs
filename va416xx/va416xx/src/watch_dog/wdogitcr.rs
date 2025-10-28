#[doc = "Register `WDOGITCR` reader"]
pub type R = crate::R<WdogitcrSpec>;
#[doc = "Register `WDOGITCR` writer"]
pub type W = crate::W<WdogitcrSpec>;
#[doc = "Field `TEST_MODE_EN` reader - Enable test mode of WDOGINT and WDOGRES"]
pub type TestModeEnR = crate::BitReader;
#[doc = "Field `TEST_MODE_EN` writer - Enable test mode of WDOGINT and WDOGRES"]
pub type TestModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable test mode of WDOGINT and WDOGRES"]
    #[inline(always)]
    pub fn test_mode_en(&self) -> TestModeEnR {
        TestModeEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable test mode of WDOGINT and WDOGRES"]
    #[inline(always)]
    pub fn test_mode_en(&mut self) -> TestModeEnW<'_, WdogitcrSpec> {
        TestModeEnW::new(self, 0)
    }
}
#[doc = "Integration test control\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogitcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogitcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogitcrSpec;
impl crate::RegisterSpec for WdogitcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogitcr::R`](R) reader structure"]
impl crate::Readable for WdogitcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdogitcr::W`](W) writer structure"]
impl crate::Writable for WdogitcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOGITCR to value 0"]
impl crate::Resettable for WdogitcrSpec {}
