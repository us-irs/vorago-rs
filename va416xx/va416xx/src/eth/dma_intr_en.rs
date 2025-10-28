#[doc = "Register `DMA_INTR_EN` reader"]
pub type R = crate::R<DmaIntrEnSpec>;
#[doc = "Register `DMA_INTR_EN` writer"]
pub type W = crate::W<DmaIntrEnSpec>;
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - Transmit Stopped Enable"]
pub type TseR = crate::BitReader;
#[doc = "Field `TSE` writer - Transmit Stopped Enable"]
pub type TseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUE` reader - Transmit Buffer Unavailable Enable"]
pub type TueR = crate::BitReader;
#[doc = "Field `TUE` writer - Transmit Buffer Unavailable Enable"]
pub type TueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THE` reader - Transmit Jabber Timeout Enable"]
pub type TheR = crate::BitReader;
#[doc = "Field `THE` writer - Transmit Jabber Timeout Enable"]
pub type TheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVE` reader - Overflow Interrupt Enable"]
pub type OveR = crate::BitReader;
#[doc = "Field `OVE` writer - Overflow Interrupt Enable"]
pub type OveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNE` reader - Underflow Interrupt Enable"]
pub type UneR = crate::BitReader;
#[doc = "Field `UNE` writer - Underflow Interrupt Enable"]
pub type UneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive Interrupt Enable"]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - Receive Interrupt Enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUE` reader - Receive Buffer Unavailable Enable"]
pub type RueR = crate::BitReader;
#[doc = "Field `RUE` writer - Receive Buffer Unavailable Enable"]
pub type RueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSE` reader - Receive Stopped Enable"]
pub type RseR = crate::BitReader;
#[doc = "Field `RSE` writer - Receive Stopped Enable"]
pub type RseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWE` reader - Receive Watchdog Timeout Enable"]
pub type RweR = crate::BitReader;
#[doc = "Field `RWE` writer - Receive Watchdog Timeout Enable"]
pub type RweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETE` reader - Early Transmit Interrupt Enable"]
pub type EteR = crate::BitReader;
#[doc = "Field `ETE` writer - Early Transmit Interrupt Enable"]
pub type EteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error Enable"]
pub type FbeR = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Enable"]
pub type FbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERE` reader - Early Receive Interrupt Enable"]
pub type EreR = crate::BitReader;
#[doc = "Field `ERE` writer - Early Receive Interrupt Enable"]
pub type EreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIE` reader - Abnormal Interrupt Summary Enable"]
pub type AieR = crate::BitReader;
#[doc = "Field `AIE` writer - Abnormal Interrupt Summary Enable"]
pub type AieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIE` reader - Normal Interrupt Summary Enable"]
pub type NieR = crate::BitReader;
#[doc = "Field `NIE` writer - Normal Interrupt Summary Enable"]
pub type NieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        TseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn tue(&self) -> TueR {
        TueR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable"]
    #[inline(always)]
    pub fn the(&self) -> TheR {
        TheR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ove(&self) -> OveR {
        OveR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn une(&self) -> UneR {
        UneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn rue(&self) -> RueR {
        RueR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn rse(&self) -> RseR {
        RseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn rwe(&self) -> RweR {
        RweR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn ete(&self) -> EteR {
        EteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn fbe(&self) -> FbeR {
        FbeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ere(&self) -> EreR {
        EreR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn aie(&self) -> AieR {
        AieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn nie(&self) -> NieR {
        NieR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, DmaIntrEnSpec> {
        TieW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn tse(&mut self) -> TseW<'_, DmaIntrEnSpec> {
        TseW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn tue(&mut self) -> TueW<'_, DmaIntrEnSpec> {
        TueW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable"]
    #[inline(always)]
    pub fn the(&mut self) -> TheW<'_, DmaIntrEnSpec> {
        TheW::new(self, 3)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ove(&mut self) -> OveW<'_, DmaIntrEnSpec> {
        OveW::new(self, 4)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn une(&mut self) -> UneW<'_, DmaIntrEnSpec> {
        UneW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<'_, DmaIntrEnSpec> {
        RieW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn rue(&mut self) -> RueW<'_, DmaIntrEnSpec> {
        RueW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn rse(&mut self) -> RseW<'_, DmaIntrEnSpec> {
        RseW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn rwe(&mut self) -> RweW<'_, DmaIntrEnSpec> {
        RweW::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn ete(&mut self) -> EteW<'_, DmaIntrEnSpec> {
        EteW::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn fbe(&mut self) -> FbeW<'_, DmaIntrEnSpec> {
        FbeW::new(self, 13)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ere(&mut self) -> EreW<'_, DmaIntrEnSpec> {
        EreW::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn aie(&mut self) -> AieW<'_, DmaIntrEnSpec> {
        AieW::new(self, 15)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn nie(&mut self) -> NieW<'_, DmaIntrEnSpec> {
        NieW::new(self, 16)
    }
}
#[doc = "Enables the interrupts reported in the status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_intr_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_intr_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntrEnSpec;
impl crate::RegisterSpec for DmaIntrEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_intr_en::R`](R) reader structure"]
impl crate::Readable for DmaIntrEnSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_intr_en::W`](W) writer structure"]
impl crate::Writable for DmaIntrEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_INTR_EN to value 0"]
impl crate::Resettable for DmaIntrEnSpec {}
