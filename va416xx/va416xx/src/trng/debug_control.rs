#[doc = "Register `DEBUG_CONTROL` reader"]
pub type R = crate::R<DebugControlSpec>;
#[doc = "Register `DEBUG_CONTROL` writer"]
pub type W = crate::W<DebugControlSpec>;
#[doc = "Field `VNC_PYPASS` reader - The Von Neumann balancer is bypassed"]
pub type VncPypassR = crate::BitReader;
#[doc = "Field `VNC_PYPASS` writer - The Von Neumann balancer is bypassed"]
pub type VncPypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRNGT_BYPASS` reader - The CRNGT test in the TRNG is bypassed"]
pub type CrngtBypassR = crate::BitReader;
#[doc = "Field `CRNGT_BYPASS` writer - The CRNGT test in the TRNG is bypassed"]
pub type CrngtBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CORRELATE_BYPASS` reader - The autocorrelation test in the TRNG module is bypassed"]
pub type AutoCorrelateBypassR = crate::BitReader;
#[doc = "Field `AUTO_CORRELATE_BYPASS` writer - The autocorrelation test in the TRNG module is bypassed"]
pub type AutoCorrelateBypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - The Von Neumann balancer is bypassed"]
    #[inline(always)]
    pub fn vnc_pypass(&self) -> VncPypassR {
        VncPypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The CRNGT test in the TRNG is bypassed"]
    #[inline(always)]
    pub fn crngt_bypass(&self) -> CrngtBypassR {
        CrngtBypassR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The autocorrelation test in the TRNG module is bypassed"]
    #[inline(always)]
    pub fn auto_correlate_bypass(&self) -> AutoCorrelateBypassR {
        AutoCorrelateBypassR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - The Von Neumann balancer is bypassed"]
    #[inline(always)]
    pub fn vnc_pypass(&mut self) -> VncPypassW<'_, DebugControlSpec> {
        VncPypassW::new(self, 1)
    }
    #[doc = "Bit 2 - The CRNGT test in the TRNG is bypassed"]
    #[inline(always)]
    pub fn crngt_bypass(&mut self) -> CrngtBypassW<'_, DebugControlSpec> {
        CrngtBypassW::new(self, 2)
    }
    #[doc = "Bit 3 - The autocorrelation test in the TRNG module is bypassed"]
    #[inline(always)]
    pub fn auto_correlate_bypass(&mut self) -> AutoCorrelateBypassW<'_, DebugControlSpec> {
        AutoCorrelateBypassW::new(self, 3)
    }
}
#[doc = "Section TBD\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugControlSpec;
impl crate::RegisterSpec for DebugControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_control::R`](R) reader structure"]
impl crate::Readable for DebugControlSpec {}
#[doc = "`write(|w| ..)` method takes [`debug_control::W`](W) writer structure"]
impl crate::Writable for DebugControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG_CONTROL to value 0"]
impl crate::Resettable for DebugControlSpec {}
