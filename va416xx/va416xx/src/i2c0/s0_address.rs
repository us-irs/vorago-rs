#[doc = "Register `S0_ADDRESS` reader"]
pub type R = crate::R<S0AddressSpec>;
#[doc = "Register `S0_ADDRESS` writer"]
pub type W = crate::W<S0AddressSpec>;
#[doc = "Field `RW` reader - Read/Write value"]
pub type RwR = crate::BitReader;
#[doc = "Field `RW` writer - Read/Write value"]
pub type RwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRESS` reader - Address value"]
pub type AddressR = crate::FieldReader<u16>;
#[doc = "Field `ADDRESS` writer - Address value"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `A10MODE` reader - Enable 10b address mode"]
pub type A10modeR = crate::BitReader;
#[doc = "Field `A10MODE` writer - Enable 10b address mode"]
pub type A10modeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read/Write value"]
    #[inline(always)]
    pub fn rw(&self) -> RwR {
        RwR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Address value"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Enable 10b address mode"]
    #[inline(always)]
    pub fn a10mode(&self) -> A10modeR {
        A10modeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read/Write value"]
    #[inline(always)]
    pub fn rw(&mut self) -> RwW<'_, S0AddressSpec> {
        RwW::new(self, 0)
    }
    #[doc = "Bits 1:10 - Address value"]
    #[inline(always)]
    pub fn address(&mut self) -> AddressW<'_, S0AddressSpec> {
        AddressW::new(self, 1)
    }
    #[doc = "Bit 15 - Enable 10b address mode"]
    #[inline(always)]
    pub fn a10mode(&mut self) -> A10modeW<'_, S0AddressSpec> {
        A10modeW::new(self, 15)
    }
}
#[doc = "Slave I2C Address Value\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0AddressSpec;
impl crate::RegisterSpec for S0AddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_address::R`](R) reader structure"]
impl crate::Readable for S0AddressSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_address::W`](W) writer structure"]
impl crate::Writable for S0AddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S0_ADDRESS to value 0"]
impl crate::Resettable for S0AddressSpec {}
