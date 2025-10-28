#[doc = "Register `CNSTAT_HCMB` reader"]
pub type R = crate::R<CnstatHcmbSpec>;
#[doc = "Register `CNSTAT_HCMB` writer"]
pub type W = crate::W<CnstatHcmbSpec>;
#[doc = "Field `ST` reader - Buffer Status"]
pub type StR = crate::FieldReader;
#[doc = "Field `ST` writer - Buffer Status"]
pub type StW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRI` reader - Transmit Priority Code"]
pub type PriR = crate::FieldReader;
#[doc = "Field `PRI` writer - Transmit Priority Code"]
pub type PriW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DLC` reader - Data Length Code"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - Data Length Code"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Buffer Status"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Transmit Priority Code"]
    #[inline(always)]
    pub fn pri(&self) -> PriR {
        PriR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Buffer Status"]
    #[inline(always)]
    pub fn st(&mut self) -> StW<'_, CnstatHcmbSpec> {
        StW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Transmit Priority Code"]
    #[inline(always)]
    pub fn pri(&mut self) -> PriW<'_, CnstatHcmbSpec> {
        PriW::new(self, 4)
    }
    #[doc = "Bits 12:15 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DlcW<'_, CnstatHcmbSpec> {
        DlcW::new(self, 12)
    }
}
#[doc = "Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_hcmb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_hcmb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CnstatHcmbSpec;
impl crate::RegisterSpec for CnstatHcmbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnstat_hcmb::R`](R) reader structure"]
impl crate::Readable for CnstatHcmbSpec {}
#[doc = "`write(|w| ..)` method takes [`cnstat_hcmb::W`](W) writer structure"]
impl crate::Writable for CnstatHcmbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNSTAT_HCMB to value 0"]
impl crate::Resettable for CnstatHcmbSpec {}
