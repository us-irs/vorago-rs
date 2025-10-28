#[doc = "Register `BG_CAL` reader"]
pub type R = crate::R<BgCalSpec>;
#[doc = "Field `BG_CAL` reader - Bandgap Calibration bits"]
pub type BgCalR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Bandgap Calibration bits"]
    #[inline(always)]
    pub fn bg_cal(&self) -> BgCalR {
        BgCalR::new((self.bits & 7) as u8)
    }
}
#[doc = "Bandgap Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bg_cal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgCalSpec;
impl crate::RegisterSpec for BgCalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bg_cal::R`](R) reader structure"]
impl crate::Readable for BgCalSpec {}
#[doc = "`reset()` method sets BG_CAL to value 0"]
impl crate::Resettable for BgCalSpec {}
