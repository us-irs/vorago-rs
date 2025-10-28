#[doc = "Register `DMA_DONE_CLR` reader"]
pub type R = crate::R<DmaDoneClrSpec>;
#[doc = "Register `DMA_DONE_CLR` writer"]
pub type W = crate::W<DmaDoneClrSpec>;
#[doc = "Field `CH0` reader - DMA Done clear for this CH"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - DMA Done clear for this CH"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - DMA Done clear for this CH"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - DMA Done clear for this CH"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - DMA Done clear for this CH"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - DMA Done clear for this CH"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - DMA Done clear for this CH"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - DMA Done clear for this CH"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Done clear for this CH"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Done clear for this CH"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Done clear for this CH"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Done clear for this CH"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Done clear for this CH"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, DmaDoneClrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Done clear for this CH"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, DmaDoneClrSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Done clear for this CH"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, DmaDoneClrSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Done clear for this CH"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, DmaDoneClrSpec> {
        Ch3W::new(self, 3)
    }
}
#[doc = "DMA done clear\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_done_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_done_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaDoneClrSpec;
impl crate::RegisterSpec for DmaDoneClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_done_clr::R`](R) reader structure"]
impl crate::Readable for DmaDoneClrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_done_clr::W`](W) writer structure"]
impl crate::Writable for DmaDoneClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_DONE_CLR to value 0"]
impl crate::Resettable for DmaDoneClrSpec {}
