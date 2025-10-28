#[doc = "Register `DAC0_CAL` reader"]
pub type R = crate::R<Dac0CalSpec>;
#[doc = "Field `DAC0_CAL` reader - DAC0 Calibration bits"]
pub type Dac0CalR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - DAC0 Calibration bits"]
    #[inline(always)]
    pub fn dac0_cal(&self) -> Dac0CalR {
        Dac0CalR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "DAC0 Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac0_cal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac0CalSpec;
impl crate::RegisterSpec for Dac0CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0_cal::R`](R) reader structure"]
impl crate::Readable for Dac0CalSpec {}
#[doc = "`reset()` method sets DAC0_CAL to value 0"]
impl crate::Resettable for Dac0CalSpec {}
