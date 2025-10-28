#[doc = "Register `VLAN_INCREPLACE` reader"]
pub type R = crate::R<VlanIncreplaceSpec>;
#[doc = "Register `VLAN_INCREPLACE` writer"]
pub type W = crate::W<VlanIncreplaceSpec>;
#[doc = "Field `VLT` reader - VLAN Tag for Transmit Frames"]
pub type VltR = crate::FieldReader<u16>;
#[doc = "Field `VLT` writer - VLAN Tag for Transmit Frames"]
pub type VltW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VLC` reader - VLAN Tag Control in Transmit Frames"]
pub type VlcR = crate::FieldReader;
#[doc = "Field `VLC` writer - VLAN Tag Control in Transmit Frames"]
pub type VlcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VLP` reader - VLAN Priority Control"]
pub type VlpR = crate::BitReader;
#[doc = "Field `VLP` writer - VLAN Priority Control"]
pub type VlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSVL` reader - C-VLAN or S-VLAN"]
pub type CsvlR = crate::BitReader;
#[doc = "Field `CSVL` writer - C-VLAN or S-VLAN"]
pub type CsvlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames"]
    #[inline(always)]
    pub fn vlt(&self) -> VltR {
        VltR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames"]
    #[inline(always)]
    pub fn vlc(&self) -> VlcR {
        VlcR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    pub fn vlp(&self) -> VlpR {
        VlpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    pub fn csvl(&self) -> CsvlR {
        CsvlR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames"]
    #[inline(always)]
    pub fn vlt(&mut self) -> VltW<'_, VlanIncreplaceSpec> {
        VltW::new(self, 0)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames"]
    #[inline(always)]
    pub fn vlc(&mut self) -> VlcW<'_, VlanIncreplaceSpec> {
        VlcW::new(self, 16)
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    pub fn vlp(&mut self) -> VlpW<'_, VlanIncreplaceSpec> {
        VlpW::new(self, 18)
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    pub fn csvl(&mut self) -> CsvlW<'_, VlanIncreplaceSpec> {
        CsvlW::new(self, 19)
    }
}
#[doc = "Holds the VLAN Tag for insertion into or replacement in the transmit frames\n\nYou can [`read`](crate::Reg::read) this register and get [`vlan_increplace::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vlan_increplace::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VlanIncreplaceSpec;
impl crate::RegisterSpec for VlanIncreplaceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vlan_increplace::R`](R) reader structure"]
impl crate::Readable for VlanIncreplaceSpec {}
#[doc = "`write(|w| ..)` method takes [`vlan_increplace::W`](W) writer structure"]
impl crate::Writable for VlanIncreplaceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VLAN_INCREPLACE to value 0"]
impl crate::Resettable for VlanIncreplaceSpec {}
