#[doc = "Register `CIPND` reader"]
pub type R = crate::R<CipndSpec>;
#[doc = "Register `CIPND` writer"]
pub type W = crate::W<CipndSpec>;
#[doc = "Field `IPND` reader - Buffer Interrupt Pending\\[14:0\\]"]
pub type IpndR = crate::FieldReader<u16>;
#[doc = "Field `IPND` writer - Buffer Interrupt Pending\\[14:0\\]"]
pub type IpndW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `EIPND` reader - Error Interrupt Pending"]
pub type EipndR = crate::BitReader;
#[doc = "Field `EIPND` writer - Error Interrupt Pending"]
pub type EipndW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - Buffer Interrupt Pending\\[14:0\\]"]
    #[inline(always)]
    pub fn ipnd(&self) -> IpndR {
        IpndR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Error Interrupt Pending"]
    #[inline(always)]
    pub fn eipnd(&self) -> EipndR {
        EipndR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Buffer Interrupt Pending\\[14:0\\]"]
    #[inline(always)]
    pub fn ipnd(&mut self) -> IpndW<'_, CipndSpec> {
        IpndW::new(self, 0)
    }
    #[doc = "Bit 15 - Error Interrupt Pending"]
    #[inline(always)]
    pub fn eipnd(&mut self) -> EipndW<'_, CipndSpec> {
        EipndW::new(self, 15)
    }
}
#[doc = "CAN Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cipnd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cipnd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CipndSpec;
impl crate::RegisterSpec for CipndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cipnd::R`](R) reader structure"]
impl crate::Readable for CipndSpec {}
#[doc = "`write(|w| ..)` method takes [`cipnd::W`](W) writer structure"]
impl crate::Writable for CipndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIPND to value 0"]
impl crate::Resettable for CipndSpec {}
