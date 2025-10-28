#[doc = "Register `DMA_ACTIVE_SET` reader"]
pub type R = crate::R<DmaActiveSetSpec>;
#[doc = "Register `DMA_ACTIVE_SET` writer"]
pub type W = crate::W<DmaActiveSetSpec>;
#[doc = "Field `CH0` reader - DMA Active Set"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - DMA Active Set"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - DMA Active Set"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - DMA Active Set"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - DMA Active Set"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - DMA Active Set"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - DMA Active Set"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - DMA Active Set"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Active Set"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Active Set"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Active Set"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Active Set"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Active Set"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, DmaActiveSetSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Active Set"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, DmaActiveSetSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Active Set"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, DmaActiveSetSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Active Set"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, DmaActiveSetSpec> {
        Ch3W::new(self, 3)
    }
}
#[doc = "DMA active set\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_active_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_active_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaActiveSetSpec;
impl crate::RegisterSpec for DmaActiveSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_active_set::R`](R) reader structure"]
impl crate::Readable for DmaActiveSetSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_active_set::W`](W) writer structure"]
impl crate::Writable for DmaActiveSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_ACTIVE_SET to value 0"]
impl crate::Resettable for DmaActiveSetSpec {}
