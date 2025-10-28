#[doc = "Register `DMA_SREQ_STATUS` reader"]
pub type R = crate::R<DmaSreqStatusSpec>;
#[doc = "Register `DMA_SREQ_STATUS` writer"]
pub type W = crate::W<DmaSreqStatusSpec>;
#[doc = "Field `CH0` reader - DMA SRequest Status for this CH"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - DMA SRequest Status for this CH"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - DMA SRequest Status for this CH"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - DMA SRequest Status for this CH"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - DMA SRequest Status for this CH"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - DMA SRequest Status for this CH"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - DMA SRequest Status for this CH"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - DMA SRequest Status for this CH"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA SRequest Status for this CH"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA SRequest Status for this CH"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA SRequest Status for this CH"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA SRequest Status for this CH"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA SRequest Status for this CH"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, DmaSreqStatusSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA SRequest Status for this CH"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, DmaSreqStatusSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA SRequest Status for this CH"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, DmaSreqStatusSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA SRequest Status for this CH"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, DmaSreqStatusSpec> {
        Ch3W::new(self, 3)
    }
}
#[doc = "DMA single request status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sreq_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sreq_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSreqStatusSpec;
impl crate::RegisterSpec for DmaSreqStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_sreq_status::R`](R) reader structure"]
impl crate::Readable for DmaSreqStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_sreq_status::W`](W) writer structure"]
impl crate::Writable for DmaSreqStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_SREQ_STATUS to value 0"]
impl crate::Resettable for DmaSreqStatusSpec {}
