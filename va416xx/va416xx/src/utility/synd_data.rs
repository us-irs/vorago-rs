#[doc = "Register `SYND_DATA` reader"]
pub type R = crate::R<SyndDataSpec>;
#[doc = "Register `SYND_DATA` writer"]
pub type W = crate::W<SyndDataSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`synd_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synd_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyndDataSpec;
impl crate::RegisterSpec for SyndDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synd_data::R`](R) reader structure"]
impl crate::Readable for SyndDataSpec {}
#[doc = "`write(|w| ..)` method takes [`synd_data::W`](W) writer structure"]
impl crate::Writable for SyndDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYND_DATA to value 0"]
impl crate::Resettable for SyndDataSpec {}
