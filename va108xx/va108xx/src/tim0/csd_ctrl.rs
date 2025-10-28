#[doc = "Register `CSD_CTRL` reader"]
pub type R = crate::R<CsdCtrlSpec>;
#[doc = "Register `CSD_CTRL` writer"]
pub type W = crate::W<CsdCtrlSpec>;
#[doc = "Field `CSDEN0` reader - Cascade 0 Enable"]
pub type Csden0R = crate::BitReader;
#[doc = "Field `CSDEN0` writer - Cascade 0 Enable"]
pub type Csden0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDINV0` reader - Cascade 0 Invert"]
pub type Csdinv0R = crate::BitReader;
#[doc = "Field `CSDINV0` writer - Cascade 0 Invert"]
pub type Csdinv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDEN1` reader - Cascade 1 Enable"]
pub type Csden1R = crate::BitReader;
#[doc = "Field `CSDEN1` writer - Cascade 1 Enable"]
pub type Csden1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDINV1` reader - Cascade 1 Invert"]
pub type Csdinv1R = crate::BitReader;
#[doc = "Field `CSDINV1` writer - Cascade 1 Invert"]
pub type Csdinv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCASOP` reader - Dual Cascade Operation (0:AND, 1:OR)"]
pub type DcasopR = crate::BitReader;
#[doc = "Field `DCASOP` writer - Dual Cascade Operation (0:AND, 1:OR)"]
pub type DcasopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDTRG0` reader - Cascade 0 Enabled as Trigger"]
pub type Csdtrg0R = crate::BitReader;
#[doc = "Field `CSDTRG0` writer - Cascade 0 Enabled as Trigger"]
pub type Csdtrg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDTRG1` reader - Cascade 1 Enabled as Trigger"]
pub type Csdtrg1R = crate::BitReader;
#[doc = "Field `CSDTRG1` writer - Cascade 1 Enabled as Trigger"]
pub type Csdtrg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDEN2` reader - Cascade 2 Enable"]
pub type Csden2R = crate::BitReader;
#[doc = "Field `CSDEN2` writer - Cascade 2 Enable"]
pub type Csden2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDINV2` reader - Cascade 2 Invert"]
pub type Csdinv2R = crate::BitReader;
#[doc = "Field `CSDINV2` writer - Cascade 2 Invert"]
pub type Csdinv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDTRG2` reader - Cascade 2 Enabled as Trigger"]
pub type Csdtrg2R = crate::BitReader;
#[doc = "Field `CSDTRG2` writer - Cascade 2 Enabled as Trigger"]
pub type Csdtrg2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDXXX2` reader - Cascade 2 test mode"]
pub type Csdxxx2R = crate::BitReader;
#[doc = "Field `CSDXXX2` writer - Cascade 2 test mode"]
pub type Csdxxx2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cascade 0 Enable"]
    #[inline(always)]
    pub fn csden0(&self) -> Csden0R {
        Csden0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cascade 0 Invert"]
    #[inline(always)]
    pub fn csdinv0(&self) -> Csdinv0R {
        Csdinv0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cascade 1 Enable"]
    #[inline(always)]
    pub fn csden1(&self) -> Csden1R {
        Csden1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cascade 1 Invert"]
    #[inline(always)]
    pub fn csdinv1(&self) -> Csdinv1R {
        Csdinv1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Dual Cascade Operation (0:AND, 1:OR)"]
    #[inline(always)]
    pub fn dcasop(&self) -> DcasopR {
        DcasopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Cascade 0 Enabled as Trigger"]
    #[inline(always)]
    pub fn csdtrg0(&self) -> Csdtrg0R {
        Csdtrg0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Cascade 1 Enabled as Trigger"]
    #[inline(always)]
    pub fn csdtrg1(&self) -> Csdtrg1R {
        Csdtrg1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Cascade 2 Enable"]
    #[inline(always)]
    pub fn csden2(&self) -> Csden2R {
        Csden2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Cascade 2 Invert"]
    #[inline(always)]
    pub fn csdinv2(&self) -> Csdinv2R {
        Csdinv2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Cascade 2 Enabled as Trigger"]
    #[inline(always)]
    pub fn csdtrg2(&self) -> Csdtrg2R {
        Csdtrg2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Cascade 2 test mode"]
    #[inline(always)]
    pub fn csdxxx2(&self) -> Csdxxx2R {
        Csdxxx2R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cascade 0 Enable"]
    #[inline(always)]
    pub fn csden0(&mut self) -> Csden0W<'_, CsdCtrlSpec> {
        Csden0W::new(self, 0)
    }
    #[doc = "Bit 1 - Cascade 0 Invert"]
    #[inline(always)]
    pub fn csdinv0(&mut self) -> Csdinv0W<'_, CsdCtrlSpec> {
        Csdinv0W::new(self, 1)
    }
    #[doc = "Bit 2 - Cascade 1 Enable"]
    #[inline(always)]
    pub fn csden1(&mut self) -> Csden1W<'_, CsdCtrlSpec> {
        Csden1W::new(self, 2)
    }
    #[doc = "Bit 3 - Cascade 1 Invert"]
    #[inline(always)]
    pub fn csdinv1(&mut self) -> Csdinv1W<'_, CsdCtrlSpec> {
        Csdinv1W::new(self, 3)
    }
    #[doc = "Bit 4 - Dual Cascade Operation (0:AND, 1:OR)"]
    #[inline(always)]
    pub fn dcasop(&mut self) -> DcasopW<'_, CsdCtrlSpec> {
        DcasopW::new(self, 4)
    }
    #[doc = "Bit 6 - Cascade 0 Enabled as Trigger"]
    #[inline(always)]
    pub fn csdtrg0(&mut self) -> Csdtrg0W<'_, CsdCtrlSpec> {
        Csdtrg0W::new(self, 6)
    }
    #[doc = "Bit 7 - Cascade 1 Enabled as Trigger"]
    #[inline(always)]
    pub fn csdtrg1(&mut self) -> Csdtrg1W<'_, CsdCtrlSpec> {
        Csdtrg1W::new(self, 7)
    }
    #[doc = "Bit 8 - Cascade 2 Enable"]
    #[inline(always)]
    pub fn csden2(&mut self) -> Csden2W<'_, CsdCtrlSpec> {
        Csden2W::new(self, 8)
    }
    #[doc = "Bit 9 - Cascade 2 Invert"]
    #[inline(always)]
    pub fn csdinv2(&mut self) -> Csdinv2W<'_, CsdCtrlSpec> {
        Csdinv2W::new(self, 9)
    }
    #[doc = "Bit 10 - Cascade 2 Enabled as Trigger"]
    #[inline(always)]
    pub fn csdtrg2(&mut self) -> Csdtrg2W<'_, CsdCtrlSpec> {
        Csdtrg2W::new(self, 10)
    }
    #[doc = "Bit 11 - Cascade 2 test mode"]
    #[inline(always)]
    pub fn csdxxx2(&mut self) -> Csdxxx2W<'_, CsdCtrlSpec> {
        Csdxxx2W::new(self, 11)
    }
}
#[doc = "The Cascade Control Register. Controls the counter external enable signals\n\nYou can [`read`](crate::Reg::read) this register and get [`csd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsdCtrlSpec;
impl crate::RegisterSpec for CsdCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csd_ctrl::R`](R) reader structure"]
impl crate::Readable for CsdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`csd_ctrl::W`](W) writer structure"]
impl crate::Writable for CsdCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSD_CTRL to value 0"]
impl crate::Resettable for CsdCtrlSpec {}
