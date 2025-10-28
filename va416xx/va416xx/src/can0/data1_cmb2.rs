#[doc = "Register `DATA1_CMB2` reader"]
pub type R = crate::R<Data1Cmb2Spec>;
#[doc = "Register `DATA1_CMB2` writer"]
pub type W = crate::W<Data1Cmb2Spec>;
#[doc = "Field `BYTE4` reader - Data Byte 4"]
pub type Byte4R = crate::FieldReader;
#[doc = "Field `BYTE4` writer - Data Byte 4"]
pub type Byte4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE3` reader - Data Byte 3"]
pub type Byte3R = crate::FieldReader;
#[doc = "Field `BYTE3` writer - Data Byte 3"]
pub type Byte3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data Byte 4"]
    #[inline(always)]
    pub fn byte4(&self) -> Byte4R {
        Byte4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 3"]
    #[inline(always)]
    pub fn byte3(&self) -> Byte3R {
        Byte3R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Byte 4"]
    #[inline(always)]
    pub fn byte4(&mut self) -> Byte4W<'_, Data1Cmb2Spec> {
        Byte4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data Byte 3"]
    #[inline(always)]
    pub fn byte3(&mut self) -> Byte3W<'_, Data1Cmb2Spec> {
        Byte3W::new(self, 8)
    }
}
#[doc = "CAN Frame Data Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`data1_cmb2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1_cmb2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data1Cmb2Spec;
impl crate::RegisterSpec for Data1Cmb2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data1_cmb2::R`](R) reader structure"]
impl crate::Readable for Data1Cmb2Spec {}
#[doc = "`write(|w| ..)` method takes [`data1_cmb2::W`](W) writer structure"]
impl crate::Writable for Data1Cmb2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA1_CMB2 to value 0"]
impl crate::Resettable for Data1Cmb2Spec {}
