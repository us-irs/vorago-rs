#[doc = "Register `CTIM` reader"]
pub type R = crate::R<CtimSpec>;
#[doc = "Register `CTIM` writer"]
pub type W = crate::W<CtimSpec>;
#[doc = "Field `TSEG2` reader - Time Segment 2"]
pub type Tseg2R = crate::FieldReader;
#[doc = "Field `TSEG2` writer - Time Segment 2"]
pub type Tseg2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSEG1` reader - Time Segment 1"]
pub type Tseg1R = crate::FieldReader;
#[doc = "Field `TSEG1` writer - Time Segment 1"]
pub type Tseg1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SJW` reader - Synchronization Jump Width"]
pub type SjwR = crate::FieldReader;
#[doc = "Field `SJW` writer - Synchronization Jump Width"]
pub type SjwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PSC` reader - Prescaler Configuration"]
pub type PscR = crate::FieldReader;
#[doc = "Field `PSC` writer - Prescaler Configuration"]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:2 - Time Segment 2"]
    #[inline(always)]
    pub fn tseg2(&self) -> Tseg2R {
        Tseg2R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - Time Segment 1"]
    #[inline(always)]
    pub fn tseg1(&self) -> Tseg1R {
        Tseg1R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:8 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SjwR {
        SjwR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:15 - Prescaler Configuration"]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Time Segment 2"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> Tseg2W<'_, CtimSpec> {
        Tseg2W::new(self, 0)
    }
    #[doc = "Bits 3:6 - Time Segment 1"]
    #[inline(always)]
    pub fn tseg1(&mut self) -> Tseg1W<'_, CtimSpec> {
        Tseg1W::new(self, 3)
    }
    #[doc = "Bits 7:8 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SjwW<'_, CtimSpec> {
        SjwW::new(self, 7)
    }
    #[doc = "Bits 9:15 - Prescaler Configuration"]
    #[inline(always)]
    pub fn psc(&mut self) -> PscW<'_, CtimSpec> {
        PscW::new(self, 9)
    }
}
#[doc = "CAN Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtimSpec;
impl crate::RegisterSpec for CtimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctim::R`](R) reader structure"]
impl crate::Readable for CtimSpec {}
#[doc = "`write(|w| ..)` method takes [`ctim::W`](W) writer structure"]
impl crate::Writable for CtimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTIM to value 0"]
impl crate::Resettable for CtimSpec {}
