#[doc = "Register `DATAINBYTE[%s]` reader"]
pub type R = crate::R<DatainbyteSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Data In Register by Byte\n\nYou can [`read`](crate::Reg::read) this register and get [`datainbyte::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatainbyteSpec;
impl crate::RegisterSpec for DatainbyteSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`datainbyte::R`](R) reader structure"]
impl crate::Readable for DatainbyteSpec {}
#[doc = "`reset()` method sets DATAINBYTE[%s] to value 0"]
impl crate::Resettable for DatainbyteSpec {}
