#[doc = "Register `IRQ_END` reader"]
pub type R = crate::R<IrqEndSpec>;
#[doc = "Field `FIFO_EMPTY` reader - Indicates the FIFO is empty and the interrupt is enabled"]
pub type FifoEmptyR = crate::BitReader;
#[doc = "Field `FIFO_FULL` reader - Indicates the FIFO is full and the interrupt is enabled"]
pub type FifoFullR = crate::BitReader;
#[doc = "Field `FIFO_OFLOW` reader - Indicates a FIFO overflow occurred and the interrupt is enabled"]
pub type FifoOflowR = crate::BitReader;
#[doc = "Field `FIFO_UFLOW` reader - Indicates a FIFO underflow occurred and the interrupt is enabled"]
pub type FifoUflowR = crate::BitReader;
#[doc = "Field `ADC_DONE` reader - Indicates that a ADC conversion is done and the interrupt is enabled"]
pub type AdcDoneR = crate::BitReader;
#[doc = "Field `TRIG_ERROR` reader - Indicates a manual or external trigger occurred when the ADC was BUSY doing a conversion and the interrupt is enabled"]
pub type TrigErrorR = crate::BitReader;
#[doc = "Field `FIFO_DEPTH_TRIG` reader - Indicates the interrupt for the FIFO entry count meets or exceeds the trigger level"]
pub type FifoDepthTrigR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates the FIFO is empty and the interrupt is enabled"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FifoEmptyR {
        FifoEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the FIFO is full and the interrupt is enabled"]
    #[inline(always)]
    pub fn fifo_full(&self) -> FifoFullR {
        FifoFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates a FIFO overflow occurred and the interrupt is enabled"]
    #[inline(always)]
    pub fn fifo_oflow(&self) -> FifoOflowR {
        FifoOflowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates a FIFO underflow occurred and the interrupt is enabled"]
    #[inline(always)]
    pub fn fifo_uflow(&self) -> FifoUflowR {
        FifoUflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that a ADC conversion is done and the interrupt is enabled"]
    #[inline(always)]
    pub fn adc_done(&self) -> AdcDoneR {
        AdcDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates a manual or external trigger occurred when the ADC was BUSY doing a conversion and the interrupt is enabled"]
    #[inline(always)]
    pub fn trig_error(&self) -> TrigErrorR {
        TrigErrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates the interrupt for the FIFO entry count meets or exceeds the trigger level"]
    #[inline(always)]
    pub fn fifo_depth_trig(&self) -> FifoDepthTrigR {
        FifoDepthTrigR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Enabled Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_end::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEndSpec;
impl crate::RegisterSpec for IrqEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_end::R`](R) reader structure"]
impl crate::Readable for IrqEndSpec {}
#[doc = "`reset()` method sets IRQ_END to value 0"]
impl crate::Resettable for IrqEndSpec {}
