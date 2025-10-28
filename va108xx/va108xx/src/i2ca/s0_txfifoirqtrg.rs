#[doc = "Register `S0_TXFIFOIRQTRG` reader"]
pub type R = crate::R<S0TxfifoirqtrgSpec>;
#[doc = "Register `S0_TXFIFOIRQTRG` writer"]
pub type W = crate::W<S0TxfifoirqtrgSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Slave Tx FIFO IRQ Trigger Level\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_txfifoirqtrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_txfifoirqtrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0TxfifoirqtrgSpec;
impl crate::RegisterSpec for S0TxfifoirqtrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_txfifoirqtrg::R`](R) reader structure"]
impl crate::Readable for S0TxfifoirqtrgSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_txfifoirqtrg::W`](W) writer structure"]
impl crate::Writable for S0TxfifoirqtrgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S0_TXFIFOIRQTRG to value 0"]
impl crate::Resettable for S0TxfifoirqtrgSpec {}
