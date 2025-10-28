#[doc = "Register `CGCR` reader"]
pub type R = crate::R<CgcrSpec>;
#[doc = "Register `CGCR` writer"]
pub type W = crate::W<CgcrSpec>;
#[doc = "Field `CANEN` reader - CAN Enable"]
pub type CanenR = crate::BitReader;
#[doc = "Field `CANEN` writer - CAN Enable"]
pub type CanenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRX` reader - RW,Control Receive"]
pub type CrxR = crate::BitReader;
#[doc = "Field `CRX` writer - RW,Control Receive"]
pub type CrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTX` reader - RW,Control Transmit"]
pub type CtxR = crate::BitReader;
#[doc = "Field `CTX` writer - RW,Control Transmit"]
pub type CtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFFLOCK` reader - Buffer Lock"]
pub type BufflockR = crate::BitReader;
#[doc = "Field `BUFFLOCK` writer - Buffer Lock"]
pub type BufflockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTPEN` reader - Time Sync Enable"]
pub type TstpenR = crate::BitReader;
#[doc = "Field `TSTPEN` writer - Time Sync Enable"]
pub type TstpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDIR` reader - Data Direction"]
pub type DdirR = crate::BitReader;
#[doc = "Field `DDIR` writer - Data Direction"]
pub type DdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LO` reader - Listen Only"]
pub type LoR = crate::BitReader;
#[doc = "Field `LO` writer - Listen Only"]
pub type LoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNACK` reader - Ignore Acknowledge"]
pub type IgnackR = crate::BitReader;
#[doc = "Field `IGNACK` writer - Ignore Acknowledge"]
pub type IgnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - Loopback"]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Loopback"]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERNAL` reader - Internal"]
pub type InternalR = crate::BitReader;
#[doc = "Field `INTERNAL` writer - Internal"]
pub type InternalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIAGEN` reader - Diagnostic Enable"]
pub type DiagenR = crate::BitReader;
#[doc = "Field `DIAGEN` writer - Diagnostic Enable"]
pub type DiagenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIT` reader - Error Interrupt Type"]
pub type EitR = crate::BitReader;
#[doc = "Field `EIT` writer - Error Interrupt Type"]
pub type EitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CAN Enable"]
    #[inline(always)]
    pub fn canen(&self) -> CanenR {
        CanenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RW,Control Receive"]
    #[inline(always)]
    pub fn crx(&self) -> CrxR {
        CrxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RW,Control Transmit"]
    #[inline(always)]
    pub fn ctx(&self) -> CtxR {
        CtxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Buffer Lock"]
    #[inline(always)]
    pub fn bufflock(&self) -> BufflockR {
        BufflockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Time Sync Enable"]
    #[inline(always)]
    pub fn tstpen(&self) -> TstpenR {
        TstpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Direction"]
    #[inline(always)]
    pub fn ddir(&self) -> DdirR {
        DdirR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Listen Only"]
    #[inline(always)]
    pub fn lo(&self) -> LoR {
        LoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    pub fn ignack(&self) -> IgnackR {
        IgnackR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Loopback"]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Internal"]
    #[inline(always)]
    pub fn internal(&self) -> InternalR {
        InternalR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Diagnostic Enable"]
    #[inline(always)]
    pub fn diagen(&self) -> DiagenR {
        DiagenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error Interrupt Type"]
    #[inline(always)]
    pub fn eit(&self) -> EitR {
        EitR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Enable"]
    #[inline(always)]
    pub fn canen(&mut self) -> CanenW<'_, CgcrSpec> {
        CanenW::new(self, 0)
    }
    #[doc = "Bit 1 - RW,Control Receive"]
    #[inline(always)]
    pub fn crx(&mut self) -> CrxW<'_, CgcrSpec> {
        CrxW::new(self, 1)
    }
    #[doc = "Bit 2 - RW,Control Transmit"]
    #[inline(always)]
    pub fn ctx(&mut self) -> CtxW<'_, CgcrSpec> {
        CtxW::new(self, 2)
    }
    #[doc = "Bit 3 - Buffer Lock"]
    #[inline(always)]
    pub fn bufflock(&mut self) -> BufflockW<'_, CgcrSpec> {
        BufflockW::new(self, 3)
    }
    #[doc = "Bit 4 - Time Sync Enable"]
    #[inline(always)]
    pub fn tstpen(&mut self) -> TstpenW<'_, CgcrSpec> {
        TstpenW::new(self, 4)
    }
    #[doc = "Bit 5 - Data Direction"]
    #[inline(always)]
    pub fn ddir(&mut self) -> DdirW<'_, CgcrSpec> {
        DdirW::new(self, 5)
    }
    #[doc = "Bit 6 - Listen Only"]
    #[inline(always)]
    pub fn lo(&mut self) -> LoW<'_, CgcrSpec> {
        LoW::new(self, 6)
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    pub fn ignack(&mut self) -> IgnackW<'_, CgcrSpec> {
        IgnackW::new(self, 7)
    }
    #[doc = "Bit 8 - Loopback"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LoopbackW<'_, CgcrSpec> {
        LoopbackW::new(self, 8)
    }
    #[doc = "Bit 9 - Internal"]
    #[inline(always)]
    pub fn internal(&mut self) -> InternalW<'_, CgcrSpec> {
        InternalW::new(self, 9)
    }
    #[doc = "Bit 10 - Diagnostic Enable"]
    #[inline(always)]
    pub fn diagen(&mut self) -> DiagenW<'_, CgcrSpec> {
        DiagenW::new(self, 10)
    }
    #[doc = "Bit 11 - Error Interrupt Type"]
    #[inline(always)]
    pub fn eit(&mut self) -> EitW<'_, CgcrSpec> {
        EitW::new(self, 11)
    }
}
#[doc = "CAN Global Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgcrSpec;
impl crate::RegisterSpec for CgcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgcr::R`](R) reader structure"]
impl crate::Readable for CgcrSpec {}
#[doc = "`write(|w| ..)` method takes [`cgcr::W`](W) writer structure"]
impl crate::Writable for CgcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CGCR to value 0"]
impl crate::Resettable for CgcrSpec {}
