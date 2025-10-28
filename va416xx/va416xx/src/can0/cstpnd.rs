#[doc = "Register `CSTPND` reader"]
pub type R = crate::R<CstpndSpec>;
#[doc = "Register `CSTPND` writer"]
pub type W = crate::W<CstpndSpec>;
#[doc = "Field `IST` reader - Interrupt Source portion of Interrupt Code"]
pub type IstR = crate::FieldReader;
#[doc = "Field `IST` writer - Interrupt Source portion of Interrupt Code"]
pub type IstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IRQ` reader - Interrupt Request portion of Interrupt Code"]
pub type IrqR = crate::BitReader;
#[doc = "Field `IRQ` writer - Interrupt Request portion of Interrupt Code"]
pub type IrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS` reader - CAN Node Status"]
pub type NsR = crate::FieldReader;
#[doc = "Field `NS` writer - CAN Node Status"]
pub type NsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Interrupt Source portion of Interrupt Code"]
    #[inline(always)]
    pub fn ist(&self) -> IstR {
        IstR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Interrupt Request portion of Interrupt Code"]
    #[inline(always)]
    pub fn irq(&self) -> IrqR {
        IrqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - CAN Node Status"]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Interrupt Source portion of Interrupt Code"]
    #[inline(always)]
    pub fn ist(&mut self) -> IstW<'_, CstpndSpec> {
        IstW::new(self, 0)
    }
    #[doc = "Bit 4 - Interrupt Request portion of Interrupt Code"]
    #[inline(always)]
    pub fn irq(&mut self) -> IrqW<'_, CstpndSpec> {
        IrqW::new(self, 4)
    }
    #[doc = "Bits 5:7 - CAN Node Status"]
    #[inline(always)]
    pub fn ns(&mut self) -> NsW<'_, CstpndSpec> {
        NsW::new(self, 5)
    }
}
#[doc = "CAN Status Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cstpnd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cstpnd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CstpndSpec;
impl crate::RegisterSpec for CstpndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cstpnd::R`](R) reader structure"]
impl crate::Readable for CstpndSpec {}
#[doc = "`write(|w| ..)` method takes [`cstpnd::W`](W) writer structure"]
impl crate::Writable for CstpndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSTPND to value 0"]
impl crate::Resettable for CstpndSpec {}
