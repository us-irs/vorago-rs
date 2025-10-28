#[doc = "Register `CLKSCALE` reader"]
pub type R = crate::R<ClkscaleSpec>;
#[doc = "Register `CLKSCALE` writer"]
pub type W = crate::W<ClkscaleSpec>;
#[doc = "Field `VALUE` reader - Enable FastMode"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Enable FastMode"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `FASTMODE` reader - Enable FastMode"]
pub type FastmodeR = crate::BitReader;
#[doc = "Field `FASTMODE` writer - Enable FastMode"]
pub type FastmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Enable FastMode"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Enable FastMode"]
    #[inline(always)]
    pub fn fastmode(&self) -> FastmodeR {
        FastmodeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Enable FastMode"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, ClkscaleSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable FastMode"]
    #[inline(always)]
    pub fn fastmode(&mut self) -> FastmodeW<'_, ClkscaleSpec> {
        FastmodeW::new(self, 31)
    }
}
#[doc = "Clock Scale divide value\n\nYou can [`read`](crate::Reg::read) this register and get [`clkscale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkscale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkscaleSpec;
impl crate::RegisterSpec for ClkscaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkscale::R`](R) reader structure"]
impl crate::Readable for ClkscaleSpec {}
#[doc = "`write(|w| ..)` method takes [`clkscale::W`](W) writer structure"]
impl crate::Writable for ClkscaleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKSCALE to value 0"]
impl crate::Resettable for ClkscaleSpec {}
