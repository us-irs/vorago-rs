#[doc = "Register `IRQ_ENB` reader"]
pub type R = crate::R<IrqEnbSpec>;
#[doc = "Register `IRQ_ENB` writer"]
pub type W = crate::W<IrqEnbSpec>;
#[doc = "Field `ROMMBE` reader - ROM Multi Bit Interrupt"]
pub type RommbeR = crate::BitReader;
#[doc = "Field `ROMMBE` writer - ROM Multi Bit Interrupt"]
pub type RommbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROMSBE` reader - ROM Single Bit Interrupt"]
pub type RomsbeR = crate::BitReader;
#[doc = "Field `ROMSBE` writer - ROM Single Bit Interrupt"]
pub type RomsbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM0MBE` reader - RAM0 Multi Bit Interrupt"]
pub type Ram0mbeR = crate::BitReader;
#[doc = "Field `RAM0MBE` writer - RAM0 Multi Bit Interrupt"]
pub type Ram0mbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM0SBE` reader - RAM0 Single Bit Interrupt"]
pub type Ram0sbeR = crate::BitReader;
#[doc = "Field `RAM0SBE` writer - RAM0 Single Bit Interrupt"]
pub type Ram0sbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1MBE` reader - RAM1 Multi Bit Interrupt"]
pub type Ram1mbeR = crate::BitReader;
#[doc = "Field `RAM1MBE` writer - RAM1 Multi Bit Interrupt"]
pub type Ram1mbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM1SBE` reader - RAM1 Single Bit Interrupt"]
pub type Ram1sbeR = crate::BitReader;
#[doc = "Field `RAM1SBE` writer - RAM1 Single Bit Interrupt"]
pub type Ram1sbeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ROM Multi Bit Interrupt"]
    #[inline(always)]
    pub fn rommbe(&self) -> RommbeR {
        RommbeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ROM Single Bit Interrupt"]
    #[inline(always)]
    pub fn romsbe(&self) -> RomsbeR {
        RomsbeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM0 Multi Bit Interrupt"]
    #[inline(always)]
    pub fn ram0mbe(&self) -> Ram0mbeR {
        Ram0mbeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM0 Single Bit Interrupt"]
    #[inline(always)]
    pub fn ram0sbe(&self) -> Ram0sbeR {
        Ram0sbeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RAM1 Multi Bit Interrupt"]
    #[inline(always)]
    pub fn ram1mbe(&self) -> Ram1mbeR {
        Ram1mbeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RAM1 Single Bit Interrupt"]
    #[inline(always)]
    pub fn ram1sbe(&self) -> Ram1sbeR {
        Ram1sbeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ROM Multi Bit Interrupt"]
    #[inline(always)]
    pub fn rommbe(&mut self) -> RommbeW<'_, IrqEnbSpec> {
        RommbeW::new(self, 0)
    }
    #[doc = "Bit 1 - ROM Single Bit Interrupt"]
    #[inline(always)]
    pub fn romsbe(&mut self) -> RomsbeW<'_, IrqEnbSpec> {
        RomsbeW::new(self, 1)
    }
    #[doc = "Bit 2 - RAM0 Multi Bit Interrupt"]
    #[inline(always)]
    pub fn ram0mbe(&mut self) -> Ram0mbeW<'_, IrqEnbSpec> {
        Ram0mbeW::new(self, 2)
    }
    #[doc = "Bit 3 - RAM0 Single Bit Interrupt"]
    #[inline(always)]
    pub fn ram0sbe(&mut self) -> Ram0sbeW<'_, IrqEnbSpec> {
        Ram0sbeW::new(self, 3)
    }
    #[doc = "Bit 4 - RAM1 Multi Bit Interrupt"]
    #[inline(always)]
    pub fn ram1mbe(&mut self) -> Ram1mbeW<'_, IrqEnbSpec> {
        Ram1mbeW::new(self, 4)
    }
    #[doc = "Bit 5 - RAM1 Single Bit Interrupt"]
    #[inline(always)]
    pub fn ram1sbe(&mut self) -> Ram1sbeW<'_, IrqEnbSpec> {
        Ram1sbeW::new(self, 5)
    }
}
#[doc = "Enable EDAC Error Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEnbSpec;
impl crate::RegisterSpec for IrqEnbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_enb::R`](R) reader structure"]
impl crate::Readable for IrqEnbSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_enb::W`](W) writer structure"]
impl crate::Writable for IrqEnbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_ENB to value 0"]
impl crate::Resettable for IrqEnbSpec {}
