#[doc = "Register `IRQ_ENB` reader"]
pub type R = crate::R<IrqEnbSpec>;
#[doc = "Register `IRQ_ENB` writer"]
pub type W = crate::W<IrqEnbSpec>;
#[doc = "Field `FIFO_EMPTY` reader - Enables the interrupt for FIFO empty"]
pub type FifoEmptyR = crate::BitReader;
#[doc = "Field `FIFO_EMPTY` writer - Enables the interrupt for FIFO empty"]
pub type FifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_FULL` reader - Enables the interrupt for FIFO full"]
pub type FifoFullR = crate::BitReader;
#[doc = "Field `FIFO_FULL` writer - Enables the interrupt for FIFO full"]
pub type FifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_OFLOW` reader - Enables the interrupt for a FIFO overflow"]
pub type FifoOflowR = crate::BitReader;
#[doc = "Field `FIFO_OFLOW` writer - Enables the interrupt for a FIFO overflow"]
pub type FifoOflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_UFLOW` reader - Enables the interrupt for a FIFO underflow"]
pub type FifoUflowR = crate::BitReader;
#[doc = "Field `FIFO_UFLOW` writer - Enables the interrupt for a FIFO underflow"]
pub type FifoUflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_DONE` reader - Enables the interrupt for a DAC data acquisition completion"]
pub type DacDoneR = crate::BitReader;
#[doc = "Field `DAC_DONE` writer - Enables the interrupt for a DAC data acquisition completion"]
pub type DacDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIG_ERROR` reader - Enables the interrupt for a trigger error"]
pub type TrigErrorR = crate::BitReader;
#[doc = "Field `TRIG_ERROR` writer - Enables the interrupt for a trigger error"]
pub type TrigErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_DEPTH_TRIG` reader - Enables the interrupt for the FIFO entry count is less than or equal to the trigger level"]
pub type FifoDepthTrigR = crate::BitReader;
#[doc = "Field `FIFO_DEPTH_TRIG` writer - Enables the interrupt for the FIFO entry count is less than or equal to the trigger level"]
pub type FifoDepthTrigW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the interrupt for FIFO empty"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FifoEmptyR {
        FifoEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the interrupt for FIFO full"]
    #[inline(always)]
    pub fn fifo_full(&self) -> FifoFullR {
        FifoFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the interrupt for a FIFO overflow"]
    #[inline(always)]
    pub fn fifo_oflow(&self) -> FifoOflowR {
        FifoOflowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the interrupt for a FIFO underflow"]
    #[inline(always)]
    pub fn fifo_uflow(&self) -> FifoUflowR {
        FifoUflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables the interrupt for a DAC data acquisition completion"]
    #[inline(always)]
    pub fn dac_done(&self) -> DacDoneR {
        DacDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the interrupt for a trigger error"]
    #[inline(always)]
    pub fn trig_error(&self) -> TrigErrorR {
        TrigErrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables the interrupt for the FIFO entry count is less than or equal to the trigger level"]
    #[inline(always)]
    pub fn fifo_depth_trig(&self) -> FifoDepthTrigR {
        FifoDepthTrigR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the interrupt for FIFO empty"]
    #[inline(always)]
    pub fn fifo_empty(&mut self) -> FifoEmptyW<'_, IrqEnbSpec> {
        FifoEmptyW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables the interrupt for FIFO full"]
    #[inline(always)]
    pub fn fifo_full(&mut self) -> FifoFullW<'_, IrqEnbSpec> {
        FifoFullW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables the interrupt for a FIFO overflow"]
    #[inline(always)]
    pub fn fifo_oflow(&mut self) -> FifoOflowW<'_, IrqEnbSpec> {
        FifoOflowW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables the interrupt for a FIFO underflow"]
    #[inline(always)]
    pub fn fifo_uflow(&mut self) -> FifoUflowW<'_, IrqEnbSpec> {
        FifoUflowW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables the interrupt for a DAC data acquisition completion"]
    #[inline(always)]
    pub fn dac_done(&mut self) -> DacDoneW<'_, IrqEnbSpec> {
        DacDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - Enables the interrupt for a trigger error"]
    #[inline(always)]
    pub fn trig_error(&mut self) -> TrigErrorW<'_, IrqEnbSpec> {
        TrigErrorW::new(self, 5)
    }
    #[doc = "Bit 6 - Enables the interrupt for the FIFO entry count is less than or equal to the trigger level"]
    #[inline(always)]
    pub fn fifo_depth_trig(&mut self) -> FifoDepthTrigW<'_, IrqEnbSpec> {
        FifoDepthTrigW::new(self, 6)
    }
}
#[doc = "Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEnbSpec;
impl crate::RegisterSpec for IrqEnbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_enb::R`](R) reader structure"]
impl crate::Readable for IrqEnbSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_enb::W`](W) writer structure"]
impl crate::Writable for IrqEnbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_ENB to value 0"]
impl crate::Resettable for IrqEnbSpec {}
