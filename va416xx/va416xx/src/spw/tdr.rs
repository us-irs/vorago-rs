#[doc = "Register `TDR` reader"]
pub type R = crate::R<TdrSpec>;
#[doc = "Field `TIMER64` reader - Used to generate the 6.4 and 12.8 us time periods"]
pub type Timer64R = crate::FieldReader<u16>;
#[doc = "Field `DISCONNECT` reader - Used to generate the 850 ns disconnect time period"]
pub type DisconnectR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Used to generate the 6.4 and 12.8 us time periods"]
    #[inline(always)]
    pub fn timer64(&self) -> Timer64R {
        Timer64R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:21 - Used to generate the 850 ns disconnect time period"]
    #[inline(always)]
    pub fn disconnect(&self) -> DisconnectR {
        DisconnectR::new(((self.bits >> 12) & 0x03ff) as u16)
    }
}
#[doc = "Timer and Disconnect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdrSpec;
impl crate::RegisterSpec for TdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdr::R`](R) reader structure"]
impl crate::Readable for TdrSpec {}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TdrSpec {}
