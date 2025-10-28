#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `MASTER_ENABLE` reader - Enable status of the controller"]
pub type MasterEnableR = crate::BitReader;
#[doc = "Field `STATE` reader - Current State of the control state machine"]
pub type StateR = crate::FieldReader;
#[doc = "Field `CHNLS_MINUS1` reader - Number of Available Channels Minus 1"]
pub type ChnlsMinus1R = crate::FieldReader;
#[doc = "Field `TEST_STATUS` reader - Test Status Logic Included"]
pub type TestStatusR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Enable status of the controller"]
    #[inline(always)]
    pub fn master_enable(&self) -> MasterEnableR {
        MasterEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Current State of the control state machine"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Number of Available Channels Minus 1"]
    #[inline(always)]
    pub fn chnls_minus1(&self) -> ChnlsMinus1R {
        ChnlsMinus1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - Test Status Logic Included"]
    #[inline(always)]
    pub fn test_status(&self) -> TestStatusR {
        TestStatusR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "DMA Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
