#[doc = "Register `RAM0_MBE` reader"]
pub type R = crate::R<Ram0MbeSpec>;
#[doc = "Register `RAM0_MBE` writer"]
pub type W = crate::W<Ram0MbeSpec>;
#[doc = "Field `COUNT` reader - RAM0 Multi Bit Errors"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - RAM0 Multi Bit Errors"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RAM0 Multi Bit Errors"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RAM0 Multi Bit Errors"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, Ram0MbeSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Count of RAM0 EDAC Multi Bit Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`ram0_mbe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram0_mbe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ram0MbeSpec;
impl crate::RegisterSpec for Ram0MbeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram0_mbe::R`](R) reader structure"]
impl crate::Readable for Ram0MbeSpec {}
#[doc = "`write(|w| ..)` method takes [`ram0_mbe::W`](W) writer structure"]
impl crate::Writable for Ram0MbeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAM0_MBE to value 0"]
impl crate::Resettable for Ram0MbeSpec {}
