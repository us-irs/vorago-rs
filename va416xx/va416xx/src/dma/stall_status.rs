#[doc = "Register `STALL_STATUS` reader"]
pub type R = crate::R<StallStatusSpec>;
#[doc = "Register `STALL_STATUS` writer"]
pub type W = crate::W<StallStatusSpec>;
#[doc = "Field `STALL_STATUS` reader - DMA is stalled"]
pub type StallStatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA is stalled"]
    #[inline(always)]
    pub fn stall_status(&self) -> StallStatusR {
        StallStatusR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "DMA stall status\n\nYou can [`read`](crate::Reg::read) this register and get [`stall_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stall_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StallStatusSpec;
impl crate::RegisterSpec for StallStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stall_status::R`](R) reader structure"]
impl crate::Readable for StallStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`stall_status::W`](W) writer structure"]
impl crate::Writable for StallStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STALL_STATUS to value 0"]
impl crate::Resettable for StallStatusSpec {}
