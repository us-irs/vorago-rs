#[doc = "Register `BMSKB` reader"]
pub type R = crate::R<BmskbSpec>;
#[doc = "Register `BMSKB` writer"]
pub type W = crate::W<BmskbSpec>;
#[doc = "Field `BM0` reader - BM\\[17:15\\] - Unused in standard, ID\\[17:15\\] in extended"]
pub type Bm0R = crate::FieldReader;
#[doc = "Field `BM0` writer - BM\\[17:15\\] - Unused in standard, ID\\[17:15\\] in extended"]
pub type Bm0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IDE` reader - Identifier Extension Bit"]
pub type IdeR = crate::BitReader;
#[doc = "Field `IDE` writer - Identifier Extension Bit"]
pub type IdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTR` reader - Remote Transmission Request in Standard, Substitute Remote Request (SRR) in extended"]
pub type RtrR = crate::BitReader;
#[doc = "Field `RTR` writer - Remote Transmission Request in Standard, Substitute Remote Request (SRR) in extended"]
pub type RtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BM1` reader - BM\\[28:18\\] - ID\\[10:0\\] in standard, ID\\[28:18\\] in extended"]
pub type Bm1R = crate::FieldReader<u16>;
#[doc = "Field `BM1` writer - BM\\[28:18\\] - ID\\[10:0\\] in standard, ID\\[28:18\\] in extended"]
pub type Bm1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:2 - BM\\[17:15\\] - Unused in standard, ID\\[17:15\\] in extended"]
    #[inline(always)]
    pub fn bm0(&self) -> Bm0R {
        Bm0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Identifier Extension Bit"]
    #[inline(always)]
    pub fn ide(&self) -> IdeR {
        IdeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Remote Transmission Request in Standard, Substitute Remote Request (SRR) in extended"]
    #[inline(always)]
    pub fn rtr(&self) -> RtrR {
        RtrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:15 - BM\\[28:18\\] - ID\\[10:0\\] in standard, ID\\[28:18\\] in extended"]
    #[inline(always)]
    pub fn bm1(&self) -> Bm1R {
        Bm1R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - BM\\[17:15\\] - Unused in standard, ID\\[17:15\\] in extended"]
    #[inline(always)]
    pub fn bm0(&mut self) -> Bm0W<'_, BmskbSpec> {
        Bm0W::new(self, 0)
    }
    #[doc = "Bit 3 - Identifier Extension Bit"]
    #[inline(always)]
    pub fn ide(&mut self) -> IdeW<'_, BmskbSpec> {
        IdeW::new(self, 3)
    }
    #[doc = "Bit 4 - Remote Transmission Request in Standard, Substitute Remote Request (SRR) in extended"]
    #[inline(always)]
    pub fn rtr(&mut self) -> RtrW<'_, BmskbSpec> {
        RtrW::new(self, 4)
    }
    #[doc = "Bits 5:15 - BM\\[28:18\\] - ID\\[10:0\\] in standard, ID\\[28:18\\] in extended"]
    #[inline(always)]
    pub fn bm1(&mut self) -> Bm1W<'_, BmskbSpec> {
        Bm1W::new(self, 5)
    }
}
#[doc = "CAN Basic Mask Base\n\nYou can [`read`](crate::Reg::read) this register and get [`bmskb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmskb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmskbSpec;
impl crate::RegisterSpec for BmskbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmskb::R`](R) reader structure"]
impl crate::Readable for BmskbSpec {}
#[doc = "`write(|w| ..)` method takes [`bmskb::W`](W) writer structure"]
impl crate::Writable for BmskbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMSKB to value 0"]
impl crate::Resettable for BmskbSpec {}
