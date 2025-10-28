#[doc = "Register `SAMPLE_CNT1` reader"]
pub type R = crate::R<SampleCnt1Spec>;
#[doc = "Register `SAMPLE_CNT1` writer"]
pub type W = crate::W<SampleCnt1Spec>;
#[doc = "Field `SAMPLE_CNTR1` reader - Sets the number of clk cycles between two consecutive ring oscillator samples"]
pub type SampleCntr1R = crate::FieldReader<u32>;
#[doc = "Field `SAMPLE_CNTR1` writer - Sets the number of clk cycles between two consecutive ring oscillator samples"]
pub type SampleCntr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sets the number of clk cycles between two consecutive ring oscillator samples"]
    #[inline(always)]
    pub fn sample_cntr1(&self) -> SampleCntr1R {
        SampleCntr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sets the number of clk cycles between two consecutive ring oscillator samples"]
    #[inline(always)]
    pub fn sample_cntr1(&mut self) -> SampleCntr1W<'_, SampleCnt1Spec> {
        SampleCntr1W::new(self, 0)
    }
}
#[doc = "Section TBD\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_cnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_cnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SampleCnt1Spec;
impl crate::RegisterSpec for SampleCnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sample_cnt1::R`](R) reader structure"]
impl crate::Readable for SampleCnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`sample_cnt1::W`](W) writer structure"]
impl crate::Writable for SampleCnt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMPLE_CNT1 to value 0xffff"]
impl crate::Resettable for SampleCnt1Spec {
    const RESET_VALUE: u32 = 0xffff;
}
