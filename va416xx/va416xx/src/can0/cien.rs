#[doc = "Register `CIEN` reader"]
pub type R = crate::R<CienSpec>;
#[doc = "Register `CIEN` writer"]
pub type W = crate::W<CienSpec>;
#[doc = "Field `IEN` reader - Buffer Interrupt Enable\\[14:0\\]"]
pub type IenR = crate::FieldReader<u16>;
#[doc = "Field `IEN` writer - Buffer Interrupt Enable\\[14:0\\]"]
pub type IenW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `EIEN` reader - Error Interrupt Enable"]
pub type EienR = crate::BitReader;
#[doc = "Field `EIEN` writer - Error Interrupt Enable"]
pub type EienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - Buffer Interrupt Enable\\[14:0\\]"]
    #[inline(always)]
    pub fn ien(&self) -> IenR {
        IenR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn eien(&self) -> EienR {
        EienR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Buffer Interrupt Enable\\[14:0\\]"]
    #[inline(always)]
    pub fn ien(&mut self) -> IenW<'_, CienSpec> {
        IenW::new(self, 0)
    }
    #[doc = "Bit 15 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn eien(&mut self) -> EienW<'_, CienSpec> {
        EienW::new(self, 15)
    }
}
#[doc = "CAN Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CienSpec;
impl crate::RegisterSpec for CienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cien::R`](R) reader structure"]
impl crate::Readable for CienSpec {}
#[doc = "`write(|w| ..)` method takes [`cien::W`](W) writer structure"]
impl crate::Writable for CienSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIEN to value 0"]
impl crate::Resettable for CienSpec {}
