#[doc = "Register `S0_RXFIFOIRQTRG` reader"]
pub type R = crate::R<S0RxfifoirqtrgSpec>;
#[doc = "Register `S0_RXFIFOIRQTRG` writer"]
pub type W = crate::W<S0RxfifoirqtrgSpec>;
#[doc = "Field `LEVEL` reader - Half full level for the Rx FIFO"]
pub type LevelR = crate::FieldReader;
#[doc = "Field `LEVEL` writer - Half full level for the Rx FIFO"]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Half full level for the Rx FIFO"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Half full level for the Rx FIFO"]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, S0RxfifoirqtrgSpec> {
        LevelW::new(self, 0)
    }
}
#[doc = "Slave Rx FIFO IRQ Trigger Level\n\nYou can [`read`](crate::Reg::read) this register and get [`s0_rxfifoirqtrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0_rxfifoirqtrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0RxfifoirqtrgSpec;
impl crate::RegisterSpec for S0RxfifoirqtrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_rxfifoirqtrg::R`](R) reader structure"]
impl crate::Readable for S0RxfifoirqtrgSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_rxfifoirqtrg::W`](W) writer structure"]
impl crate::Writable for S0RxfifoirqtrgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S0_RXFIFOIRQTRG to value 0"]
impl crate::Resettable for S0RxfifoirqtrgSpec {}
