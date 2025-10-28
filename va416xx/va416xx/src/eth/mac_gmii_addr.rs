#[doc = "Register `MAC_GMII_ADDR` reader"]
pub type R = crate::R<MacGmiiAddrSpec>;
#[doc = "Register `MAC_GMII_ADDR` writer"]
pub type W = crate::W<MacGmiiAddrSpec>;
#[doc = "Field `GB` reader - GMII Busy"]
pub type GbR = crate::BitReader;
#[doc = "Field `GB` writer - GMII Busy"]
pub type GbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GW` reader - GMII Write/Read"]
pub type GwR = crate::BitReader;
#[doc = "Field `GW` writer - GMII Write/Read"]
pub type GwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - CSR Clock Range"]
pub type CrR = crate::FieldReader;
#[doc = "Field `CR` writer - CSR Clock Range"]
pub type CrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GR` reader - GMII Register"]
pub type GrR = crate::FieldReader;
#[doc = "Field `GR` writer - GMII Register"]
pub type GrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - Physical Layer Address"]
pub type PaR = crate::FieldReader;
#[doc = "Field `PA` writer - Physical Layer Address"]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - GMII Busy"]
    #[inline(always)]
    pub fn gb(&self) -> GbR {
        GbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GMII Write/Read"]
    #[inline(always)]
    pub fn gw(&self) -> GwR {
        GwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - CSR Clock Range"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - GMII Register"]
    #[inline(always)]
    pub fn gr(&self) -> GrR {
        GrR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - GMII Busy"]
    #[inline(always)]
    pub fn gb(&mut self) -> GbW<'_, MacGmiiAddrSpec> {
        GbW::new(self, 0)
    }
    #[doc = "Bit 1 - GMII Write/Read"]
    #[inline(always)]
    pub fn gw(&mut self) -> GwW<'_, MacGmiiAddrSpec> {
        GwW::new(self, 1)
    }
    #[doc = "Bits 2:5 - CSR Clock Range"]
    #[inline(always)]
    pub fn cr(&mut self) -> CrW<'_, MacGmiiAddrSpec> {
        CrW::new(self, 2)
    }
    #[doc = "Bits 6:10 - GMII Register"]
    #[inline(always)]
    pub fn gr(&mut self) -> GrW<'_, MacGmiiAddrSpec> {
        GrW::new(self, 6)
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PaW<'_, MacGmiiAddrSpec> {
        PaW::new(self, 11)
    }
}
#[doc = "Controls the management cycles to an external PHY\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_gmii_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_gmii_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacGmiiAddrSpec;
impl crate::RegisterSpec for MacGmiiAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_gmii_addr::R`](R) reader structure"]
impl crate::Readable for MacGmiiAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_gmii_addr::W`](W) writer structure"]
impl crate::Writable for MacGmiiAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_GMII_ADDR to value 0"]
impl crate::Resettable for MacGmiiAddrSpec {}
