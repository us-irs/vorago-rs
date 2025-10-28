#[doc = "Register `ROM_TRAP_ADDR` reader"]
pub type R = crate::R<RomTrapAddrSpec>;
#[doc = "Register `ROM_TRAP_ADDR` writer"]
pub type W = crate::W<RomTrapAddrSpec>;
#[doc = "Field `ADDR` reader - Trap Address Match Bits"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Trap Address Match Bits"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `ENABLE` reader - Trap Enable Bit"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Trap Enable Bit"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - Trap Address Match Bits"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Trap Enable Bit"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Trap Address Match Bits"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, RomTrapAddrSpec> {
        AddrW::new(self, 2)
    }
    #[doc = "Bit 31 - Trap Enable Bit"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, RomTrapAddrSpec> {
        EnableW::new(self, 31)
    }
}
#[doc = "ROM Trap Address\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_trap_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_trap_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomTrapAddrSpec;
impl crate::RegisterSpec for RomTrapAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_trap_addr::R`](R) reader structure"]
impl crate::Readable for RomTrapAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`rom_trap_addr::W`](W) writer structure"]
impl crate::Writable for RomTrapAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROM_TRAP_ADDR to value 0"]
impl crate::Resettable for RomTrapAddrSpec {}
