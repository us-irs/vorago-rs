#[doc = "Register `RXFIFOIRQTRG` reader"]
pub type R = crate::R<RxfifoirqtrgSpec>;
#[doc = "Register `RXFIFOIRQTRG` writer"]
pub type W = crate::W<RxfifoirqtrgSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Rx FIFO IRQ Trigger Level\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifoirqtrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfifoirqtrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfifoirqtrgSpec;
impl crate::RegisterSpec for RxfifoirqtrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifoirqtrg::R`](R) reader structure"]
impl crate::Readable for RxfifoirqtrgSpec {}
#[doc = "`write(|w| ..)` method takes [`rxfifoirqtrg::W`](W) writer structure"]
impl crate::Writable for RxfifoirqtrgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXFIFOIRQTRG to value 0"]
impl crate::Resettable for RxfifoirqtrgSpec {}
