#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `FBSLIP` reader - Feedback cycle slip output (CLKOUT frequency low)"]
pub type FbslipR = crate::BitReader;
#[doc = "Field `RFSLIP` reader - Reference cycle slip output (CLKOUT frequency high)"]
pub type RfslipR = crate::BitReader;
#[doc = "Field `LOCKLOST` reader - LOCK high Symbol indicates that RFLSIP or FBSLIP have occurred for 64 consecutive cycles"]
pub type LocklostR = crate::BitReader;
#[doc = "Field `SYSCLKLOST` reader - Set when SYS_CLK has dropped to less than 1MHz"]
pub type SysclklostR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Feedback cycle slip output (CLKOUT frequency low)"]
    #[inline(always)]
    pub fn fbslip(&self) -> FbslipR {
        FbslipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reference cycle slip output (CLKOUT frequency high)"]
    #[inline(always)]
    pub fn rfslip(&self) -> RfslipR {
        RfslipR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LOCK high Symbol indicates that RFLSIP or FBSLIP have occurred for 64 consecutive cycles"]
    #[inline(always)]
    pub fn locklost(&self) -> LocklostR {
        LocklostR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set when SYS_CLK has dropped to less than 1MHz"]
    #[inline(always)]
    pub fn sysclklost(&self) -> SysclklostR {
        SysclklostR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Clock Generation Module Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
