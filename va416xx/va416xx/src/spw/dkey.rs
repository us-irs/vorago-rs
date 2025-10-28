#[doc = "Register `DKEY` reader"]
pub type R = crate::R<DkeySpec>;
#[doc = "Register `DKEY` writer"]
pub type W = crate::W<DkeySpec>;
#[doc = "Field `DESTKEY` reader - RMAP destination key"]
pub type DestkeyR = crate::FieldReader;
#[doc = "Field `DESTKEY` writer - RMAP destination key"]
pub type DestkeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RMAP destination key"]
    #[inline(always)]
    pub fn destkey(&self) -> DestkeyR {
        DestkeyR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RMAP destination key"]
    #[inline(always)]
    pub fn destkey(&mut self) -> DestkeyW<'_, DkeySpec> {
        DestkeyW::new(self, 0)
    }
}
#[doc = "Destination Key\n\nYou can [`read`](crate::Reg::read) this register and get [`dkey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dkey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DkeySpec;
impl crate::RegisterSpec for DkeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dkey::R`](R) reader structure"]
impl crate::Readable for DkeySpec {}
#[doc = "`write(|w| ..)` method takes [`dkey::W`](W) writer structure"]
impl crate::Writable for DkeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DKEY to value 0"]
impl crate::Resettable for DkeySpec {}
