#[doc = "Register `REFRESH_CONFIG_L` reader"]
pub type R = crate::R<RefreshConfigLSpec>;
#[doc = "Register `REFRESH_CONFIG_L` writer"]
pub type W = crate::W<RefreshConfigLSpec>;
#[doc = "Field `DIVCOUNT` reader - Lower 32-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
pub type DivcountR = crate::FieldReader<u32>;
#[doc = "Field `DIVCOUNT` writer - Lower 32-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
pub type DivcountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Lower 32-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
    #[inline(always)]
    pub fn divcount(&self) -> DivcountR {
        DivcountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lower 32-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
    #[inline(always)]
    pub fn divcount(&mut self) -> DivcountW<'_, RefreshConfigLSpec> {
        DivcountW::new(self, 0)
    }
}
#[doc = "Register Refresh Rate for TMR registers\n\nYou can [`read`](crate::Reg::read) this register and get [`refresh_config_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refresh_config_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefreshConfigLSpec;
impl crate::RegisterSpec for RefreshConfigLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`refresh_config_l::R`](R) reader structure"]
impl crate::Readable for RefreshConfigLSpec {}
#[doc = "`write(|w| ..)` method takes [`refresh_config_l::W`](W) writer structure"]
impl crate::Writable for RefreshConfigLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REFRESH_CONFIG_L to value 0x0f"]
impl crate::Resettable for RefreshConfigLSpec {
    const RESET_VALUE: u32 = 0x0f;
}
