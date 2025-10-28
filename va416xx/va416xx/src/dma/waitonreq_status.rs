#[doc = "Register `WAITONREQ_STATUS` reader"]
pub type R = crate::R<WaitonreqStatusSpec>;
#[doc = "Field `CH0` reader - DMA wait on request"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH1` reader - DMA wait on request"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH2` reader - DMA wait on request"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH3` reader - DMA wait on request"]
pub type Ch3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA wait on request"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA wait on request"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA wait on request"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA wait on request"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "DMA channel wait on request status\n\nYou can [`read`](crate::Reg::read) this register and get [`waitonreq_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaitonreqStatusSpec;
impl crate::RegisterSpec for WaitonreqStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`waitonreq_status::R`](R) reader structure"]
impl crate::Readable for WaitonreqStatusSpec {}
#[doc = "`reset()` method sets WAITONREQ_STATUS to value 0"]
impl crate::Resettable for WaitonreqStatusSpec {}
