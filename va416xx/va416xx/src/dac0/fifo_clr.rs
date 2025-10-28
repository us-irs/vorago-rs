#[doc = "Register `FIFO_CLR` reader"]
pub type R = crate::R<FifoClrSpec>;
#[doc = "Register `FIFO_CLR` writer"]
pub type W = crate::W<FifoClrSpec>;
#[doc = "Field `FIFO_CLR` writer - Clears the DAC FIFO. Always reads 0"]
pub type FifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clears the DAC FIFO. Always reads 0"]
    #[inline(always)]
    pub fn fifo_clr(&mut self) -> FifoClrW<'_, FifoClrSpec> {
        FifoClrW::new(self, 0)
    }
}
#[doc = "FIFO Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoClrSpec;
impl crate::RegisterSpec for FifoClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_clr::R`](R) reader structure"]
impl crate::Readable for FifoClrSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_clr::W`](W) writer structure"]
impl crate::Writable for FifoClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO_CLR to value 0"]
impl crate::Resettable for FifoClrSpec {}
