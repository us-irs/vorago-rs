#[doc = "Register `DMA_AHB_STATUS` reader"]
pub type R = crate::R<DmaAhbStatusSpec>;
#[doc = "Register `DMA_AHB_STATUS` writer"]
pub type W = crate::W<DmaAhbStatusSpec>;
#[doc = "Field `AHBMASTRSTS` reader - When high, indicates that the AHB master interface FSMs are in the non-idle state"]
pub type AhbmastrstsR = crate::BitReader;
#[doc = "Field `AHBMASTRSTS` writer - When high, indicates that the AHB master interface FSMs are in the non-idle state"]
pub type AhbmastrstsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When high, indicates that the AHB master interface FSMs are in the non-idle state"]
    #[inline(always)]
    pub fn ahbmastrsts(&self) -> AhbmastrstsR {
        AhbmastrstsR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When high, indicates that the AHB master interface FSMs are in the non-idle state"]
    #[inline(always)]
    pub fn ahbmastrsts(&mut self) -> AhbmastrstsW<'_, DmaAhbStatusSpec> {
        AhbmastrstsW::new(self, 0)
    }
}
#[doc = "Provides the active status of the read and write channels of the AHB master interface\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ahb_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ahb_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaAhbStatusSpec;
impl crate::RegisterSpec for DmaAhbStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ahb_status::R`](R) reader structure"]
impl crate::Readable for DmaAhbStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_ahb_status::W`](W) writer structure"]
impl crate::Writable for DmaAhbStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_AHB_STATUS to value 0"]
impl crate::Resettable for DmaAhbStatusSpec {}
