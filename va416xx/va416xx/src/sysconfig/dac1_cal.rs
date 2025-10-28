#[doc = "Register `DAC1_CAL` reader"]
pub type R = crate::R<Dac1CalSpec>;
#[doc = "Field `DAC1_CAL` reader - DAC1 Calibration bits"]
pub type Dac1CalR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - DAC1 Calibration bits"]
    #[inline(always)]
    pub fn dac1_cal(&self) -> Dac1CalR {
        Dac1CalR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "DAC1 Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac1_cal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac1CalSpec;
impl crate::RegisterSpec for Dac1CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1_cal::R`](R) reader structure"]
impl crate::Readable for Dac1CalSpec {}
#[doc = "`reset()` method sets DAC1_CAL to value 0"]
impl crate::Resettable for Dac1CalSpec {}
