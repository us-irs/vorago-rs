#[doc = "Register `DMA_MISS_OVER_COUNTER` reader"]
pub type R = crate::R<DmaMissOverCounterSpec>;
#[doc = "Register `DMA_MISS_OVER_COUNTER` writer"]
pub type W = crate::W<DmaMissOverCounterSpec>;
#[doc = "Field `MISFRMCNT` reader - This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable."]
pub type MisfrmcntR = crate::FieldReader<u16>;
#[doc = "Field `MISFRMCNT` writer - This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable."]
pub type MisfrmcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MISCNTOVF` reader - This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows"]
pub type MiscntovfR = crate::BitReader;
#[doc = "Field `MISCNTOVF` writer - This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows"]
pub type MiscntovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFFRMCNT` reader - This field indicates the number of frames missed by the application"]
pub type OvffrmcntR = crate::FieldReader<u16>;
#[doc = "Field `OVFFRMCNT` writer - This field indicates the number of frames missed by the application"]
pub type OvffrmcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `OVFCNTOVF` reader - This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\])overflows"]
pub type OvfcntovfR = crate::BitReader;
#[doc = "Field `OVFCNTOVF` writer - This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\])overflows"]
pub type OvfcntovfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable."]
    #[inline(always)]
    pub fn misfrmcnt(&self) -> MisfrmcntR {
        MisfrmcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MiscntovfR {
        MiscntovfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - This field indicates the number of frames missed by the application"]
    #[inline(always)]
    pub fn ovffrmcnt(&self) -> OvffrmcntR {
        OvffrmcntR::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\])overflows"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OvfcntovfR {
        OvfcntovfR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable."]
    #[inline(always)]
    pub fn misfrmcnt(&mut self) -> MisfrmcntW<'_, DmaMissOverCounterSpec> {
        MisfrmcntW::new(self, 0)
    }
    #[doc = "Bit 16 - This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows"]
    #[inline(always)]
    pub fn miscntovf(&mut self) -> MiscntovfW<'_, DmaMissOverCounterSpec> {
        MiscntovfW::new(self, 16)
    }
    #[doc = "Bits 17:27 - This field indicates the number of frames missed by the application"]
    #[inline(always)]
    pub fn ovffrmcnt(&mut self) -> OvffrmcntW<'_, DmaMissOverCounterSpec> {
        OvffrmcntW::new(self, 17)
    }
    #[doc = "Bit 28 - This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\])overflows"]
    #[inline(always)]
    pub fn ovfcntovf(&mut self) -> OvfcntovfW<'_, DmaMissOverCounterSpec> {
        OvfcntovfW::new(self, 28)
    }
}
#[doc = "Contains the counters for discarded frames because no Receive Descriptor is available\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_miss_over_counter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_miss_over_counter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaMissOverCounterSpec;
impl crate::RegisterSpec for DmaMissOverCounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_miss_over_counter::R`](R) reader structure"]
impl crate::Readable for DmaMissOverCounterSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_miss_over_counter::W`](W) writer structure"]
impl crate::Writable for DmaMissOverCounterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_MISS_OVER_COUNTER to value 0"]
impl crate::Resettable for DmaMissOverCounterSpec {}
