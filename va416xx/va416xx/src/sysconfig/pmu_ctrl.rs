#[doc = "Register `PMU_CTRL` reader"]
pub type R = crate::R<PmuCtrlSpec>;
#[doc = "Register `PMU_CTRL` writer"]
pub type W = crate::W<PmuCtrlSpec>;
#[doc = "Field `LVL_SLCT` reader - Select the POK detect level"]
pub type LvlSlctR = crate::FieldReader;
#[doc = "Field `LVL_SLCT` writer - Select the POK detect level"]
pub type LvlSlctW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Select the POK detect level"]
    #[inline(always)]
    pub fn lvl_slct(&self) -> LvlSlctR {
        LvlSlctR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select the POK detect level"]
    #[inline(always)]
    pub fn lvl_slct(&mut self) -> LvlSlctW<'_, PmuCtrlSpec> {
        LvlSlctW::new(self, 0)
    }
}
#[doc = "PMU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmu_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmu_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuCtrlSpec;
impl crate::RegisterSpec for PmuCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmu_ctrl::R`](R) reader structure"]
impl crate::Readable for PmuCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pmu_ctrl::W`](W) writer structure"]
impl crate::Writable for PmuCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMU_CTRL to value 0"]
impl crate::Resettable for PmuCtrlSpec {}
