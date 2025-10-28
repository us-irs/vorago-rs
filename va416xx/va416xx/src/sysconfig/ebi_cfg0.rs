#[doc = "Register `EBI_CFG0` reader"]
pub type R = crate::R<EbiCfg0Spec>;
#[doc = "Register `EBI_CFG0` writer"]
pub type W = crate::W<EbiCfg0Spec>;
#[doc = "Field `ADDRLOW0` reader - Lower bound address for CEN0"]
pub type Addrlow0R = crate::FieldReader;
#[doc = "Field `ADDRLOW0` writer - Lower bound address for CEN0"]
pub type Addrlow0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDRHIGH0` reader - Upper bound address for CEN0"]
pub type Addrhigh0R = crate::FieldReader;
#[doc = "Field `ADDRHIGH0` writer - Upper bound address for CEN0"]
pub type Addrhigh0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CFGREADCYCLE` reader - Number of cycles for a read - N plus 1"]
pub type CfgreadcycleR = crate::FieldReader;
#[doc = "Field `CFGREADCYCLE` writer - Number of cycles for a read - N plus 1"]
pub type CfgreadcycleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CFGWRITECYCLE` reader - Number of cycles for a write - N plus 1"]
pub type CfgwritecycleR = crate::FieldReader;
#[doc = "Field `CFGWRITECYCLE` writer - Number of cycles for a write - N plus 1"]
pub type CfgwritecycleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CFGTURNAROUNDCYCLE` reader - Number of cycles for turnaround - N plus 1"]
pub type CfgturnaroundcycleR = crate::FieldReader;
#[doc = "Field `CFGTURNAROUNDCYCLE` writer - Number of cycles for turnaround - N plus 1"]
pub type CfgturnaroundcycleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CFGSIZE` reader - 8 bit (0) or 16 bit (1) port size"]
pub type CfgsizeR = crate::BitReader;
#[doc = "Field `CFGSIZE` writer - 8 bit (0) or 16 bit (1) port size"]
pub type CfgsizeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Lower bound address for CEN0"]
    #[inline(always)]
    pub fn addrlow0(&self) -> Addrlow0R {
        Addrlow0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Upper bound address for CEN0"]
    #[inline(always)]
    pub fn addrhigh0(&self) -> Addrhigh0R {
        Addrhigh0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Number of cycles for a read - N plus 1"]
    #[inline(always)]
    pub fn cfgreadcycle(&self) -> CfgreadcycleR {
        CfgreadcycleR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Number of cycles for a write - N plus 1"]
    #[inline(always)]
    pub fn cfgwritecycle(&self) -> CfgwritecycleR {
        CfgwritecycleR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Number of cycles for turnaround - N plus 1"]
    #[inline(always)]
    pub fn cfgturnaroundcycle(&self) -> CfgturnaroundcycleR {
        CfgturnaroundcycleR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - 8 bit (0) or 16 bit (1) port size"]
    #[inline(always)]
    pub fn cfgsize(&self) -> CfgsizeR {
        CfgsizeR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lower bound address for CEN0"]
    #[inline(always)]
    pub fn addrlow0(&mut self) -> Addrlow0W<'_, EbiCfg0Spec> {
        Addrlow0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Upper bound address for CEN0"]
    #[inline(always)]
    pub fn addrhigh0(&mut self) -> Addrhigh0W<'_, EbiCfg0Spec> {
        Addrhigh0W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Number of cycles for a read - N plus 1"]
    #[inline(always)]
    pub fn cfgreadcycle(&mut self) -> CfgreadcycleW<'_, EbiCfg0Spec> {
        CfgreadcycleW::new(self, 16)
    }
    #[doc = "Bits 19:21 - Number of cycles for a write - N plus 1"]
    #[inline(always)]
    pub fn cfgwritecycle(&mut self) -> CfgwritecycleW<'_, EbiCfg0Spec> {
        CfgwritecycleW::new(self, 19)
    }
    #[doc = "Bits 22:24 - Number of cycles for turnaround - N plus 1"]
    #[inline(always)]
    pub fn cfgturnaroundcycle(&mut self) -> CfgturnaroundcycleW<'_, EbiCfg0Spec> {
        CfgturnaroundcycleW::new(self, 22)
    }
    #[doc = "Bit 25 - 8 bit (0) or 16 bit (1) port size"]
    #[inline(always)]
    pub fn cfgsize(&mut self) -> CfgsizeW<'_, EbiCfg0Spec> {
        CfgsizeW::new(self, 25)
    }
}
#[doc = "EBI Config Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ebi_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ebi_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EbiCfg0Spec;
impl crate::RegisterSpec for EbiCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ebi_cfg0::R`](R) reader structure"]
impl crate::Readable for EbiCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ebi_cfg0::W`](W) writer structure"]
impl crate::Writable for EbiCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EBI_CFG0 to value 0"]
impl crate::Resettable for EbiCfg0Spec {}
