#[doc = "Register `RST_BITS_COUNTER` reader"]
pub type R = crate::R<RstBitsCounterSpec>;
#[doc = "Register `RST_BITS_COUNTER` writer"]
pub type W = crate::W<RstBitsCounterSpec>;
#[doc = "Field `RST_BITS_COUNTER` reader - Writing any value to this bit resets the bits counter and TRNG valid registers"]
pub type RstBitsCounterR = crate::BitReader;
#[doc = "Field `RST_BITS_COUNTER` writer - Writing any value to this bit resets the bits counter and TRNG valid registers"]
pub type RstBitsCounterW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing any value to this bit resets the bits counter and TRNG valid registers"]
    #[inline(always)]
    pub fn rst_bits_counter(&self) -> RstBitsCounterR {
        RstBitsCounterR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing any value to this bit resets the bits counter and TRNG valid registers"]
    #[inline(always)]
    pub fn rst_bits_counter(&mut self) -> RstBitsCounterW<'_, RstBitsCounterSpec> {
        RstBitsCounterW::new(self, 0)
    }
}
#[doc = "Reset Bits Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_bits_counter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_bits_counter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstBitsCounterSpec;
impl crate::RegisterSpec for RstBitsCounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_bits_counter::R`](R) reader structure"]
impl crate::Readable for RstBitsCounterSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_bits_counter::W`](W) writer structure"]
impl crate::Writable for RstBitsCounterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RST_BITS_COUNTER to value 0"]
impl crate::Resettable for RstBitsCounterSpec {}
