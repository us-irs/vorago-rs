#[doc = "Register `WDOGLOAD` reader"]
pub type R = crate::R<WdogloadSpec>;
#[doc = "Register `WDOGLOAD` writer"]
pub type W = crate::W<WdogloadSpec>;
#[doc = "Field `CNT` reader - Count to load"]
pub type CntR = crate::FieldReader<u32>;
#[doc = "Field `CNT` writer - Count to load"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Count to load"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Count to load"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, WdogloadSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Counter Start Value\n\nYou can [`read`](crate::Reg::read) this register and get [`wdogload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogloadSpec;
impl crate::RegisterSpec for WdogloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogload::R`](R) reader structure"]
impl crate::Readable for WdogloadSpec {}
#[doc = "`write(|w| ..)` method takes [`wdogload::W`](W) writer structure"]
impl crate::Writable for WdogloadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOGLOAD to value 0xffff_ffff"]
impl crate::Resettable for WdogloadSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
