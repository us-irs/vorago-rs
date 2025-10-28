#[doc = "Register `RST_STAT` reader"]
pub type R = crate::R<RstStatSpec>;
#[doc = "Register `RST_STAT` writer"]
pub type W = crate::W<RstStatSpec>;
#[doc = "Field `POR` reader - Power On Reset Status"]
pub type PorR = crate::BitReader;
#[doc = "Field `POR` writer - Power On Reset Status"]
pub type PorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRST` reader - External Reset Status"]
pub type ExtrstR = crate::BitReader;
#[doc = "Field `EXTRST` writer - External Reset Status"]
pub type ExtrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRSTREQ` reader - SYSRESETREQ Reset Status"]
pub type SysrstreqR = crate::BitReader;
#[doc = "Field `SYSRSTREQ` writer - SYSRESETREQ Reset Status"]
pub type SysrstreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOKUP` reader - LOOKUP Reset Status"]
pub type LookupR = crate::BitReader;
#[doc = "Field `LOOKUP` writer - LOOKUP Reset Status"]
pub type LookupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WATCHDOG` reader - WATCHDOG Reset Status"]
pub type WatchdogR = crate::BitReader;
#[doc = "Field `WATCHDOG` writer - WATCHDOG Reset Status"]
pub type WatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMERR` reader - Memory Error Reset Status"]
pub type MemerrR = crate::BitReader;
#[doc = "Field `MEMERR` writer - Memory Error Reset Status"]
pub type MemerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power On Reset Status"]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Reset Status"]
    #[inline(always)]
    pub fn extrst(&self) -> ExtrstR {
        ExtrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYSRESETREQ Reset Status"]
    #[inline(always)]
    pub fn sysrstreq(&self) -> SysrstreqR {
        SysrstreqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LOOKUP Reset Status"]
    #[inline(always)]
    pub fn lookup(&self) -> LookupR {
        LookupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WATCHDOG Reset Status"]
    #[inline(always)]
    pub fn watchdog(&self) -> WatchdogR {
        WatchdogR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Memory Error Reset Status"]
    #[inline(always)]
    pub fn memerr(&self) -> MemerrR {
        MemerrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power On Reset Status"]
    #[inline(always)]
    pub fn por(&mut self) -> PorW<'_, RstStatSpec> {
        PorW::new(self, 0)
    }
    #[doc = "Bit 1 - External Reset Status"]
    #[inline(always)]
    pub fn extrst(&mut self) -> ExtrstW<'_, RstStatSpec> {
        ExtrstW::new(self, 1)
    }
    #[doc = "Bit 2 - SYSRESETREQ Reset Status"]
    #[inline(always)]
    pub fn sysrstreq(&mut self) -> SysrstreqW<'_, RstStatSpec> {
        SysrstreqW::new(self, 2)
    }
    #[doc = "Bit 3 - LOOKUP Reset Status"]
    #[inline(always)]
    pub fn lookup(&mut self) -> LookupW<'_, RstStatSpec> {
        LookupW::new(self, 3)
    }
    #[doc = "Bit 4 - WATCHDOG Reset Status"]
    #[inline(always)]
    pub fn watchdog(&mut self) -> WatchdogW<'_, RstStatSpec> {
        WatchdogW::new(self, 4)
    }
    #[doc = "Bit 5 - Memory Error Reset Status"]
    #[inline(always)]
    pub fn memerr(&mut self) -> MemerrW<'_, RstStatSpec> {
        MemerrW::new(self, 5)
    }
}
#[doc = "System Reset Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstStatSpec;
impl crate::RegisterSpec for RstStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_stat::R`](R) reader structure"]
impl crate::Readable for RstStatSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_stat::W`](W) writer structure"]
impl crate::Writable for RstStatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RST_STAT to value 0x01"]
impl crate::Resettable for RstStatSpec {
    const RESET_VALUE: u32 = 0x01;
}
