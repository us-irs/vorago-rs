#[doc = "Register `BMSKX` reader"]
pub type R = crate::R<BmskxSpec>;
#[doc = "Register `BMSKX` writer"]
pub type W = crate::W<BmskxSpec>;
#[doc = "Field `XRTR` reader - Extended Remote transmission Request Bit"]
pub type XrtrR = crate::BitReader;
#[doc = "Field `XRTR` writer - Extended Remote transmission Request Bit"]
pub type XrtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BM` reader - BM\\[14:0\\] used when an extended frame is received. ID\\[14:0\\] in extended, unused standard"]
pub type BmR = crate::FieldReader<u16>;
#[doc = "Field `BM` writer - BM\\[14:0\\] used when an extended frame is received. ID\\[14:0\\] in extended, unused standard"]
pub type BmW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - Extended Remote transmission Request Bit"]
    #[inline(always)]
    pub fn xrtr(&self) -> XrtrR {
        XrtrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15 - BM\\[14:0\\] used when an extended frame is received. ID\\[14:0\\] in extended, unused standard"]
    #[inline(always)]
    pub fn bm(&self) -> BmR {
        BmR::new(((self.bits >> 1) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Extended Remote transmission Request Bit"]
    #[inline(always)]
    pub fn xrtr(&mut self) -> XrtrW<'_, BmskxSpec> {
        XrtrW::new(self, 0)
    }
    #[doc = "Bits 1:15 - BM\\[14:0\\] used when an extended frame is received. ID\\[14:0\\] in extended, unused standard"]
    #[inline(always)]
    pub fn bm(&mut self) -> BmW<'_, BmskxSpec> {
        BmW::new(self, 1)
    }
}
#[doc = "CAN Basic Mask Extension\n\nYou can [`read`](crate::Reg::read) this register and get [`bmskx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmskx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmskxSpec;
impl crate::RegisterSpec for BmskxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmskx::R`](R) reader structure"]
impl crate::Readable for BmskxSpec {}
#[doc = "`write(|w| ..)` method takes [`bmskx::W`](W) writer structure"]
impl crate::Writable for BmskxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMSKX to value 0"]
impl crate::Resettable for BmskxSpec {}
