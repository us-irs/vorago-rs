#[doc = "Register `CICEN` reader"]
pub type R = crate::R<CicenSpec>;
#[doc = "Register `CICEN` writer"]
pub type W = crate::W<CicenSpec>;
#[doc = "Field `ICEN` reader - Buffer Interrupt Code Enable\\[14:0\\]"]
pub type IcenR = crate::FieldReader<u16>;
#[doc = "Field `ICEN` writer - Buffer Interrupt Code Enable\\[14:0\\]"]
pub type IcenW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `EICEN` reader - Error Interrupt Code Enable"]
pub type EicenR = crate::BitReader;
#[doc = "Field `EICEN` writer - Error Interrupt Code Enable"]
pub type EicenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - Buffer Interrupt Code Enable\\[14:0\\]"]
    #[inline(always)]
    pub fn icen(&self) -> IcenR {
        IcenR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Error Interrupt Code Enable"]
    #[inline(always)]
    pub fn eicen(&self) -> EicenR {
        EicenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Buffer Interrupt Code Enable\\[14:0\\]"]
    #[inline(always)]
    pub fn icen(&mut self) -> IcenW<'_, CicenSpec> {
        IcenW::new(self, 0)
    }
    #[doc = "Bit 15 - Error Interrupt Code Enable"]
    #[inline(always)]
    pub fn eicen(&mut self) -> EicenW<'_, CicenSpec> {
        EicenW::new(self, 15)
    }
}
#[doc = "CAN Interrupt Code Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cicen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CicenSpec;
impl crate::RegisterSpec for CicenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cicen::R`](R) reader structure"]
impl crate::Readable for CicenSpec {}
#[doc = "`write(|w| ..)` method takes [`cicen::W`](W) writer structure"]
impl crate::Writable for CicenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CICEN to value 0"]
impl crate::Resettable for CicenSpec {}
