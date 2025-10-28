#[doc = "Register `MAC_VLAN_TAG` reader"]
pub type R = crate::R<MacVlanTagSpec>;
#[doc = "Register `MAC_VLAN_TAG` writer"]
pub type W = crate::W<MacVlanTagSpec>;
#[doc = "Field `VL` reader - VLAN Tag identifier for Receive Frames"]
pub type VlR = crate::FieldReader<u16>;
#[doc = "Field `VL` writer - VLAN Tag identifier for Receive Frames"]
pub type VlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison"]
pub type EtvR = crate::BitReader;
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison"]
pub type EtvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTIM` reader - VLAN Tag Inverse Match Enable"]
pub type VtimR = crate::BitReader;
#[doc = "Field `VTIM` writer - VLAN Tag Inverse Match Enable"]
pub type VtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESVL` reader - Enable S-VLAN"]
pub type EsvlR = crate::BitReader;
#[doc = "Field `ESVL` writer - Enable S-VLAN"]
pub type EsvlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag identifier for Receive Frames"]
    #[inline(always)]
    pub fn vl(&self) -> VlR {
        VlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&self) -> EtvR {
        EtvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn vtim(&self) -> VtimR {
        VtimR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn esvl(&self) -> EsvlR {
        EsvlR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag identifier for Receive Frames"]
    #[inline(always)]
    pub fn vl(&mut self) -> VlW<'_, MacVlanTagSpec> {
        VlW::new(self, 0)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&mut self) -> EtvW<'_, MacVlanTagSpec> {
        EtvW::new(self, 16)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn vtim(&mut self) -> VtimW<'_, MacVlanTagSpec> {
        VtimW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn esvl(&mut self) -> EsvlW<'_, MacVlanTagSpec> {
        EsvlW::new(self, 18)
    }
}
#[doc = "Identifies IEEE 802.1Q VLAN type frames\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_vlan_tag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_vlan_tag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacVlanTagSpec;
impl crate::RegisterSpec for MacVlanTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_vlan_tag::R`](R) reader structure"]
impl crate::Readable for MacVlanTagSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_vlan_tag::W`](W) writer structure"]
impl crate::Writable for MacVlanTagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_VLAN_TAG to value 0"]
impl crate::Resettable for MacVlanTagSpec {}
