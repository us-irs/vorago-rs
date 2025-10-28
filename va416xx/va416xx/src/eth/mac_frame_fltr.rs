#[doc = "Register `MAC_FRAME_FLTR` reader"]
pub type R = crate::R<MacFrameFltrSpec>;
#[doc = "Register `MAC_FRAME_FLTR` writer"]
pub type W = crate::W<MacFrameFltrSpec>;
#[doc = "Field `PR` reader - Promiscuous Mode"]
pub type PrR = crate::BitReader;
#[doc = "Field `PR` writer - Promiscuous Mode"]
pub type PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUC` reader - Hash Unicast"]
pub type HucR = crate::BitReader;
#[doc = "Field `HUC` writer - Hash Unicast"]
pub type HucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMC` reader - Hash Multicast"]
pub type HmcR = crate::BitReader;
#[doc = "Field `HMC` writer - Hash Multicast"]
pub type HmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAIF` reader - DA Inverse Filtering"]
pub type DaifR = crate::BitReader;
#[doc = "Field `DAIF` writer - DA Inverse Filtering"]
pub type DaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM` reader - Pass All Multicast"]
pub type PmR = crate::BitReader;
#[doc = "Field `PM` writer - Pass All Multicast"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBF` reader - Disable Broadcast Frames"]
pub type DbfR = crate::BitReader;
#[doc = "Field `DBF` writer - Disable Broadcast Frames"]
pub type DbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCF` reader - Pass Control Frames"]
pub type PcfR = crate::FieldReader;
#[doc = "Field `PCF` writer - Pass Control Frames"]
pub type PcfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAIF` reader - SA Inverse Filtering"]
pub type SaifR = crate::BitReader;
#[doc = "Field `SAIF` writer - SA Inverse Filtering"]
pub type SaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF` reader - Source Address Filter Enable"]
pub type SafR = crate::BitReader;
#[doc = "Field `SAF` writer - Source Address Filter Enable"]
pub type SafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDF` reader - Hash or Perfect Filter"]
pub type HdfR = crate::BitReader;
#[doc = "Field `HDF` writer - Hash or Perfect Filter"]
pub type HdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VFTE` reader - VLAN Tag Filter Enable"]
pub type VfteR = crate::BitReader;
#[doc = "Field `VFTE` writer - VLAN Tag Filter Enable"]
pub type VfteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNTU` reader - Drop non TCP/UDP over IP Frames"]
pub type DntuR = crate::BitReader;
#[doc = "Field `DNTU` writer - Drop non TCP/UDP over IP Frames"]
pub type DntuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RA` reader - Receive All"]
pub type RaR = crate::BitReader;
#[doc = "Field `RA` writer - Receive All"]
pub type RaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn huc(&self) -> HucR {
        HucR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn hmc(&self) -> HmcR {
        HmcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DA Inverse Filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DaifR {
        DaifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames"]
    #[inline(always)]
    pub fn dbf(&self) -> DbfR {
        DbfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PcfR {
        PcfR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SA Inverse Filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SaifR {
        SaifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    pub fn saf(&self) -> SafR {
        SafR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn hdf(&self) -> HdfR {
        HdfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    pub fn vfte(&self) -> VfteR {
        VfteR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - Drop non TCP/UDP over IP Frames"]
    #[inline(always)]
    pub fn dntu(&self) -> DntuR {
        DntuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn pr(&mut self) -> PrW<'_, MacFrameFltrSpec> {
        PrW::new(self, 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn huc(&mut self) -> HucW<'_, MacFrameFltrSpec> {
        HucW::new(self, 1)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn hmc(&mut self) -> HmcW<'_, MacFrameFltrSpec> {
        HmcW::new(self, 2)
    }
    #[doc = "Bit 3 - DA Inverse Filtering"]
    #[inline(always)]
    pub fn daif(&mut self) -> DaifW<'_, MacFrameFltrSpec> {
        DaifW::new(self, 3)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<'_, MacFrameFltrSpec> {
        PmW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames"]
    #[inline(always)]
    pub fn dbf(&mut self) -> DbfW<'_, MacFrameFltrSpec> {
        DbfW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    pub fn pcf(&mut self) -> PcfW<'_, MacFrameFltrSpec> {
        PcfW::new(self, 6)
    }
    #[doc = "Bit 8 - SA Inverse Filtering"]
    #[inline(always)]
    pub fn saif(&mut self) -> SaifW<'_, MacFrameFltrSpec> {
        SaifW::new(self, 8)
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    pub fn saf(&mut self) -> SafW<'_, MacFrameFltrSpec> {
        SafW::new(self, 9)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn hdf(&mut self) -> HdfW<'_, MacFrameFltrSpec> {
        HdfW::new(self, 10)
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    pub fn vfte(&mut self) -> VfteW<'_, MacFrameFltrSpec> {
        VfteW::new(self, 16)
    }
    #[doc = "Bit 21 - Drop non TCP/UDP over IP Frames"]
    #[inline(always)]
    pub fn dntu(&mut self) -> DntuW<'_, MacFrameFltrSpec> {
        DntuW::new(self, 21)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn ra(&mut self) -> RaW<'_, MacFrameFltrSpec> {
        RaW::new(self, 31)
    }
}
#[doc = "Contains the frame filtering controls\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_frame_fltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_frame_fltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacFrameFltrSpec;
impl crate::RegisterSpec for MacFrameFltrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_frame_fltr::R`](R) reader structure"]
impl crate::Readable for MacFrameFltrSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_frame_fltr::W`](W) writer structure"]
impl crate::Writable for MacFrameFltrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_FRAME_FLTR to value 0"]
impl crate::Resettable for MacFrameFltrSpec {}
