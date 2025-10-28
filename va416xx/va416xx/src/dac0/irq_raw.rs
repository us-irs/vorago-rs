#[doc = "Register `IRQ_RAW` reader"]
pub type R = crate::R<IrqRawSpec>;
#[doc = "Field `FIFO_EMPTY` reader - Indicates the FIFO is empty"]
pub type FifoEmptyR = crate::BitReader;
#[doc = "Field `FIFO_FULL` reader - Indicates the FIFO is full"]
pub type FifoFullR = crate::BitReader;
#[doc = "Field `FIFO_OFLOW` reader - Indicates a FIFO overflow occurred (FIFO was full when new data was written)"]
pub type FifoOflowR = crate::BitReader;
#[doc = "Field `FIFO_UFLOW` reader - Indicates data was unavailable when a new trigger for DAC update is received"]
pub type FifoUflowR = crate::BitReader;
#[doc = "Field `DAC_DONE` reader - Indicates that a DAC conversion is done"]
pub type DacDoneR = crate::BitReader;
#[doc = "Field `TRIG_ERROR` reader - Indicates a manual or external trigger occurred when the DAC was BUSY doing a conversion"]
pub type TrigErrorR = crate::BitReader;
#[doc = "Field `FIFO_DEPTH_TRIG` reader - Indicates the FIFO entry count is less than or equal to the trigger level"]
pub type FifoDepthTrigR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates the FIFO is empty"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FifoEmptyR {
        FifoEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the FIFO is full"]
    #[inline(always)]
    pub fn fifo_full(&self) -> FifoFullR {
        FifoFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates a FIFO overflow occurred (FIFO was full when new data was written)"]
    #[inline(always)]
    pub fn fifo_oflow(&self) -> FifoOflowR {
        FifoOflowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates data was unavailable when a new trigger for DAC update is received"]
    #[inline(always)]
    pub fn fifo_uflow(&self) -> FifoUflowR {
        FifoUflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that a DAC conversion is done"]
    #[inline(always)]
    pub fn dac_done(&self) -> DacDoneR {
        DacDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates a manual or external trigger occurred when the DAC was BUSY doing a conversion"]
    #[inline(always)]
    pub fn trig_error(&self) -> TrigErrorR {
        TrigErrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates the FIFO entry count is less than or equal to the trigger level"]
    #[inline(always)]
    pub fn fifo_depth_trig(&self) -> FifoDepthTrigR {
        FifoDepthTrigR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Raw Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqRawSpec;
impl crate::RegisterSpec for IrqRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_raw::R`](R) reader structure"]
impl crate::Readable for IrqRawSpec {}
#[doc = "`reset()` method sets IRQ_RAW to value 0x41"]
impl crate::Resettable for IrqRawSpec {
    const RESET_VALUE: u32 = 0x41;
}
