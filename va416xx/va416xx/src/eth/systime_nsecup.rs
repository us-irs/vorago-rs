#[doc = "Register `SYSTIME_NSECUP` reader"]
pub type R = crate::R<SystimeNsecupSpec>;
#[doc = "Register `SYSTIME_NSECUP` writer"]
pub type W = crate::W<SystimeNsecupSpec>;
#[doc = "Field `TSSS` reader - Timestamp Sub Seconds"]
pub type TsssR = crate::FieldReader<u32>;
#[doc = "Field `TSSS` writer - Timestamp Sub Seconds"]
pub type TsssW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `ADDSUB` reader - Add or Subtract Time"]
pub type AddsubR = crate::BitReader;
#[doc = "Field `ADDSUB` writer - Add or Subtract Time"]
pub type AddsubW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub Seconds"]
    #[inline(always)]
    pub fn tsss(&self) -> TsssR {
        TsssR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Add or Subtract Time"]
    #[inline(always)]
    pub fn addsub(&self) -> AddsubR {
        AddsubR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Timestamp Sub Seconds"]
    #[inline(always)]
    pub fn tsss(&mut self) -> TsssW<'_, SystimeNsecupSpec> {
        TsssW::new(self, 0)
    }
    #[doc = "Bit 31 - Add or Subtract Time"]
    #[inline(always)]
    pub fn addsub(&mut self) -> AddsubW<'_, SystimeNsecupSpec> {
        AddsubW::new(self, 31)
    }
}
#[doc = "Holds 32 bits of the nano-second field to be written to, added to, or subtracted from the system time value\n\nYou can [`read`](crate::Reg::read) this register and get [`systime_nsecup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systime_nsecup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystimeNsecupSpec;
impl crate::RegisterSpec for SystimeNsecupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systime_nsecup::R`](R) reader structure"]
impl crate::Readable for SystimeNsecupSpec {}
#[doc = "`write(|w| ..)` method takes [`systime_nsecup::W`](W) writer structure"]
impl crate::Writable for SystimeNsecupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSTIME_NSECUP to value 0"]
impl crate::Resettable for SystimeNsecupSpec {}
