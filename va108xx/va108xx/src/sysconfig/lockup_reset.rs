#[doc = "Register `LOCKUP_RESET` reader"]
pub type R = crate::R<LockupResetSpec>;
#[doc = "Register `LOCKUP_RESET` writer"]
pub type W = crate::W<LockupResetSpec>;
#[doc = "Field `LREN` reader - Lockup Reset Enable Bit"]
pub type LrenR = crate::BitReader;
#[doc = "Field `LREN` writer - Lockup Reset Enable Bit"]
pub type LrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lockup Reset Enable Bit"]
    #[inline(always)]
    pub fn lren(&self) -> LrenR {
        LrenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lockup Reset Enable Bit"]
    #[inline(always)]
    pub fn lren(&mut self) -> LrenW<'_, LockupResetSpec> {
        LrenW::new(self, 0)
    }
}
#[doc = "Lockup Reset Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`lockup_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockup_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockupResetSpec;
impl crate::RegisterSpec for LockupResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockup_reset::R`](R) reader structure"]
impl crate::Readable for LockupResetSpec {}
#[doc = "`write(|w| ..)` method takes [`lockup_reset::W`](W) writer structure"]
impl crate::Writable for LockupResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCKUP_RESET to value 0x01"]
impl crate::Resettable for LockupResetSpec {
    const RESET_VALUE: u32 = 0x01;
}
