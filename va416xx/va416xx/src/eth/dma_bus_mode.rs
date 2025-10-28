#[doc = "Register `DMA_BUS_MODE` reader"]
pub type R = crate::R<DmaBusModeSpec>;
#[doc = "Register `DMA_BUS_MODE` writer"]
pub type W = crate::W<DmaBusModeSpec>;
#[doc = "Field `SWR` reader - Software Reset (Read, Write Set, and Self Clear)"]
pub type SwrR = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset (Read, Write Set, and Self Clear)"]
pub type SwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - DMA Arbitration Scheme"]
pub type DaR = crate::BitReader;
#[doc = "Field `DA` writer - DMA Arbitration Scheme"]
pub type DaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor Skip Length"]
pub type DslR = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor Skip Length"]
pub type DslW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PBL` reader - Programmable Burst Lengthe"]
pub type PblR = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable Burst Lengthe"]
pub type PblW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PR` reader - Priority Ratio"]
pub type PrR = crate::FieldReader;
#[doc = "Field `PR` writer - Priority Ratio"]
pub type PrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FB` reader - Fixed Burste"]
pub type FbR = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burste"]
pub type FbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPBL` reader - Rx DMA PBL"]
pub type RpblR = crate::FieldReader;
#[doc = "Field `RPBL` writer - Rx DMA PBL"]
pub type RpblW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `USP` reader - Use Separate PBL"]
pub type UspR = crate::BitReader;
#[doc = "Field `USP` writer - Use Separate PBL"]
pub type UspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBLx8` reader - PBLx8 Mode"]
pub type Pblx8R = crate::BitReader;
#[doc = "Field `PBLx8` writer - PBLx8 Mode"]
pub type Pblx8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAL` reader - Address-Aligned Beats"]
pub type AalR = crate::BitReader;
#[doc = "Field `AAL` writer - Address-Aligned Beats"]
pub type AalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB` reader - Mixed Burst"]
pub type MbR = crate::BitReader;
#[doc = "Field `MB` writer - Mixed Burst"]
pub type MbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPR` reader - Transmit Priority"]
pub type TxprR = crate::BitReader;
#[doc = "Field `TXPR` writer - Transmit Priority"]
pub type TxprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRWG` reader - Channel Priority Weights"]
pub type PrwgR = crate::FieldReader;
#[doc = "Field `PRWG` writer - Channel Priority Weights"]
pub type PrwgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RIB` reader - Rebuild INCRx Burst"]
pub type RibR = crate::BitReader;
#[doc = "Field `RIB` writer - Rebuild INCRx Burst"]
pub type RibW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset (Read, Write Set, and Self Clear)"]
    #[inline(always)]
    pub fn swr(&self) -> SwrR {
        SwrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme"]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn dsl(&self) -> DslR {
        DslR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Programmable Burst Lengthe"]
    #[inline(always)]
    pub fn pbl(&self) -> PblR {
        PblR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Priority Ratio"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed Burste"]
    #[inline(always)]
    pub fn fb(&self) -> FbR {
        FbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rpbl(&self) -> RpblR {
        RpblR::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use Separate PBL"]
    #[inline(always)]
    pub fn usp(&self) -> UspR {
        UspR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PBLx8 Mode"]
    #[inline(always)]
    pub fn pblx8(&self) -> Pblx8R {
        Pblx8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&self) -> AalR {
        AalR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        MbR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    pub fn txpr(&self) -> TxprR {
        TxprR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Channel Priority Weights"]
    #[inline(always)]
    pub fn prwg(&self) -> PrwgR {
        PrwgR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Rebuild INCRx Burst"]
    #[inline(always)]
    pub fn rib(&self) -> RibR {
        RibR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset (Read, Write Set, and Self Clear)"]
    #[inline(always)]
    pub fn swr(&mut self) -> SwrW<'_, DmaBusModeSpec> {
        SwrW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme"]
    #[inline(always)]
    pub fn da(&mut self) -> DaW<'_, DmaBusModeSpec> {
        DaW::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn dsl(&mut self) -> DslW<'_, DmaBusModeSpec> {
        DslW::new(self, 2)
    }
    #[doc = "Bits 8:13 - Programmable Burst Lengthe"]
    #[inline(always)]
    pub fn pbl(&mut self) -> PblW<'_, DmaBusModeSpec> {
        PblW::new(self, 8)
    }
    #[doc = "Bits 14:15 - Priority Ratio"]
    #[inline(always)]
    pub fn pr(&mut self) -> PrW<'_, DmaBusModeSpec> {
        PrW::new(self, 14)
    }
    #[doc = "Bit 16 - Fixed Burste"]
    #[inline(always)]
    pub fn fb(&mut self) -> FbW<'_, DmaBusModeSpec> {
        FbW::new(self, 16)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rpbl(&mut self) -> RpblW<'_, DmaBusModeSpec> {
        RpblW::new(self, 17)
    }
    #[doc = "Bit 23 - Use Separate PBL"]
    #[inline(always)]
    pub fn usp(&mut self) -> UspW<'_, DmaBusModeSpec> {
        UspW::new(self, 23)
    }
    #[doc = "Bit 24 - PBLx8 Mode"]
    #[inline(always)]
    pub fn pblx8(&mut self) -> Pblx8W<'_, DmaBusModeSpec> {
        Pblx8W::new(self, 24)
    }
    #[doc = "Bit 25 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&mut self) -> AalW<'_, DmaBusModeSpec> {
        AalW::new(self, 25)
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&mut self) -> MbW<'_, DmaBusModeSpec> {
        MbW::new(self, 26)
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    pub fn txpr(&mut self) -> TxprW<'_, DmaBusModeSpec> {
        TxprW::new(self, 27)
    }
    #[doc = "Bits 28:29 - Channel Priority Weights"]
    #[inline(always)]
    pub fn prwg(&mut self) -> PrwgW<'_, DmaBusModeSpec> {
        PrwgW::new(self, 28)
    }
    #[doc = "Bit 31 - Rebuild INCRx Burst"]
    #[inline(always)]
    pub fn rib(&mut self) -> RibW<'_, DmaBusModeSpec> {
        RibW::new(self, 31)
    }
}
#[doc = "Controls the DMA Host Interface Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_bus_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_bus_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaBusModeSpec;
impl crate::RegisterSpec for DmaBusModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_bus_mode::R`](R) reader structure"]
impl crate::Readable for DmaBusModeSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_bus_mode::W`](W) writer structure"]
impl crate::Writable for DmaBusModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_BUS_MODE to value 0x0002_0101"]
impl crate::Resettable for DmaBusModeSpec {
    const RESET_VALUE: u32 = 0x0002_0101;
}
