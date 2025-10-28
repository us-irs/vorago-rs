#[doc = "Register `EDGE_STATUS` reader"]
pub type R = crate::R<EdgeStatusSpec>;
#[doc = "Register `EDGE_STATUS` writer"]
pub type W = crate::W<EdgeStatusSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Edge Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`edge_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edge_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EdgeStatusSpec;
impl crate::RegisterSpec for EdgeStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edge_status::R`](R) reader structure"]
impl crate::Readable for EdgeStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`edge_status::W`](W) writer structure"]
impl crate::Writable for EdgeStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EDGE_STATUS to value 0"]
impl crate::Resettable for EdgeStatusSpec {}
