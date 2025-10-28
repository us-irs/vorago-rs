#[doc = "Register `RAM0_SBE` reader"]
pub type R = crate::R<Ram0SbeSpec>;
#[doc = "Register `RAM0_SBE` writer"]
pub type W = crate::W<Ram0SbeSpec>;
#[doc = "Field `COUNT` reader - RAM0 EDAC Single Bit Errors"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - RAM0 EDAC Single Bit Errors"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RAM0 EDAC Single Bit Errors"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RAM0 EDAC Single Bit Errors"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, Ram0SbeSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Count of RAM0 EDAC Single Bit Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`ram0_sbe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram0_sbe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ram0SbeSpec;
impl crate::RegisterSpec for Ram0SbeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram0_sbe::R`](R) reader structure"]
impl crate::Readable for Ram0SbeSpec {}
#[doc = "`write(|w| ..)` method takes [`ram0_sbe::W`](W) writer structure"]
impl crate::Writable for Ram0SbeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAM0_SBE to value 0"]
impl crate::Resettable for Ram0SbeSpec {}
