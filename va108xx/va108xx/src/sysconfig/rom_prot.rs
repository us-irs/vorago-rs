#[doc = "Register `ROM_PROT` reader"]
pub type R = crate::R<RomProtSpec>;
#[doc = "Register `ROM_PROT` writer"]
pub type W = crate::W<RomProtSpec>;
#[doc = "Field `WREN` reader - ROM Write Enable Bit"]
pub type WrenR = crate::BitReader;
#[doc = "Field `WREN` writer - ROM Write Enable Bit"]
pub type WrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ROM Write Enable Bit"]
    #[inline(always)]
    pub fn wren(&self) -> WrenR {
        WrenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ROM Write Enable Bit"]
    #[inline(always)]
    pub fn wren(&mut self) -> WrenW<'_, RomProtSpec> {
        WrenW::new(self, 0)
    }
}
#[doc = "ROM Protection Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_prot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_prot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomProtSpec;
impl crate::RegisterSpec for RomProtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_prot::R`](R) reader structure"]
impl crate::Readable for RomProtSpec {}
#[doc = "`write(|w| ..)` method takes [`rom_prot::W`](W) writer structure"]
impl crate::Writable for RomProtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROM_PROT to value 0x01"]
impl crate::Resettable for RomProtSpec {
    const RESET_VALUE: u32 = 0x01;
}
