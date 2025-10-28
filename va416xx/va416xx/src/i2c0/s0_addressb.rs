#[doc = "Register `S0_ADDRESSB` reader"]
pub type R = crate::R<S0AddressbSpec>;
#[doc = "Register `S0_ADDRESSB` writer"]
pub type W = crate::W<S0AddressbSpec>;
#[doc = "Field `RW` reader - Read write value"]
pub type RwR = crate::BitReader;
#[doc = "Field `RW` writer - Read write value"]
pub type RwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESS` reader - Address value"]
pub type AddressR = crate::FieldReader<u16>;
#[doc = "Field `ADDRESS` writer - Address value"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ADDRESSBEN` reader - Enable Address B"]
pub type AddressbenR = crate::BitReader;
#[doc = "Field `ADDRESSBEN` writer - Enable Address B"]
pub type AddressbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read write value"]
    #[inline(always)]
    pub fn rw(&self) -> RwR {
        RwR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Address value"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Enable Address B"]
    #[inline(always)]
    pub fn addressben(&self) -> AddressbenR {
        AddressbenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read write value"]
    #[inline(always)]
    pub fn rw(&mut self) -> RwW<'_, S0AddressbSpec> {
        RwW::new(self, 0)
    }
    #[doc = "Bits 1:10 - Address value"]
    #[inline(always)]
    pub fn address(&mut self) -> AddressW<'_, S0AddressbSpec> {
        AddressW::new(self, 1)
    }
    #[doc = "Bit 15 - Enable Address B"]
    #[inline(always)]
    pub fn addressben(&mut self) -> AddressbenW<'_, S0AddressbSpec> {
        AddressbenW::new(self, 15)
    }
}
#[doc = "Slave I2C Address B Value\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_addressb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_addressb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0AddressbSpec;
impl crate::RegisterSpec for S0AddressbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_addressb::R`](R) reader structure"]
impl crate::Readable for S0AddressbSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_addressb::W`](W) writer structure"]
impl crate::Writable for S0AddressbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S0_ADDRESSB to value 0"]
impl crate::Resettable for S0AddressbSpec {}
