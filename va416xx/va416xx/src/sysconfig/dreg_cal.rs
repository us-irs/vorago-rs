#[doc = "Register `DREG_CAL` reader"]
pub type R = crate::R<DregCalSpec>;
#[doc = "Field `DREG_CAL` reader - Digital LDO Regulator Calibration bits"]
pub type DregCalR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Digital LDO Regulator Calibration bits"]
    #[inline(always)]
    pub fn dreg_cal(&self) -> DregCalR {
        DregCalR::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Digital LDO Regulator Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dreg_cal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DregCalSpec;
impl crate::RegisterSpec for DregCalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dreg_cal::R`](R) reader structure"]
impl crate::Readable for DregCalSpec {}
#[doc = "`reset()` method sets DREG_CAL to value 0"]
impl crate::Resettable for DregCalSpec {}
