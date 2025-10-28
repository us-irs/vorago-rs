#[doc = "Register `RAM_TRAP_SYND0` reader"]
pub type R = crate::R<RamTrapSynd0Spec>;
#[doc = "Register `RAM_TRAP_SYND0` writer"]
pub type W = crate::W<RamTrapSynd0Spec>;
#[doc = "Field `RAM_SYND_7_0` reader - 6-bit syndrome value for bits 15-0"]
pub type RamSynd7_0R = crate::FieldReader;
#[doc = "Field `RAM_SYND_7_0` writer - 6-bit syndrome value for bits 15-0"]
pub type RamSynd7_0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RAM_SYND_31_16` reader - 6-bit syndrome value for bits 31-16"]
pub type RamSynd31_16R = crate::FieldReader;
#[doc = "Field `RAM_SYND_31_16` writer - 6-bit syndrome value for bits 31-16"]
pub type RamSynd31_16W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 6-bit syndrome value for bits 15-0"]
    #[inline(always)]
    pub fn ram_synd_7_0(&self) -> RamSynd7_0R {
        RamSynd7_0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - 6-bit syndrome value for bits 31-16"]
    #[inline(always)]
    pub fn ram_synd_31_16(&self) -> RamSynd31_16R {
        RamSynd31_16R::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 6-bit syndrome value for bits 15-0"]
    #[inline(always)]
    pub fn ram_synd_7_0(&mut self) -> RamSynd7_0W<'_, RamTrapSynd0Spec> {
        RamSynd7_0W::new(self, 0)
    }
    #[doc = "Bits 6:11 - 6-bit syndrome value for bits 31-16"]
    #[inline(always)]
    pub fn ram_synd_31_16(&mut self) -> RamSynd31_16W<'_, RamTrapSynd0Spec> {
        RamSynd31_16W::new(self, 6)
    }
}
#[doc = "RAM0 EDAC Trap Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_trap_synd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_trap_synd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamTrapSynd0Spec;
impl crate::RegisterSpec for RamTrapSynd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram_trap_synd0::R`](R) reader structure"]
impl crate::Readable for RamTrapSynd0Spec {}
#[doc = "`write(|w| ..)` method takes [`ram_trap_synd0::W`](W) writer structure"]
impl crate::Writable for RamTrapSynd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAM_TRAP_SYND0 to value 0"]
impl crate::Resettable for RamTrapSynd0Spec {}
