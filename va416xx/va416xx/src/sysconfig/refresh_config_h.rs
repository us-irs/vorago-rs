#[doc = "Register `REFRESH_CONFIG_H` reader"]
pub type R = crate::R<RefreshConfigHSpec>;
#[doc = "Register `REFRESH_CONFIG_H` writer"]
pub type W = crate::W<RefreshConfigHSpec>;
#[doc = "Field `DIVCOUNT` reader - Upper 8-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
pub type DivcountR = crate::FieldReader;
#[doc = "Field `DIVCOUNT` writer - Upper 8-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
pub type DivcountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TESTMODE` reader - Special Test Mode Configuration. 00/01=normal. 10=Force refresh off. 11=Force refresh on constantly."]
pub type TestmodeR = crate::FieldReader;
#[doc = "Field `TESTMODE` writer - Special Test Mode Configuration. 00/01=normal. 10=Force refresh off. 11=Force refresh on constantly."]
pub type TestmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Upper 8-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
    #[inline(always)]
    pub fn divcount(&self) -> DivcountR {
        DivcountR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 30:31 - Special Test Mode Configuration. 00/01=normal. 10=Force refresh off. 11=Force refresh on constantly."]
    #[inline(always)]
    pub fn testmode(&self) -> TestmodeR {
        TestmodeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Upper 8-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
    #[inline(always)]
    pub fn divcount(&mut self) -> DivcountW<'_, RefreshConfigHSpec> {
        DivcountW::new(self, 0)
    }
    #[doc = "Bits 30:31 - Special Test Mode Configuration. 00/01=normal. 10=Force refresh off. 11=Force refresh on constantly."]
    #[inline(always)]
    pub fn testmode(&mut self) -> TestmodeW<'_, RefreshConfigHSpec> {
        TestmodeW::new(self, 30)
    }
}
#[doc = "Register Refresh Rate for TMR registers\n\nYou can [`read`](crate::Reg::read) this register and get [`refresh_config_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refresh_config_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefreshConfigHSpec;
impl crate::RegisterSpec for RefreshConfigHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`refresh_config_h::R`](R) reader structure"]
impl crate::Readable for RefreshConfigHSpec {}
#[doc = "`write(|w| ..)` method takes [`refresh_config_h::W`](W) writer structure"]
impl crate::Writable for RefreshConfigHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REFRESH_CONFIG_H to value 0"]
impl crate::Resettable for RefreshConfigHSpec {}
