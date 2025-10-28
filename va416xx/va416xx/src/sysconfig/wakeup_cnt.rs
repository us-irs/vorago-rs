#[doc = "Register `WAKEUP_CNT` reader"]
pub type R = crate::R<WakeupCntSpec>;
#[doc = "Register `WAKEUP_CNT` writer"]
pub type W = crate::W<WakeupCntSpec>;
#[doc = "Field `WKUP_CNT` reader - Used to set a time to wake up the processor after the device has been put in a low power state"]
pub type WkupCntR = crate::FieldReader;
#[doc = "Field `WKUP_CNT` writer - Used to set a time to wake up the processor after the device has been put in a low power state"]
pub type WkupCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CNTSTRT` reader - Launch SLP mode in analog block"]
pub type CntstrtR = crate::BitReader;
#[doc = "Field `CNTSTRT` writer - Launch SLP mode in analog block"]
pub type CntstrtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Used to set a time to wake up the processor after the device has been put in a low power state"]
    #[inline(always)]
    pub fn wkup_cnt(&self) -> WkupCntR {
        WkupCntR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Launch SLP mode in analog block"]
    #[inline(always)]
    pub fn cntstrt(&self) -> CntstrtR {
        CntstrtR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to set a time to wake up the processor after the device has been put in a low power state"]
    #[inline(always)]
    pub fn wkup_cnt(&mut self) -> WkupCntW<'_, WakeupCntSpec> {
        WkupCntW::new(self, 0)
    }
    #[doc = "Bit 3 - Launch SLP mode in analog block"]
    #[inline(always)]
    pub fn cntstrt(&mut self) -> CntstrtW<'_, WakeupCntSpec> {
        CntstrtW::new(self, 3)
    }
}
#[doc = "Wakeup Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeupCntSpec;
impl crate::RegisterSpec for WakeupCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup_cnt::R`](R) reader structure"]
impl crate::Readable for WakeupCntSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeup_cnt::W`](W) writer structure"]
impl crate::Writable for WakeupCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAKEUP_CNT to value 0x07"]
impl crate::Resettable for WakeupCntSpec {
    const RESET_VALUE: u32 = 0x07;
}
