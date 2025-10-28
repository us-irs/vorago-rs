#[doc = "Register `AREG_CAL` reader"]
pub type R = crate::R<AregCalSpec>;
#[doc = "Field `AREG_CAL` reader - Analog LDO Regulator Calibration bits"]
pub type AregCalR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Analog LDO Regulator Calibration bits"]
    #[inline(always)]
    pub fn areg_cal(&self) -> AregCalR {
        AregCalR::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Analog LDO Regulator Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`areg_cal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AregCalSpec;
impl crate::RegisterSpec for AregCalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`areg_cal::R`](R) reader structure"]
impl crate::Readable for AregCalSpec {}
#[doc = "`reset()` method sets AREG_CAL to value 0"]
impl crate::Resettable for AregCalSpec {}
