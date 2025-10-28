#[doc = "Register `ADDRESS` reader"]
pub type R = crate::R<AddressSpec>;
#[doc = "Register `ADDRESS` writer"]
pub type W = crate::W<AddressSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "I2C Address value\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddressSpec;
impl crate::RegisterSpec for AddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`address::R`](R) reader structure"]
impl crate::Readable for AddressSpec {}
#[doc = "`write(|w| ..)` method takes [`address::W`](W) writer structure"]
impl crate::Writable for AddressSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDRESS to value 0"]
impl crate::Resettable for AddressSpec {}
