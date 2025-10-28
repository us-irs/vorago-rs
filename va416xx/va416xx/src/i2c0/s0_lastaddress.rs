#[doc = "Register `S0_LASTADDRESS` reader"]
pub type R = crate::R<S0LastaddressSpec>;
#[doc = "Field `DIRECTION` reader - Transaction direction 0=master send, 1=master receive"]
pub type DirectionR = crate::BitReader;
#[doc = "Field `ADDRESS` reader - Address value"]
pub type AddressR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Transaction direction 0=master send, 1=master receive"]
    #[inline(always)]
    pub fn direction(&self) -> DirectionR {
        DirectionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Address value"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
#[doc = "Slave I2C Last Address value\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_lastaddress::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0LastaddressSpec;
impl crate::RegisterSpec for S0LastaddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_lastaddress::R`](R) reader structure"]
impl crate::Readable for S0LastaddressSpec {}
#[doc = "`reset()` method sets S0_LASTADDRESS to value 0"]
impl crate::Resettable for S0LastaddressSpec {}
