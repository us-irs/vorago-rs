#[doc = "Register `TXFIFOIRQTRG` reader"]
pub type R = crate::R<TxfifoirqtrgSpec>;
#[doc = "Register `TXFIFOIRQTRG` writer"]
pub type W = crate::W<TxfifoirqtrgSpec>;
#[doc = "Field `LEVEL` reader - Sets the FIFO_ENTRY_CNT value that asserts the FIFO_DEPTH_TRIG interrupt"]
pub type LevelR = crate::FieldReader;
#[doc = "Field `LEVEL` writer - Sets the FIFO_ENTRY_CNT value that asserts the FIFO_DEPTH_TRIG interrupt"]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Sets the FIFO_ENTRY_CNT value that asserts the FIFO_DEPTH_TRIG interrupt"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Sets the FIFO_ENTRY_CNT value that asserts the FIFO_DEPTH_TRIG interrupt"]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, TxfifoirqtrgSpec> {
        LevelW::new(self, 0)
    }
}
#[doc = "Receive FIFO Interrupt Trigger Value\n\nYou can [`read`](crate::Reg::read) this register and get [`txfifoirqtrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifoirqtrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets TXFIFOIRQTRG to value 0x10"]
impl crate::Resettable for TxfifoirqtrgSpec {
    const RESET_VALUE: u32 = 0x10;
}
