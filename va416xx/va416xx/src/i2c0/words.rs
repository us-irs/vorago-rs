#[doc = "Register `WORDS` reader"]
pub type R = crate::R<WordsSpec>;
#[doc = "Register `WORDS` writer"]
pub type W = crate::W<WordsSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Word Count value\n\nYou can [`read`](crate::Reg::read) this register and get [`words::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`words::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WordsSpec;
impl crate::RegisterSpec for WordsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`words::R`](R) reader structure"]
impl crate::Readable for WordsSpec {}
#[doc = "`write(|w| ..)` method takes [`words::W`](W) writer structure"]
impl crate::Writable for WordsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WORDS to value 0"]
impl crate::Resettable for WordsSpec {}
