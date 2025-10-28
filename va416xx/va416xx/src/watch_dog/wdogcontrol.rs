#[doc = "Register `WDOGCONTROL` reader"]
pub type R = crate::R<WdogcontrolSpec>;
#[doc = "Register `WDOGCONTROL` writer"]
pub type W = crate::W<WdogcontrolSpec>;
#[doc = "Field `INTEN` reader - Enable watchdog interrupt"]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - Enable watchdog interrupt"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESEN` reader - Enable watchdog reset output"]
pub type ResenR = crate::BitReader;
#[doc = "Field `RESEN` writer - Enable watchdog reset output"]
pub type ResenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable watchdog interrupt"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable watchdog reset output"]
    #[inline(always)]
    pub fn resen(&self) -> ResenR {
        ResenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable watchdog interrupt"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<'_, WdogcontrolSpec> {
        IntenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable watchdog reset output"]
    #[inline(always)]
    pub fn resen(&mut self) -> ResenW<'_, WdogcontrolSpec> {
        ResenW::new(self, 1)
    }
}
#[doc = "Enable for block reset and interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogcontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogcontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogcontrolSpec;
impl crate::RegisterSpec for WdogcontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogcontrol::R`](R) reader structure"]
impl crate::Readable for WdogcontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`wdogcontrol::W`](W) writer structure"]
impl crate::Writable for WdogcontrolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOGCONTROL to value 0"]
impl crate::Resettable for WdogcontrolSpec {}
