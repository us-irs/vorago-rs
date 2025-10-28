#[doc = "Register `PERID` reader"]
pub type R = crate::R<PeridSpec>;
#[doc = "Field `MANUFACTURER_ID` reader - MANUFACTURER_ID"]
pub type ManufacturerIdR = crate::FieldReader<u16>;
#[doc = "Field `PERIPHERAL_ID` reader - PERIPHERAL_ID"]
pub type PeripheralIdR = crate::FieldReader;
#[doc = "Field `PERIPHERAL_VER` reader - PERIPHERAL_VER"]
pub type PeripheralVerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - MANUFACTURER_ID"]
    #[inline(always)]
    pub fn manufacturer_id(&self) -> ManufacturerIdR {
        ManufacturerIdR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:23 - PERIPHERAL_ID"]
    #[inline(always)]
    pub fn peripheral_id(&self) -> PeripheralIdR {
        PeripheralIdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - PERIPHERAL_VER"]
    #[inline(always)]
    pub fn peripheral_ver(&self) -> PeripheralVerR {
        PeripheralVerR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeridSpec;
impl crate::RegisterSpec for PeridSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perid::R`](R) reader structure"]
impl crate::Readable for PeridSpec {}
#[doc = "`reset()` method sets PERID to value 0x0280_07e9"]
impl crate::Resettable for PeridSpec {
    const RESET_VALUE: u32 = 0x0280_07e9;
}
