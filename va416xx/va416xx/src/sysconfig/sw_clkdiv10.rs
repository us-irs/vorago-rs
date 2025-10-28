#[doc = "Register `SW_CLKDIV10` reader"]
pub type R = crate::R<SwClkdiv10Spec>;
#[doc = "Register `SW_CLKDIV10` writer"]
pub type W = crate::W<SwClkdiv10Spec>;
#[doc = "Field `SW_CLKDIV10` reader - Defines the initial value for the SpW clock, defaults to divide by ten"]
pub type SwClkdiv10R = crate::FieldReader;
#[doc = "Field `SW_CLKDIV10` writer - Defines the initial value for the SpW clock, defaults to divide by ten"]
pub type SwClkdiv10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Defines the initial value for the SpW clock, defaults to divide by ten"]
    #[inline(always)]
    pub fn sw_clkdiv10(&self) -> SwClkdiv10R {
        SwClkdiv10R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the initial value for the SpW clock, defaults to divide by ten"]
    #[inline(always)]
    pub fn sw_clkdiv10(&mut self) -> SwClkdiv10W<'_, SwClkdiv10Spec> {
        SwClkdiv10W::new(self, 0)
    }
}
#[doc = "Initial SpW Clock Divider Value\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_clkdiv10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_clkdiv10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwClkdiv10Spec;
impl crate::RegisterSpec for SwClkdiv10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_clkdiv10::R`](R) reader structure"]
impl crate::Readable for SwClkdiv10Spec {}
#[doc = "`write(|w| ..)` method takes [`sw_clkdiv10::W`](W) writer structure"]
impl crate::Writable for SwClkdiv10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SW_CLKDIV10 to value 0x09"]
impl crate::Resettable for SwClkdiv10Spec {
    const RESET_VALUE: u32 = 0x09;
}
