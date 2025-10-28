#[doc = "Register `DATA3_CMB1` reader"]
pub type R = crate::R<Data3Cmb1Spec>;
#[doc = "Register `DATA3_CMB1` writer"]
pub type W = crate::W<Data3Cmb1Spec>;
#[doc = "Field `BYTE8` reader - Data Byte 8"]
pub type Byte8R = crate::FieldReader;
#[doc = "Field `BYTE8` writer - Data Byte 8"]
pub type Byte8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE7` reader - Data Byte 7"]
pub type Byte7R = crate::FieldReader;
#[doc = "Field `BYTE7` writer - Data Byte 7"]
pub type Byte7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Byte 8"]
    #[inline(always)]
    pub fn byte8(&self) -> Byte8R {
        Byte8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 7"]
    #[inline(always)]
    pub fn byte7(&self) -> Byte7R {
        Byte7R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 8"]
    #[inline(always)]
    pub fn byte8(&mut self) -> Byte8W<'_, Data3Cmb1Spec> {
        Byte8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data Byte 7"]
    #[inline(always)]
    pub fn byte7(&mut self) -> Byte7W<'_, Data3Cmb1Spec> {
        Byte7W::new(self, 8)
    }
}
#[doc = "CAN Frame Data Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`data3_cmb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3_cmb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data3Cmb1Spec;
impl crate::RegisterSpec for Data3Cmb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data3_cmb1::R`](R) reader structure"]
impl crate::Readable for Data3Cmb1Spec {}
#[doc = "`write(|w| ..)` method takes [`data3_cmb1::W`](W) writer structure"]
impl crate::Writable for Data3Cmb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA3_CMB1 to value 0"]
impl crate::Resettable for Data3Cmb1Spec {}
