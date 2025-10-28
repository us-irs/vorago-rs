#[doc = "Register `ADDR9` reader"]
pub type R = crate::R<Addr9Spec>;
#[doc = "Register `ADDR9` writer"]
pub type W = crate::W<Addr9Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Address9 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr9Spec;
impl crate::RegisterSpec for Addr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr9::R`](R) reader structure"]
impl crate::Readable for Addr9Spec {}
#[doc = "`write(|w| ..)` method takes [`addr9::W`](W) writer structure"]
impl crate::Writable for Addr9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR9 to value 0"]
impl crate::Resettable for Addr9Spec {}
