#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    rst_value: RstValue,
    cnt_value: CntValue,
    enable: Enable,
    csd_ctrl: CsdCtrl,
    cascade0: Cascade0,
    cascade1: Cascade1,
    cascade2: Cascade2,
    _reserved_8_pwm_value: [u8; 0x04],
    pwmb_value: PwmbValue,
    _reserved10: [u8; 0x0fd4],
    perid: Perid,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - The value that counter start from after reaching 0."]
    #[inline(always)]
    pub const fn rst_value(&self) -> &RstValue {
        &self.rst_value
    }
    #[doc = "0x08 - The current value of the counter"]
    #[inline(always)]
    pub const fn cnt_value(&self) -> &CntValue {
        &self.cnt_value
    }
    #[doc = "0x0c - Alternate access to the Counter ENABLE bit in the CTRL Register"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x10 - The Cascade Control Register. Controls the counter external enable signals"]
    #[inline(always)]
    pub const fn csd_ctrl(&self) -> &CsdCtrl {
        &self.csd_ctrl
    }
    #[doc = "0x14 - Cascade Enable Selection"]
    #[inline(always)]
    pub const fn cascade0(&self) -> &Cascade0 {
        &self.cascade0
    }
    #[doc = "0x18 - Cascade Enable Selection"]
    #[inline(always)]
    pub const fn cascade1(&self) -> &Cascade1 {
        &self.cascade1
    }
    #[doc = "0x1c - Cascade Enable Selection"]
    #[inline(always)]
    pub const fn cascade2(&self) -> &Cascade2 {
        &self.cascade2
    }
    #[doc = "0x20 - The Pulse Width Modulation ValueA"]
    #[inline(always)]
    pub const fn pwma_value(&self) -> &PwmaValue {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - The Pulse Width Modulation Value"]
    #[inline(always)]
    pub const fn pwm_value(&self) -> &PwmValue {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - The Pulse Width Modulation ValueB"]
    #[inline(always)]
    pub const fn pwmb_value(&self) -> &PwmbValue {
        &self.pwmb_value
    }
    #[doc = "0xffc - Peripheral ID Register"]
    #[inline(always)]
    pub const fn perid(&self) -> &Perid {
        &self.perid
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "RST_VALUE (rw) register accessor: The value that counter start from after reaching 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_value`] module"]
#[doc(alias = "RST_VALUE")]
pub type RstValue = crate::Reg<rst_value::RstValueSpec>;
#[doc = "The value that counter start from after reaching 0."]
pub mod rst_value;
#[doc = "CNT_VALUE (rw) register accessor: The current value of the counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt_value`] module"]
#[doc(alias = "CNT_VALUE")]
pub type CntValue = crate::Reg<cnt_value::CntValueSpec>;
#[doc = "The current value of the counter"]
pub mod cnt_value;
#[doc = "ENABLE (rw) register accessor: Alternate access to the Counter ENABLE bit in the CTRL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Alternate access to the Counter ENABLE bit in the CTRL Register"]
pub mod enable;
#[doc = "CSD_CTRL (rw) register accessor: The Cascade Control Register. Controls the counter external enable signals\n\nYou can [`read`](crate::Reg::read) this register and get [`csd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csd_ctrl`] module"]
#[doc(alias = "CSD_CTRL")]
pub type CsdCtrl = crate::Reg<csd_ctrl::CsdCtrlSpec>;
#[doc = "The Cascade Control Register. Controls the counter external enable signals"]
pub mod csd_ctrl;
#[doc = "CASCADE0 (rw) register accessor: Cascade Enable Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`cascade0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cascade0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cascade0`] module"]
#[doc(alias = "CASCADE0")]
pub type Cascade0 = crate::Reg<cascade0::Cascade0Spec>;
#[doc = "Cascade Enable Selection"]
pub mod cascade0;
pub use cascade0 as cascade1;
pub use cascade0 as cascade2;
pub use Cascade0 as Cascade1;
pub use Cascade0 as Cascade2;
#[doc = "PWM_VALUE (rw) register accessor: The Pulse Width Modulation Value\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_value`] module"]
#[doc(alias = "PWM_VALUE")]
pub type PwmValue = crate::Reg<pwm_value::PwmValueSpec>;
#[doc = "The Pulse Width Modulation Value"]
pub mod pwm_value;
#[doc = "PWMA_VALUE (rw) register accessor: The Pulse Width Modulation ValueA\n\nYou can [`read`](crate::Reg::read) this register and get [`pwma_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwma_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwma_value`] module"]
#[doc(alias = "PWMA_VALUE")]
pub type PwmaValue = crate::Reg<pwma_value::PwmaValueSpec>;
#[doc = "The Pulse Width Modulation ValueA"]
pub mod pwma_value;
#[doc = "PWMB_VALUE (rw) register accessor: The Pulse Width Modulation ValueB\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmb_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmb_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmb_value`] module"]
#[doc(alias = "PWMB_VALUE")]
pub type PwmbValue = crate::Reg<pwmb_value::PwmbValueSpec>;
#[doc = "The Pulse Width Modulation ValueB"]
pub mod pwmb_value;
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`] module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
