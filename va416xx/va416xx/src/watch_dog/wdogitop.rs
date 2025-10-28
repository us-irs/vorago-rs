#[doc = "Register `WDOGITOP` reader"]
pub type R = crate::R<WdogitopSpec>;
#[doc = "Register `WDOGITOP` writer"]
pub type W = crate::W<WdogitopSpec>;
#[doc = "Field `WDOGRES` reader - Set output value"]
pub type WdogresR = crate::BitReader;
#[doc = "Field `WDOGRES` writer - Set output value"]
pub type WdogresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOGINT` reader - Set output value"]
pub type WdogintR = crate::BitReader;
#[doc = "Field `WDOGINT` writer - Set output value"]
pub type WdogintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set output value"]
    #[inline(always)]
    pub fn wdogres(&self) -> WdogresR {
        WdogresR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set output value"]
    #[inline(always)]
    pub fn wdogint(&self) -> WdogintR {
        WdogintR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set output value"]
    #[inline(always)]
    pub fn wdogres(&mut self) -> WdogresW<'_, WdogitopSpec> {
        WdogresW::new(self, 0)
    }
    #[doc = "Bit 1 - Set output value"]
    #[inline(always)]
    pub fn wdogint(&mut self) -> WdogintW<'_, WdogitopSpec> {
        WdogintW::new(self, 1)
    }
}
#[doc = "Integration test output set\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogitop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogitop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogitopSpec;
impl crate::RegisterSpec for WdogitopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogitop::R`](R) reader structure"]
impl crate::Readable for WdogitopSpec {}
#[doc = "`write(|w| ..)` method takes [`wdogitop::W`](W) writer structure"]
impl crate::Writable for WdogitopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOGITOP to value 0"]
impl crate::Resettable for WdogitopSpec {}
