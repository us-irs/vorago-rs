#[doc = "Register `INTEGRATION_CFG` reader"]
pub type R = crate::R<IntegrationCfgSpec>;
#[doc = "Register `INTEGRATION_CFG` writer"]
pub type W = crate::W<IntegrationCfgSpec>;
#[doc = "Field `INT_TEST_EN` reader - Error Clear"]
pub type IntTestEnR = crate::BitReader;
#[doc = "Field `INT_TEST_EN` writer - Error Clear"]
pub type IntTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error Clear"]
    #[inline(always)]
    pub fn int_test_en(&self) -> IntTestEnR {
        IntTestEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Clear"]
    #[inline(always)]
    pub fn int_test_en(&mut self) -> IntTestEnW<'_, IntegrationCfgSpec> {
        IntTestEnW::new(self, 0)
    }
}
#[doc = "DMA integration configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`integration_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`integration_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntegrationCfgSpec;
impl crate::RegisterSpec for IntegrationCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`integration_cfg::R`](R) reader structure"]
impl crate::Readable for IntegrationCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`integration_cfg::W`](W) writer structure"]
impl crate::Writable for IntegrationCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEGRATION_CFG to value 0"]
impl crate::Resettable for IntegrationCfgSpec {}
