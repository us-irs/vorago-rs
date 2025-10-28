#[doc = "Register `WDOGINTCLR` reader"]
pub type R = crate::R<WdogintclrSpec>;
#[doc = "Register `WDOGINTCLR` writer"]
pub type W = crate::W<WdogintclrSpec>;
#[doc = "Field `CLEAR` reader - Write any value to clear interrupt"]
pub type ClearR = crate::FieldReader<u32>;
#[doc = "Field `CLEAR` writer - Write any value to clear interrupt"]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write any value to clear interrupt"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write any value to clear interrupt"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<'_, WdogintclrSpec> {
        ClearW::new(self, 0)
    }
}
#[doc = "A write of any value clears the WDT module interrupt, and reloads the counter from the value in the WDOGLOAD Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogintclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogintclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogintclrSpec;
impl crate::RegisterSpec for WdogintclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogintclr::R`](R) reader structure"]
impl crate::Readable for WdogintclrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdogintclr::W`](W) writer structure"]
impl crate::Writable for WdogintclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOGINTCLR to value 0"]
impl crate::Resettable for WdogintclrSpec {}
