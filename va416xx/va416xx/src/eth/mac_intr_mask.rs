#[doc = "Register `MAC_INTR_MASK` reader"]
pub type R = crate::R<MacIntrMaskSpec>;
#[doc = "Register `MAC_INTR_MASK` writer"]
pub type W = crate::W<MacIntrMaskSpec>;
#[doc = "Field `TSIM` reader - Timestamp Interrupt Mask"]
pub type TsimR = crate::BitReader;
#[doc = "Field `TSIM` writer - Timestamp Interrupt Mask"]
pub type TsimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&self) -> TsimR {
        TsimR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&mut self) -> TsimW<'_, MacIntrMaskSpec> {
        TsimW::new(self, 9)
    }
}
#[doc = "Contains the masks for generating interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacIntrMaskSpec;
impl crate::RegisterSpec for MacIntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_intr_mask::R`](R) reader structure"]
impl crate::Readable for MacIntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_intr_mask::W`](W) writer structure"]
impl crate::Writable for MacIntrMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_INTR_MASK to value 0"]
impl crate::Resettable for MacIntrMaskSpec {}
