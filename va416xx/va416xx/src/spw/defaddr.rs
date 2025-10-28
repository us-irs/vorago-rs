#[doc = "Register `DEFADDR` reader"]
pub type R = crate::R<DefaddrSpec>;
#[doc = "Register `DEFADDR` writer"]
pub type W = crate::W<DefaddrSpec>;
#[doc = "Field `DEFADDR` reader - 8-bit node address used for node identification on the SpaceWire network"]
pub type DefaddrR = crate::FieldReader;
#[doc = "Field `DEFADDR` writer - 8-bit node address used for node identification on the SpaceWire network"]
pub type DefaddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DEFMASK` reader - 8-bit default mask used for node identification on the SpaceWire network"]
pub type DefmaskR = crate::FieldReader;
#[doc = "Field `DEFMASK` writer - 8-bit default mask used for node identification on the SpaceWire network"]
pub type DefmaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit node address used for node identification on the SpaceWire network"]
    #[inline(always)]
    pub fn defaddr(&self) -> DefaddrR {
        DefaddrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit default mask used for node identification on the SpaceWire network"]
    #[inline(always)]
    pub fn defmask(&self) -> DefmaskR {
        DefmaskR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit node address used for node identification on the SpaceWire network"]
    #[inline(always)]
    pub fn defaddr(&mut self) -> DefaddrW<'_, DefaddrSpec> {
        DefaddrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit default mask used for node identification on the SpaceWire network"]
    #[inline(always)]
    pub fn defmask(&mut self) -> DefmaskW<'_, DefaddrSpec> {
        DefmaskW::new(self, 8)
    }
}
#[doc = "Node Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`defaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`defaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DefaddrSpec;
impl crate::RegisterSpec for DefaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`defaddr::R`](R) reader structure"]
impl crate::Readable for DefaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`defaddr::W`](W) writer structure"]
impl crate::Writable for DefaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEFADDR to value 0xfe"]
impl crate::Resettable for DefaddrSpec {
    const RESET_VALUE: u32 = 0xfe;
}
