#[doc = "Register `ROM_TRAP_ADDRESS` reader"]
pub type R = crate::R<RomTrapAddressSpec>;
#[doc = "Register `ROM_TRAP_ADDRESS` writer"]
pub type W = crate::W<RomTrapAddressSpec>;
#[doc = "Field `ADDR` reader - Address bits for trap match"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Address bits for trap match"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `ENABLE` reader - Enable Trap mode"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable Trap mode"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:30 - Address bits for trap match"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x1fff_ffff)
    }
    #[doc = "Bit 31 - Enable Trap mode"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:30 - Address bits for trap match"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, RomTrapAddressSpec> {
        AddrW::new(self, 2)
    }
    #[doc = "Bit 31 - Enable Trap mode"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, RomTrapAddressSpec> {
        EnableW::new(self, 31)
    }
}
#[doc = "ROM EDAC Trap Address\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_trap_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_trap_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomTrapAddressSpec;
impl crate::RegisterSpec for RomTrapAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_trap_address::R`](R) reader structure"]
impl crate::Readable for RomTrapAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`rom_trap_address::W`](W) writer structure"]
impl crate::Writable for RomTrapAddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROM_TRAP_ADDRESS to value 0"]
impl crate::Resettable for RomTrapAddressSpec {}
