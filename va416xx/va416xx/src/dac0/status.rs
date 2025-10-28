#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `FIFO_ENTRY_CNT` reader - Indicates the number of entries in the FIFO"]
pub type FifoEntryCntR = crate::FieldReader;
#[doc = "Field `DAC_BUSY` reader - Indicates a DAC data acquisition is in process"]
pub type DacBusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Indicates the number of entries in the FIFO"]
    #[inline(always)]
    pub fn fifo_entry_cnt(&self) -> FifoEntryCntR {
        FifoEntryCntR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Indicates a DAC data acquisition is in process"]
    #[inline(always)]
    pub fn dac_busy(&self) -> DacBusyR {
        DacBusyR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
