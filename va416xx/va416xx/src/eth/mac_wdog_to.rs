#[doc = "Register `MAC_WDOG_TO` reader"]
pub type R = crate::R<MacWdogToSpec>;
#[doc = "Register `MAC_WDOG_TO` writer"]
pub type W = crate::W<MacWdogToSpec>;
#[doc = "Field `WTO` reader - Watchdog Timeout"]
pub type WtoR = crate::FieldReader<u16>;
#[doc = "Field `WTO` writer - Watchdog Timeout"]
pub type WtoW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PWE` reader - Programmable Watchdog Enable"]
pub type PweR = crate::BitReader;
#[doc = "Field `PWE` writer - Programmable Watchdog Enable"]
pub type PweW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Watchdog Timeout"]
    #[inline(always)]
    pub fn wto(&self) -> WtoR {
        WtoR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn pwe(&self) -> PweR {
        PweR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Watchdog Timeout"]
    #[inline(always)]
    pub fn wto(&mut self) -> WtoW<'_, MacWdogToSpec> {
        WtoW::new(self, 0)
    }
    #[doc = "Bit 16 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn pwe(&mut self) -> PweW<'_, MacWdogToSpec> {
        PweW::new(self, 16)
    }
}
#[doc = "Controls the watchdog time-out for received frames\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_wdog_to::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_wdog_to::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacWdogToSpec;
impl crate::RegisterSpec for MacWdogToSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_wdog_to::R`](R) reader structure"]
impl crate::Readable for MacWdogToSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_wdog_to::W`](W) writer structure"]
impl crate::Writable for MacWdogToSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_WDOG_TO to value 0"]
impl crate::Resettable for MacWdogToSpec {}
