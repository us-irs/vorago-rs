#[doc = "Register `CANEC` reader"]
pub type R = crate::R<CanecSpec>;
#[doc = "Register `CANEC` writer"]
pub type W = crate::W<CanecSpec>;
#[doc = "Field `TEC` reader - Transmit Error Counter"]
pub type TecR = crate::FieldReader;
#[doc = "Field `TEC` writer - Transmit Error Counter"]
pub type TecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REC` reader - Receive Error Counter"]
pub type RecR = crate::FieldReader;
#[doc = "Field `REC` writer - Receive Error Counter"]
pub type RecW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&mut self) -> TecW<'_, CanecSpec> {
        TecW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&mut self) -> RecW<'_, CanecSpec> {
        RecW::new(self, 8)
    }
}
#[doc = "CAN Error Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`canec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanecSpec;
impl crate::RegisterSpec for CanecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`canec::R`](R) reader structure"]
impl crate::Readable for CanecSpec {}
#[doc = "`write(|w| ..)` method takes [`canec::W`](W) writer structure"]
impl crate::Writable for CanecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CANEC to value 0"]
impl crate::Resettable for CanecSpec {}
