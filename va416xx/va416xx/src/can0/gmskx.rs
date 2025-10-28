#[doc = "Register `GMSKX` reader"]
pub type R = crate::R<GmskxSpec>;
#[doc = "Register `GMSKX` writer"]
pub type W = crate::W<GmskxSpec>;
#[doc = "Field `XRTR` reader - Extended Remote transmission Request Bit"]
pub type XrtrR = crate::BitReader;
#[doc = "Field `XRTR` writer - Extended Remote transmission Request Bit"]
pub type XrtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GM` reader - GM\\[14:0\\] used when an extended frame is received. ID\\[14:0\\] in extended, unused standard"]
pub type GmR = crate::FieldReader<u16>;
#[doc = "Field `GM` writer - GM\\[14:0\\] used when an extended frame is received. ID\\[14:0\\] in extended, unused standard"]
pub type GmW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - Extended Remote transmission Request Bit"]
    #[inline(always)]
    pub fn xrtr(&self) -> XrtrR {
        XrtrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15 - GM\\[14:0\\] used when an extended frame is received. ID\\[14:0\\] in extended, unused standard"]
    #[inline(always)]
    pub fn gm(&self) -> GmR {
        GmR::new(((self.bits >> 1) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Extended Remote transmission Request Bit"]
    #[inline(always)]
    pub fn xrtr(&mut self) -> XrtrW<'_, GmskxSpec> {
        XrtrW::new(self, 0)
    }
    #[doc = "Bits 1:15 - GM\\[14:0\\] used when an extended frame is received. ID\\[14:0\\] in extended, unused standard"]
    #[inline(always)]
    pub fn gm(&mut self) -> GmW<'_, GmskxSpec> {
        GmW::new(self, 1)
    }
}
#[doc = "CAN Global Mask Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`gmskx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmskx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmskxSpec;
impl crate::RegisterSpec for GmskxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmskx::R`](R) reader structure"]
impl crate::Readable for GmskxSpec {}
#[doc = "`write(|w| ..)` method takes [`gmskx::W`](W) writer structure"]
impl crate::Writable for GmskxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GMSKX to value 0"]
impl crate::Resettable for GmskxSpec {}
