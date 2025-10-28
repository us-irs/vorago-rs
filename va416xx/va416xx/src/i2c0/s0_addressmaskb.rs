#[doc = "Register `S0_ADDRESSMASKB` reader"]
pub type R = crate::R<S0AddressmaskbSpec>;
#[doc = "Register `S0_ADDRESSMASKB` writer"]
pub type W = crate::W<S0AddressmaskbSpec>;
#[doc = "Field `RWMASK` reader - Read write mask"]
pub type RwmaskR = crate::BitReader;
#[doc = "Field `RWMASK` writer - Read write mask"]
pub type RwmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK` reader - Address mask value"]
pub type MaskR = crate::FieldReader<u16>;
#[doc = "Field `MASK` writer - Address mask value"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - Read write mask"]
    #[inline(always)]
    pub fn rwmask(&self) -> RwmaskR {
        RwmaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Address mask value"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Read write mask"]
    #[inline(always)]
    pub fn rwmask(&mut self) -> RwmaskW<'_, S0AddressmaskbSpec> {
        RwmaskW::new(self, 0)
    }
    #[doc = "Bits 1:10 - Address mask value"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<'_, S0AddressmaskbSpec> {
        MaskW::new(self, 1)
    }
}
#[doc = "Slave I2C Address B Mask value\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_addressmaskb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_addressmaskb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0AddressmaskbSpec;
impl crate::RegisterSpec for S0AddressmaskbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_addressmaskb::R`](R) reader structure"]
impl crate::Readable for S0AddressmaskbSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_addressmaskb::W`](W) writer structure"]
impl crate::Writable for S0AddressmaskbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S0_ADDRESSMASKB to value 0x07fe"]
impl crate::Resettable for S0AddressmaskbSpec {
    const RESET_VALUE: u32 = 0x07fe;
}
