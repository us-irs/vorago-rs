#[doc = "Register `WDOGLOCK` reader"]
pub type R = crate::R<WdoglockSpec>;
#[doc = "Register `WDOGLOCK` writer"]
pub type W = crate::W<WdoglockSpec>;
#[doc = "Field `REG_WR_EN` reader - Register write enable status"]
pub type RegWrEnR = crate::FieldReader<u32>;
#[doc = "Field `REG_WR_EN` writer - Register write enable status"]
pub type RegWrEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register write enable status"]
    #[inline(always)]
    pub fn reg_wr_en(&self) -> RegWrEnR {
        RegWrEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register write enable status"]
    #[inline(always)]
    pub fn reg_wr_en(&mut self) -> RegWrEnW<'_, WdoglockSpec> {
        RegWrEnW::new(self, 0)
    }
}
#[doc = "Lock\n\nYou can [`read`](crate::Reg::read) this register and get [`wdoglock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdoglock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdoglockSpec;
impl crate::RegisterSpec for WdoglockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdoglock::R`](R) reader structure"]
impl crate::Readable for WdoglockSpec {}
#[doc = "`write(|w| ..)` method takes [`wdoglock::W`](W) writer structure"]
impl crate::Writable for WdoglockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOGLOCK to value 0"]
impl crate::Resettable for WdoglockSpec {}
