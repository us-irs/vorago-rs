#[doc = "Register `ROM_TRAP_SYND` reader"]
pub type R = crate::R<RomTrapSyndSpec>;
#[doc = "Register `ROM_TRAP_SYND` writer"]
pub type W = crate::W<RomTrapSyndSpec>;
#[doc = "Field `SYND` reader - Trap Syndrom Bits"]
pub type SyndR = crate::FieldReader<u32>;
#[doc = "Field `SYND` writer - Trap Syndrom Bits"]
pub type SyndW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Trap Syndrom Bits"]
    #[inline(always)]
    pub fn synd(&self) -> SyndR {
        SyndR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Trap Syndrom Bits"]
    #[inline(always)]
    pub fn synd(&mut self) -> SyndW<'_, RomTrapSyndSpec> {
        SyndW::new(self, 0)
    }
}
#[doc = "ROM Trap Syndrome\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_trap_synd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_trap_synd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
