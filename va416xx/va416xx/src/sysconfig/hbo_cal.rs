#[doc = "Register `HBO_CAL` reader"]
pub type R = crate::R<HboCalSpec>;
#[doc = "Field `HBO_CAL` reader - Heart Beat OSC Calibration bits"]
pub type HboCalR = crate::FieldReader;
#[doc = "Field `OSC_CAL` reader - 1MHz OSC Calibration bit"]
pub type OscCalR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Heart Beat OSC Calibration bits"]
    #[inline(always)]
    pub fn hbo_cal(&self) -> HboCalR {
        HboCalR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 1MHz OSC Calibration bit"]
    #[inline(always)]
    pub fn osc_cal(&self) -> OscCalR {
        OscCalR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Heart Beat OSC Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hbo_cal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HboCalSpec;
impl crate::RegisterSpec for HboCalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbo_cal::R`](R) reader structure"]
impl crate::Readable for HboCalSpec {}
#[doc = "`reset()` method sets HBO_CAL to value 0"]
impl crate::Resettable for HboCalSpec {}
