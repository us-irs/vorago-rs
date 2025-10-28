#[doc = "Register `PERIPH_ID_1` reader"]
pub type R = crate::R<PeriphId1Spec>;
#[doc = "Field `PART_NUMBER_1` reader - Part Number 1"]
pub type PartNumber1R = crate::FieldReader;
#[doc = "Field `JEP106_ID_3_0` reader - Indentity Code"]
pub type Jep106Id3_0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Part Number 1"]
    #[inline(always)]
    pub fn part_number_1(&self) -> PartNumber1R {
        PartNumber1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indentity Code"]
    #[inline(always)]
    pub fn jep106_id_3_0(&self) -> Jep106Id3_0R {
        Jep106Id3_0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DMA Peripheral ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId1Spec;
impl crate::RegisterSpec for PeriphId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id_1::R`](R) reader structure"]
impl crate::Readable for PeriphId1Spec {}
#[doc = "`reset()` method sets PERIPH_ID_1 to value 0xb2"]
impl crate::Resettable for PeriphId1Spec {
    const RESET_VALUE: u32 = 0xb2;
}
