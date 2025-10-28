#[doc = "Register `TXFIFOIRQTRG` reader"]
pub type R = crate::R<TxfifoirqtrgSpec>;
#[doc = "Register `TXFIFOIRQTRG` writer"]
pub type W = crate::W<TxfifoirqtrgSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Tx FIFO IRQ Trigger Level\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifoirqtrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifoirqtrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfifoirqtrgSpec;
impl crate::RegisterSpec for TxfifoirqtrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfifoirqtrg::R`](R) reader structure"]
impl crate::Readable for TxfifoirqtrgSpec {}
#[doc = "`write(|w| ..)` method takes [`txfifoirqtrg::W`](W) writer structure"]
impl crate::Writable for TxfifoirqtrgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXFIFOIRQTRG to value 0"]
impl crate::Resettable for TxfifoirqtrgSpec {}
