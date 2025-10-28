#[doc = "Register `SPW_M4_CTRL` reader"]
pub type R = crate::R<SpwM4CtrlSpec>;
#[doc = "Register `SPW_M4_CTRL` writer"]
pub type W = crate::W<SpwM4CtrlSpec>;
#[doc = "Field `REG_WR_KEY` reader - Fuse-analog register writes enabled when key = 0xfeed"]
pub type RegWrKeyR = crate::FieldReader<u16>;
#[doc = "Field `REG_WR_KEY` writer - Fuse-analog register writes enabled when key = 0xfeed"]
pub type RegWrKeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SPW_PAD_EN` reader - SPW pad enable"]
pub type SpwPadEnR = crate::BitReader;
#[doc = "Field `SPW_PAD_EN` writer - SPW pad enable"]
pub type SpwPadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LREN` reader - Lockup reset enable"]
pub type LrenR = crate::BitReader;
#[doc = "Field `LREN` writer - Lockup reset enable"]
pub type LrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Fuse-analog register writes enabled when key = 0xfeed"]
    #[inline(always)]
    pub fn reg_wr_key(&self) -> RegWrKeyR {
        RegWrKeyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - SPW pad enable"]
    #[inline(always)]
    pub fn spw_pad_en(&self) -> SpwPadEnR {
        SpwPadEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Lockup reset enable"]
    #[inline(always)]
    pub fn lren(&self) -> LrenR {
        LrenR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Fuse-analog register writes enabled when key = 0xfeed"]
    #[inline(always)]
    pub fn reg_wr_key(&mut self) -> RegWrKeyW<'_, SpwM4CtrlSpec> {
        RegWrKeyW::new(self, 0)
    }
    #[doc = "Bit 16 - SPW pad enable"]
    #[inline(always)]
    pub fn spw_pad_en(&mut self) -> SpwPadEnW<'_, SpwM4CtrlSpec> {
        SpwPadEnW::new(self, 16)
    }
    #[doc = "Bit 17 - Lockup reset enable"]
    #[inline(always)]
    pub fn lren(&mut self) -> LrenW<'_, SpwM4CtrlSpec> {
        LrenW::new(self, 17)
    }
}
#[doc = "SPW M4 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spw_m4_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spw_m4_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpwM4CtrlSpec;
impl crate::RegisterSpec for SpwM4CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spw_m4_ctrl::R`](R) reader structure"]
impl crate::Readable for SpwM4CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`spw_m4_ctrl::W`](W) writer structure"]
impl crate::Writable for SpwM4CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPW_M4_CTRL to value 0x0003_0000"]
impl crate::Resettable for SpwM4CtrlSpec {
    const RESET_VALUE: u32 = 0x0003_0000;
}
