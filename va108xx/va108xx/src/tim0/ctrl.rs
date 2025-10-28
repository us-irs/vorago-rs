#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ENABLE` reader - Counter Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Counter Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE` reader - Counter Active"]
pub type ActiveR = crate::BitReader;
#[doc = "Field `AUTO_DISABLE` reader - Auto Disables the counter (set ENABLE to 0) when the count reaches 0"]
pub type AutoDisableR = crate::BitReader;
#[doc = "Field `AUTO_DISABLE` writer - Auto Disables the counter (set ENABLE to 0) when the count reaches 0"]
pub type AutoDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_DEACTIVATE` reader - Auto Deactivate the counter (set ACTIVE to 0) when the count reaches 0"]
pub type AutoDeactivateR = crate::BitReader;
#[doc = "Field `AUTO_DEACTIVATE` writer - Auto Deactivate the counter (set ACTIVE to 0) when the count reaches 0"]
pub type AutoDeactivateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_ENB` reader - Interrupt Enable"]
pub type IrqEnbR = crate::BitReader;
#[doc = "Field `IRQ_ENB` writer - Interrupt Enable"]
pub type IrqEnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Counter Status Selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StatusSel {
    #[doc = "0: Single cycle pulse when the counter reaches 0"]
    Done = 0,
    #[doc = "1: Returns the counter ACTIVE bit"]
    Active = 1,
    #[doc = "2: Toggles the STATUS bit everytime the counter reaches 0. Basically a divide by 2 counter output."]
    Toggle = 2,
    #[doc = "3: Selects the Pulse Width Modulated output. It 1 when the counter value is >= the PWMA_VALUE"]
    Pwma = 3,
    #[doc = "4: Selects the Pulse Width Modulated output. It 1 when the counter value is < the PWMA_VALUE and value is > PWMA_VALUE"]
    Pwmb = 4,
    #[doc = "5: Returns the counter ENABLED bit"]
    Enabled = 5,
    #[doc = "6: Selects the Pulse Width Modulated output. It 1 when the counter value is <= the PWMA_VALUE and value is >= 0"]
    PwmaActive = 6,
}
impl From<StatusSel> for u8 {
    #[inline(always)]
    fn from(variant: StatusSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for StatusSel {
    type Ux = u8;
}
impl crate::IsEnum for StatusSel {}
#[doc = "Field `STATUS_SEL` reader - Counter Status Selection"]
pub type StatusSelR = crate::FieldReader<StatusSel>;
impl StatusSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<StatusSel> {
        match self.bits {
            0 => Some(StatusSel::Done),
            1 => Some(StatusSel::Active),
            2 => Some(StatusSel::Toggle),
            3 => Some(StatusSel::Pwma),
            4 => Some(StatusSel::Pwmb),
            5 => Some(StatusSel::Enabled),
            6 => Some(StatusSel::PwmaActive),
            _ => None,
        }
    }
    #[doc = "Single cycle pulse when the counter reaches 0"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == StatusSel::Done
    }
    #[doc = "Returns the counter ACTIVE bit"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == StatusSel::Active
    }
    #[doc = "Toggles the STATUS bit everytime the counter reaches 0. Basically a divide by 2 counter output."]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == StatusSel::Toggle
    }
    #[doc = "Selects the Pulse Width Modulated output. It 1 when the counter value is >= the PWMA_VALUE"]
    #[inline(always)]
    pub fn is_pwma(&self) -> bool {
        *self == StatusSel::Pwma
    }
    #[doc = "Selects the Pulse Width Modulated output. It 1 when the counter value is < the PWMA_VALUE and value is > PWMA_VALUE"]
    #[inline(always)]
    pub fn is_pwmb(&self) -> bool {
        *self == StatusSel::Pwmb
    }
    #[doc = "Returns the counter ENABLED bit"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == StatusSel::Enabled
    }
    #[doc = "Selects the Pulse Width Modulated output. It 1 when the counter value is <= the PWMA_VALUE and value is >= 0"]
    #[inline(always)]
    pub fn is_pwma_active(&self) -> bool {
        *self == StatusSel::PwmaActive
    }
}
#[doc = "Field `STATUS_SEL` writer - Counter Status Selection"]
pub type StatusSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, StatusSel>;
impl<'a, REG> StatusSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single cycle pulse when the counter reaches 0"]
    #[inline(always)]
    pub fn done(self) -> &'a mut crate::W<REG> {
        self.variant(StatusSel::Done)
    }
    #[doc = "Returns the counter ACTIVE bit"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(StatusSel::Active)
    }
    #[doc = "Toggles the STATUS bit everytime the counter reaches 0. Basically a divide by 2 counter output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(StatusSel::Toggle)
    }
    #[doc = "Selects the Pulse Width Modulated output. It 1 when the counter value is >= the PWMA_VALUE"]
    #[inline(always)]
    pub fn pwma(self) -> &'a mut crate::W<REG> {
        self.variant(StatusSel::Pwma)
    }
    #[doc = "Selects the Pulse Width Modulated output. It 1 when the counter value is < the PWMA_VALUE and value is > PWMA_VALUE"]
    #[inline(always)]
    pub fn pwmb(self) -> &'a mut crate::W<REG> {
        self.variant(StatusSel::Pwmb)
    }
    #[doc = "Returns the counter ENABLED bit"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(StatusSel::Enabled)
    }
    #[doc = "Selects the Pulse Width Modulated output. It 1 when the counter value is <= the PWMA_VALUE and value is >= 0"]
    #[inline(always)]
    pub fn pwma_active(self) -> &'a mut crate::W<REG> {
        self.variant(StatusSel::PwmaActive)
    }
}
#[doc = "Field `STATUS_INV` reader - Invert the Output Status"]
pub type StatusInvR = crate::BitReader;
#[doc = "Field `STATUS_INV` writer - Invert the Output Status"]
pub type StatusInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQ_STOP` reader - Stop Request"]
pub type ReqStopR = crate::BitReader;
#[doc = "Field `REQ_STOP` writer - Stop Request"]
pub type ReqStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Active"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto Disables the counter (set ENABLE to 0) when the count reaches 0"]
    #[inline(always)]
    pub fn auto_disable(&self) -> AutoDisableR {
        AutoDisableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto Deactivate the counter (set ACTIVE to 0) when the count reaches 0"]
    #[inline(always)]
    pub fn auto_deactivate(&self) -> AutoDeactivateR {
        AutoDeactivateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Enable"]
    #[inline(always)]
    pub fn irq_enb(&self) -> IrqEnbR {
        IrqEnbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Counter Status Selection"]
    #[inline(always)]
    pub fn status_sel(&self) -> StatusSelR {
        StatusSelR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Invert the Output Status"]
    #[inline(always)]
    pub fn status_inv(&self) -> StatusInvR {
        StatusInvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop Request"]
    #[inline(always)]
    pub fn req_stop(&self) -> ReqStopR {
        ReqStopR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 2 - Auto Disables the counter (set ENABLE to 0) when the count reaches 0"]
    #[inline(always)]
    pub fn auto_disable(&mut self) -> AutoDisableW<'_, CtrlSpec> {
        AutoDisableW::new(self, 2)
    }
    #[doc = "Bit 3 - Auto Deactivate the counter (set ACTIVE to 0) when the count reaches 0"]
    #[inline(always)]
    pub fn auto_deactivate(&mut self) -> AutoDeactivateW<'_, CtrlSpec> {
        AutoDeactivateW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Enable"]
    #[inline(always)]
    pub fn irq_enb(&mut self) -> IrqEnbW<'_, CtrlSpec> {
        IrqEnbW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Counter Status Selection"]
    #[inline(always)]
    pub fn status_sel(&mut self) -> StatusSelW<'_, CtrlSpec> {
        StatusSelW::new(self, 5)
    }
    #[doc = "Bit 8 - Invert the Output Status"]
    #[inline(always)]
    pub fn status_inv(&mut self) -> StatusInvW<'_, CtrlSpec> {
        StatusInvW::new(self, 8)
    }
    #[doc = "Bit 9 - Stop Request"]
    #[inline(always)]
    pub fn req_stop(&mut self) -> ReqStopW<'_, CtrlSpec> {
        ReqStopW::new(self, 9)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
