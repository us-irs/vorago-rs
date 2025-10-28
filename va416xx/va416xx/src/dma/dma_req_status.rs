#[doc = "Register `DMA_REQ_STATUS` reader"]
pub type R = crate::R<DmaReqStatusSpec>;
#[doc = "Register `DMA_REQ_STATUS` writer"]
pub type W = crate::W<DmaReqStatusSpec>;
#[doc = "Field `CH0` reader - DMA Request Status for this CH"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - DMA Request Status for this CH"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - DMA Request Status for this CH"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - DMA Request Status for this CH"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - DMA Request Status for this CH"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - DMA Request Status for this CH"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - DMA Request Status for this CH"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - DMA Request Status for this CH"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Request Status for this CH"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Request Status for this CH"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Request Status for this CH"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Request Status for this CH"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Request Status for this CH"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, DmaReqStatusSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Request Status for this CH"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, DmaReqStatusSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Request Status for this CH"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, DmaReqStatusSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Request Status for this CH"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, DmaReqStatusSpec> {
        Ch3W::new(self, 3)
    }
}
#[doc = "DMA Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_req_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_req_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaReqStatusSpec;
impl crate::RegisterSpec for DmaReqStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_req_status::R`](R) reader structure"]
impl crate::Readable for DmaReqStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_req_status::W`](W) writer structure"]
impl crate::Writable for DmaReqStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_REQ_STATUS to value 0"]
impl crate::Resettable for DmaReqStatusSpec {}
