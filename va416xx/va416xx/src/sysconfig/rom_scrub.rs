#[doc = "Register `ROM_SCRUB` reader"]
pub type R = crate::R<RomScrubSpec>;
#[doc = "Register `ROM_SCRUB` writer"]
pub type W = crate::W<RomScrubSpec>;
#[doc = "Field `VALUE` reader - Counter divide value"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Counter divide value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESET` writer - Reset Counter"]
pub type ResetW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Counter divide value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Counter divide value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, RomScrubSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bit 31 - Reset Counter"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, RomScrubSpec> {
        ResetW::new(self, 31)
    }
}
#[doc = "ROM Scrub Period Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_scrub::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_scrub::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomScrubSpec;
impl crate::RegisterSpec for RomScrubSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_scrub::R`](R) reader structure"]
impl crate::Readable for RomScrubSpec {}
#[doc = "`write(|w| ..)` method takes [`rom_scrub::W`](W) writer structure"]
impl crate::Writable for RomScrubSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8000_0000;
}
#[doc = "`reset()` method sets ROM_SCRUB to value 0"]
impl crate::Resettable for RomScrubSpec {}
