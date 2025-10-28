#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `LBM` reader - Loop Back"]
pub type LbmR = crate::BitReader;
#[doc = "Field `LBM` writer - Loop Back"]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS` reader - Master/Slave (0:Master, 1:Slave)"]
pub type MsR = crate::BitReader;
#[doc = "Field `MS` writer - Master/Slave (0:Master, 1:Slave)"]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOD` reader - Slave output Disable"]
pub type SodR = crate::BitReader;
#[doc = "Field `SOD` writer - Slave output Disable"]
pub type SodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS` reader - Slave Select"]
pub type SsR = crate::FieldReader;
#[doc = "Field `SS` writer - Slave Select"]
pub type SsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BLOCKMODE` reader - Block Mode Enable"]
pub type BlockmodeR = crate::BitReader;
#[doc = "Field `BLOCKMODE` writer - Block Mode Enable"]
pub type BlockmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMSTART` reader - Block Mode Start Status Enable"]
pub type BmstartR = crate::BitReader;
#[doc = "Field `BMSTART` writer - Block Mode Start Status Enable"]
pub type BmstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMSTALL` reader - Block Mode Stall Enable"]
pub type BmstallR = crate::BitReader;
#[doc = "Field `BMSTALL` writer - Block Mode Stall Enable"]
pub type BmstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDLYCAP` reader - Master Delayed Capture Enable"]
pub type MdlycapR = crate::BitReader;
#[doc = "Field `MDLYCAP` writer - Master Delayed Capture Enable"]
pub type MdlycapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTXPAUSE` reader - Master Tx Pause Enable"]
pub type MtxpauseR = crate::BitReader;
#[doc = "Field `MTXPAUSE` writer - Master Tx Pause Enable"]
pub type MtxpauseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Loop Back"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master/Slave (0:Master, 1:Slave)"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave output Disable"]
    #[inline(always)]
    pub fn sod(&self) -> SodR {
        SodR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Slave Select"]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Block Mode Enable"]
    #[inline(always)]
    pub fn blockmode(&self) -> BlockmodeR {
        BlockmodeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Block Mode Start Status Enable"]
    #[inline(always)]
    pub fn bmstart(&self) -> BmstartR {
        BmstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Block Mode Stall Enable"]
    #[inline(always)]
    pub fn bmstall(&self) -> BmstallR {
        BmstallR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Master Delayed Capture Enable"]
    #[inline(always)]
    pub fn mdlycap(&self) -> MdlycapR {
        MdlycapR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Tx Pause Enable"]
    #[inline(always)]
    pub fn mtxpause(&self) -> MtxpauseR {
        MtxpauseR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Loop Back"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LbmW<'_, Ctrl1Spec> {
        LbmW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, Ctrl1Spec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Master/Slave (0:Master, 1:Slave)"]
    #[inline(always)]
    pub fn ms(&mut self) -> MsW<'_, Ctrl1Spec> {
        MsW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave output Disable"]
    #[inline(always)]
    pub fn sod(&mut self) -> SodW<'_, Ctrl1Spec> {
        SodW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Slave Select"]
    #[inline(always)]
    pub fn ss(&mut self) -> SsW<'_, Ctrl1Spec> {
        SsW::new(self, 4)
    }
    #[doc = "Bit 7 - Block Mode Enable"]
    #[inline(always)]
    pub fn blockmode(&mut self) -> BlockmodeW<'_, Ctrl1Spec> {
        BlockmodeW::new(self, 7)
    }
    #[doc = "Bit 8 - Block Mode Start Status Enable"]
    #[inline(always)]
    pub fn bmstart(&mut self) -> BmstartW<'_, Ctrl1Spec> {
        BmstartW::new(self, 8)
    }
    #[doc = "Bit 9 - Block Mode Stall Enable"]
    #[inline(always)]
    pub fn bmstall(&mut self) -> BmstallW<'_, Ctrl1Spec> {
        BmstallW::new(self, 9)
    }
    #[doc = "Bit 10 - Master Delayed Capture Enable"]
    #[inline(always)]
    pub fn mdlycap(&mut self) -> MdlycapW<'_, Ctrl1Spec> {
        MdlycapW::new(self, 10)
    }
    #[doc = "Bit 11 - Master Tx Pause Enable"]
    #[inline(always)]
    pub fn mtxpause(&mut self) -> MtxpauseW<'_, Ctrl1Spec> {
        MtxpauseW::new(self, 11)
    }
}
#[doc = "Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {}
