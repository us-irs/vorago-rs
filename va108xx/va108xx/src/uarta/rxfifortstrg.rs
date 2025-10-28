#[doc = "Register `RXFIFORTSTRG` reader"]
pub type R = crate::R<RxfifortstrgSpec>;
#[doc = "Register `RXFIFORTSTRG` writer"]
pub type W = crate::W<RxfifortstrgSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Rx FIFO RTS Trigger Level\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifortstrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifortstrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfifortstrgSpec;
impl crate::RegisterSpec for RxfifortstrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifortstrg::R`](R) reader structure"]
impl crate::Readable for RxfifortstrgSpec {}
#[doc = "`write(|w| ..)` method takes [`rxfifortstrg::W`](W) writer structure"]
impl crate::Writable for RxfifortstrgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXFIFORTSTRG to value 0"]
impl crate::Resettable for RxfifortstrgSpec {}
