#[doc = "Register `VLAN_HASHTABLE` reader"]
pub type R = crate::R<VlanHashtableSpec>;
#[doc = "Register `VLAN_HASHTABLE` writer"]
pub type W = crate::W<VlanHashtableSpec>;
#[doc = "Field `VLHT` reader - VLAN Hash Table"]
pub type VlhtR = crate::FieldReader<u16>;
#[doc = "Field `VLHT` writer - VLAN Hash Table"]
pub type VlhtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VLAN Hash Table"]
    #[inline(always)]
    pub fn vlht(&self) -> VlhtR {
        VlhtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Hash Table"]
    #[inline(always)]
    pub fn vlht(&mut self) -> VlhtW<'_, VlanHashtableSpec> {
        VlhtW::new(self, 0)
    }
}
#[doc = "Holds the VLAN Hash Table\n\nYou can [`read`](crate::Reg::read) this register and get [`vlan_hashtable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vlan_hashtable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VlanHashtableSpec;
impl crate::RegisterSpec for VlanHashtableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vlan_hashtable::R`](R) reader structure"]
impl crate::Readable for VlanHashtableSpec {}
#[doc = "`write(|w| ..)` method takes [`vlan_hashtable::W`](W) writer structure"]
impl crate::Writable for VlanHashtableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VLAN_HASHTABLE to value 0"]
impl crate::Resettable for VlanHashtableSpec {}
