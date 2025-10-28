#[doc = "Register `ADC_CAL` reader"]
pub type R = crate::R<AdcCalSpec>;
#[doc = "Field `ADC_CAL` reader - ADC Calibration bits"]
pub type AdcCalR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - ADC Calibration bits"]
    #[inline(always)]
    pub fn adc_cal(&self) -> AdcCalR {
        AdcCalR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "ADC Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_cal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcCalSpec;
impl crate::RegisterSpec for AdcCalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_cal::R`](R) reader structure"]
impl crate::Readable for AdcCalSpec {}
#[doc = "`reset()` method sets ADC_CAL to value 0"]
impl crate::Resettable for AdcCalSpec {}
