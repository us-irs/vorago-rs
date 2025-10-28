#[doc = "Register `RAM_TRAP_ADDR1` reader"]
pub type R = crate::R<RamTrapAddr1Spec>;
#[doc = "Register `RAM_TRAP_ADDR1` writer"]
pub type W = crate::W<RamTrapAddr1Spec>;
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
    pub fn addr(&mut self) -> AddrW<'_, RamTrapAddr1Spec> {
        AddrW::new(self, 2)
    }
    #[doc = "Bit 31 - Enable Trap mode"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, RamTrapAddr1Spec> {
        EnableW::new(self, 31)
    }
}
#[doc = "RAM1 EDAC Trap Address\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_trap_addr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_trap_addr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamTrapAddr1Spec;
impl crate::RegisterSpec for RamTrapAddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram_trap_addr1::R`](R) reader structure"]
impl crate::Readable for RamTrapAddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ram_trap_addr1::W`](W) writer structure"]
impl crate::Writable for RamTrapAddr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAM_TRAP_ADDR1 to value 0"]
impl crate::Resettable for RamTrapAddr1Spec {}
