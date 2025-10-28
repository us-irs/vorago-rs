#[doc = "Register `DMARXDESC0` reader"]
pub type R = crate::R<Dmarxdesc0Spec>;
#[doc = "Register `DMARXDESC0` writer"]
pub type W = crate::W<Dmarxdesc0Spec>;
#[doc = "Field `DESCSEL` reader - Offset into the descriptor table"]
pub type DescselR = crate::FieldReader;
#[doc = "Field `DESCSEL` writer - Offset into the descriptor table"]
pub type DescselW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DESCBASEADDR` reader - Sets the base address of the descriptor table"]
pub type DescbaseaddrR = crate::FieldReader<u32>;
#[doc = "Field `DESCBASEADDR` writer - Sets the base address of the descriptor table"]
pub type DescbaseaddrW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 3:9 - Offset into the descriptor table"]
    #[inline(always)]
    pub fn descsel(&self) -> DescselR {
        DescselR::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bits 10:31 - Sets the base address of the descriptor table"]
    #[inline(always)]
    pub fn descbaseaddr(&self) -> DescbaseaddrR {
        DescbaseaddrR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 3:9 - Offset into the descriptor table"]
    #[inline(always)]
    pub fn descsel(&mut self) -> DescselW<'_, Dmarxdesc0Spec> {
        DescselW::new(self, 3)
    }
    #[doc = "Bits 10:31 - Sets the base address of the descriptor table"]
    #[inline(always)]
    pub fn descbaseaddr(&mut self) -> DescbaseaddrW<'_, Dmarxdesc0Spec> {
        DescbaseaddrW::new(self, 10)
    }
}
#[doc = "DMA Receiver Table Destination Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarxdesc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarxdesc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmarxdesc0Spec;
impl crate::RegisterSpec for Dmarxdesc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarxdesc0::R`](R) reader structure"]
impl crate::Readable for Dmarxdesc0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmarxdesc0::W`](W) writer structure"]
impl crate::Writable for Dmarxdesc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMARXDESC0 to value 0"]
impl crate::Resettable for Dmarxdesc0Spec {}
