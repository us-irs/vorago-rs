#[doc = "Register `TSTP_CMB9` reader"]
pub type R = crate::R<TstpCmb9Spec>;
#[doc = "Register `TSTP_CMB9` writer"]
pub type W = crate::W<TstpCmb9Spec>;
#[doc = "Field `TIMESTAMP` reader - Timestamp"]
pub type TimestampR = crate::FieldReader<u16>;
#[doc = "Field `TIMESTAMP` writer - Timestamp"]
pub type TimestampW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp"]
    #[inline(always)]
    pub fn timestamp(&self) -> TimestampR {
        TimestampR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp"]
    #[inline(always)]
    pub fn timestamp(&mut self) -> TimestampW<'_, TstpCmb9Spec> {
        TimestampW::new(self, 0)
    }
}
#[doc = "CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TstpCmb9Spec;
impl crate::RegisterSpec for TstpCmb9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tstp_cmb9::R`](R) reader structure"]
impl crate::Readable for TstpCmb9Spec {}
#[doc = "`write(|w| ..)` method takes [`tstp_cmb9::W`](W) writer structure"]
impl crate::Writable for TstpCmb9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSTP_CMB9 to value 0"]
impl crate::Resettable for TstpCmb9Spec {}
