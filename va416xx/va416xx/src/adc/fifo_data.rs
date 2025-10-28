#[doc = "Register `FIFO_DATA` reader"]
pub type R = crate::R<FifoDataSpec>;
#[doc = "Field `ADC_DATA` reader - ADC acquisition data from the FIFO"]
pub type AdcDataR = crate::FieldReader<u16>;
#[doc = "Field `CHAN_TAG` reader - If enabled, this will include the number of the channel corresponding to the measurement"]
pub type ChanTagR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - ADC acquisition data from the FIFO"]
    #[inline(always)]
    pub fn adc_data(&self) -> AdcDataR {
        AdcDataR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - If enabled, this will include the number of the channel corresponding to the measurement"]
    #[inline(always)]
    pub fn chan_tag(&self) -> ChanTagR {
        ChanTagR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "FIFO data\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoDataSpec;
impl crate::RegisterSpec for FifoDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_data::R`](R) reader structure"]
impl crate::Readable for FifoDataSpec {}
#[doc = "`reset()` method sets FIFO_DATA to value 0"]
impl crate::Resettable for FifoDataSpec {}
