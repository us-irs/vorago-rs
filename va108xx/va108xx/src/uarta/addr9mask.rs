#[doc = "Register `ADDR9MASK` reader"]
pub type R = crate::R<Addr9maskSpec>;
#[doc = "Register `ADDR9MASK` writer"]
pub type W = crate::W<Addr9maskSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Address9 Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr9mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr9mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr9maskSpec;
impl crate::RegisterSpec for Addr9maskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr9mask::R`](R) reader structure"]
impl crate::Readable for Addr9maskSpec {}
#[doc = "`write(|w| ..)` method takes [`addr9mask::W`](W) writer structure"]
impl crate::Writable for Addr9maskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR9MASK to value 0"]
impl crate::Resettable for Addr9maskSpec {}
