#[doc = "Register `RXFIFOOVERFLOW` reader"]
pub type R = crate::R<RxfifooverflowSpec>;
#[doc = "Field `COUNT` reader - Number of frames"]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "MMC Number of missed received frames because of FIFO overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifooverflow::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfifooverflowSpec;
impl crate::RegisterSpec for RxfifooverflowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifooverflow::R`](R) reader structure"]
impl crate::Readable for RxfifooverflowSpec {}
#[doc = "`reset()` method sets RXFIFOOVERFLOW to value 0"]
impl crate::Resettable for RxfifooverflowSpec {}
