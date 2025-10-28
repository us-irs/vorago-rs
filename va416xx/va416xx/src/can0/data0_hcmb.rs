#[doc = "Register `DATA0_HCMB` reader"]
pub type R = crate::R<Data0HcmbSpec>;
#[doc = "Register `DATA0_HCMB` writer"]
pub type W = crate::W<Data0HcmbSpec>;
#[doc = "Field `BYTE2` reader - Data Byte 2"]
pub type Byte2R = crate::FieldReader;
#[doc = "Field `BYTE2` writer - Data Byte 2"]
pub type Byte2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE1` reader - Data Byte 1"]
pub type Byte1R = crate::FieldReader;
#[doc = "Field `BYTE1` writer - Data Byte 1"]
pub type Byte1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Byte 2"]
    #[inline(always)]
    pub fn byte2(&self) -> Byte2R {
        Byte2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 1"]
    #[inline(always)]
    pub fn byte1(&self) -> Byte1R {
        Byte1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 2"]
    #[inline(always)]
    pub fn byte2(&mut self) -> Byte2W<'_, Data0HcmbSpec> {
        Byte2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data Byte 1"]
    #[inline(always)]
    pub fn byte1(&mut self) -> Byte1W<'_, Data0HcmbSpec> {
        Byte1W::new(self, 8)
    }
}
#[doc = "CAN Frame Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0_hcmb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0_hcmb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data0HcmbSpec;
impl crate::RegisterSpec for Data0HcmbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0_hcmb::R`](R) reader structure"]
impl crate::Readable for Data0HcmbSpec {}
#[doc = "`write(|w| ..)` method takes [`data0_hcmb::W`](W) writer structure"]
impl crate::Writable for Data0HcmbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA0_HCMB to value 0"]
impl crate::Resettable for Data0HcmbSpec {}
