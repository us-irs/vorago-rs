#[doc = "Register `GMSKB` reader"]
pub type R = crate::R<GmskbSpec>;
#[doc = "Register `GMSKB` writer"]
pub type W = crate::W<GmskbSpec>;
#[doc = "Field `GM0` reader - GM\\[17:15\\] - Unused in standard, ID\\[17:15\\] in extended"]
pub type Gm0R = crate::FieldReader;
#[doc = "Field `GM0` writer - GM\\[17:15\\] - Unused in standard, ID\\[17:15\\] in extended"]
pub type Gm0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IDE` reader - Identifier Extension Bit"]
pub type IdeR = crate::BitReader;
#[doc = "Field `IDE` writer - Identifier Extension Bit"]
pub type IdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTR` reader - Remote Transmission Request in Standard, Substitute Remote Request (SRR) in extended"]
pub type RtrR = crate::BitReader;
#[doc = "Field `RTR` writer - Remote Transmission Request in Standard, Substitute Remote Request (SRR) in extended"]
pub type RtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GM1` reader - GM\\[28:18\\] - ID\\[10:0\\] in standard, ID\\[28:18\\] in extended"]
pub type Gm1R = crate::FieldReader<u16>;
#[doc = "Field `GM1` writer - GM\\[28:18\\] - ID\\[10:0\\] in standard, ID\\[28:18\\] in extended"]
pub type Gm1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:2 - GM\\[17:15\\] - Unused in standard, ID\\[17:15\\] in extended"]
    #[inline(always)]
    pub fn gm0(&self) -> Gm0R {
        Gm0R::new((self.bits & 7) as u8)
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
    #[doc = "Bits 5:15 - GM\\[28:18\\] - ID\\[10:0\\] in standard, ID\\[28:18\\] in extended"]
    #[inline(always)]
    pub fn gm1(&self) -> Gm1R {
        Gm1R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - GM\\[17:15\\] - Unused in standard, ID\\[17:15\\] in extended"]
    #[inline(always)]
    pub fn gm0(&mut self) -> Gm0W<'_, GmskbSpec> {
        Gm0W::new(self, 0)
    }
    #[doc = "Bit 3 - Identifier Extension Bit"]
    #[inline(always)]
    pub fn ide(&mut self) -> IdeW<'_, GmskbSpec> {
        IdeW::new(self, 3)
    }
    #[doc = "Bit 4 - Remote Transmission Request in Standard, Substitute Remote Request (SRR) in extended"]
    #[inline(always)]
    pub fn rtr(&mut self) -> RtrW<'_, GmskbSpec> {
        RtrW::new(self, 4)
    }
    #[doc = "Bits 5:15 - GM\\[28:18\\] - ID\\[10:0\\] in standard, ID\\[28:18\\] in extended"]
    #[inline(always)]
    pub fn gm1(&mut self) -> Gm1W<'_, GmskbSpec> {
        Gm1W::new(self, 5)
    }
}
#[doc = "CAN Global Mask Base\n\nYou can [`read`](crate::Reg::read) this register and get [`gmskb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmskb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmskbSpec;
impl crate::RegisterSpec for GmskbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmskb::R`](R) reader structure"]
impl crate::Readable for GmskbSpec {}
#[doc = "`write(|w| ..)` method takes [`gmskb::W`](W) writer structure"]
impl crate::Writable for GmskbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GMSKB to value 0"]
impl crate::Resettable for GmskbSpec {}
