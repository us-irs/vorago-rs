#[doc = "Register `MMC_CNTRL` reader"]
pub type R = crate::R<MmcCntrlSpec>;
#[doc = "Register `MMC_CNTRL` writer"]
pub type W = crate::W<MmcCntrlSpec>;
#[doc = "Field `CNTRST` reader - Counters Reset"]
pub type CntrstR = crate::BitReader;
#[doc = "Field `CNTRST` writer - Counters Reset"]
pub type CntrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTSTOPRO` reader - Counter Stop Rollover"]
pub type CntstoproR = crate::BitReader;
#[doc = "Field `CNTSTOPRO` writer - Counter Stop Rollover"]
pub type CntstoproW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTONRD` reader - Reset on Read"]
pub type RstonrdR = crate::BitReader;
#[doc = "Field `RSTONRD` writer - Reset on Read"]
pub type RstonrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTFREEZ` reader - MMC Counter Freeze"]
pub type CntfreezR = crate::BitReader;
#[doc = "Field `CNTFREEZ` writer - MMC Counter Freeze"]
pub type CntfreezW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTPRST` reader - Counters Preset"]
pub type CntprstR = crate::BitReader;
#[doc = "Field `CNTPRST` writer - Counters Preset"]
pub type CntprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTPRSTLVL` reader - Full-Half Preset"]
pub type CntprstlvlR = crate::BitReader;
#[doc = "Field `CNTPRSTLVL` writer - Full-Half Preset"]
pub type CntprstlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCDBC` reader - Update MMC Counters for Dropped Broadcast Frames"]
pub type UcdbcR = crate::BitReader;
#[doc = "Field `UCDBC` writer - Update MMC Counters for Dropped Broadcast Frames"]
pub type UcdbcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    pub fn cntrst(&self) -> CntrstR {
        CntrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Stop Rollover"]
    #[inline(always)]
    pub fn cntstopro(&self) -> CntstoproR {
        CntstoproR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    pub fn rstonrd(&self) -> RstonrdR {
        RstonrdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    pub fn cntfreez(&self) -> CntfreezR {
        CntfreezR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    pub fn cntprst(&self) -> CntprstR {
        CntprstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full-Half Preset"]
    #[inline(always)]
    pub fn cntprstlvl(&self) -> CntprstlvlR {
        CntprstlvlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Frames"]
    #[inline(always)]
    pub fn ucdbc(&self) -> UcdbcR {
        UcdbcR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    pub fn cntrst(&mut self) -> CntrstW<'_, MmcCntrlSpec> {
        CntrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Counter Stop Rollover"]
    #[inline(always)]
    pub fn cntstopro(&mut self) -> CntstoproW<'_, MmcCntrlSpec> {
        CntstoproW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    pub fn rstonrd(&mut self) -> RstonrdW<'_, MmcCntrlSpec> {
        RstonrdW::new(self, 2)
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    pub fn cntfreez(&mut self) -> CntfreezW<'_, MmcCntrlSpec> {
        CntfreezW::new(self, 3)
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    pub fn cntprst(&mut self) -> CntprstW<'_, MmcCntrlSpec> {
        CntprstW::new(self, 4)
    }
    #[doc = "Bit 5 - Full-Half Preset"]
    #[inline(always)]
    pub fn cntprstlvl(&mut self) -> CntprstlvlW<'_, MmcCntrlSpec> {
        CntprstlvlW::new(self, 5)
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Frames"]
    #[inline(always)]
    pub fn ucdbc(&mut self) -> UcdbcW<'_, MmcCntrlSpec> {
        UcdbcW::new(self, 8)
    }
}
#[doc = "MMC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_cntrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_cntrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcCntrlSpec;
impl crate::RegisterSpec for MmcCntrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_cntrl::R`](R) reader structure"]
impl crate::Readable for MmcCntrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_cntrl::W`](W) writer structure"]
impl crate::Writable for MmcCntrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMC_CNTRL to value 0"]
impl crate::Resettable for MmcCntrlSpec {}
