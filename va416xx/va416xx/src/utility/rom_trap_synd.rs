#[doc = "Register `ROM_TRAP_SYND` reader"]
pub type R = crate::R<RomTrapSyndSpec>;
#[doc = "Register `ROM_TRAP_SYND` writer"]
pub type W = crate::W<RomTrapSyndSpec>;
#[doc = "Field `ROM_SYND_7_0` reader - 6-bit syndrome value for bits 15-0"]
pub type RomSynd7_0R = crate::FieldReader;
#[doc = "Field `ROM_SYND_7_0` writer - 6-bit syndrome value for bits 15-0"]
pub type RomSynd7_0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `R0M_SYND_31_16` reader - 6-bit syndrome value for bits 31-16"]
pub type R0mSynd31_16R = crate::FieldReader;
#[doc = "Field `R0M_SYND_31_16` writer - 6-bit syndrome value for bits 31-16"]
pub type R0mSynd31_16W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 6-bit syndrome value for bits 15-0"]
    #[inline(always)]
    pub fn rom_synd_7_0(&self) -> RomSynd7_0R {
        RomSynd7_0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - 6-bit syndrome value for bits 31-16"]
    #[inline(always)]
    pub fn r0m_synd_31_16(&self) -> R0mSynd31_16R {
        R0mSynd31_16R::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 6-bit syndrome value for bits 15-0"]
    #[inline(always)]
    pub fn rom_synd_7_0(&mut self) -> RomSynd7_0W<'_, RomTrapSyndSpec> {
        RomSynd7_0W::new(self, 0)
    }
    #[doc = "Bits 6:11 - 6-bit syndrome value for bits 31-16"]
    #[inline(always)]
    pub fn r0m_synd_31_16(&mut self) -> R0mSynd31_16W<'_, RomTrapSyndSpec> {
        R0mSynd31_16W::new(self, 6)
    }
}
#[doc = "ROM EDAC Trap Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_trap_synd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_trap_synd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomTrapSyndSpec;
impl crate::RegisterSpec for RomTrapSyndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_trap_synd::R`](R) reader structure"]
impl crate::Readable for RomTrapSyndSpec {}
#[doc = "`write(|w| ..)` method takes [`rom_trap_synd::W`](W) writer structure"]
impl crate::Writable for RomTrapSyndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROM_TRAP_SYND to value 0"]
impl crate::Resettable for RomTrapSyndSpec {}
