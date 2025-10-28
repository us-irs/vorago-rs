#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    porta: [Porta; 16],
    portb: [Portb; 16],
    portc: [Portc; 16],
    portd: [Portd; 16],
    porte: [Porte; 16],
    portf: [Portf; 16],
    portg: [Portg; 8],
    _reserved7: [u8; 0x20],
    clkdiv0: Clkdiv0,
    clkdiv1: Clkdiv1,
    clkdiv2: Clkdiv2,
    clkdiv3: Clkdiv3,
    clkdiv4: Clkdiv4,
    clkdiv5: Clkdiv5,
    clkdiv6: Clkdiv6,
    clkdiv7: Clkdiv7,
    _reserved15: [u8; 0x0e1c],
    perid: Perid,
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - PORTA Pin Configuration Register"]
    #[inline(always)]
    pub const fn porta(&self, n: usize) -> &Porta {
        &self.porta[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - PORTA Pin Configuration Register"]
    #[inline(always)]
    pub fn porta_iter(&self) -> impl Iterator<Item = &Porta> {
        self.porta.iter()
    }
    #[doc = "0x40..0x80 - PORTB Pin Configuration Register"]
    #[inline(always)]
    pub const fn portb(&self, n: usize) -> &Portb {
        &self.portb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x80 - PORTB Pin Configuration Register"]
    #[inline(always)]
    pub fn portb_iter(&self) -> impl Iterator<Item = &Portb> {
        self.portb.iter()
    }
    #[doc = "0x80..0xc0 - PORTC Pin Configuration Register"]
    #[inline(always)]
    pub const fn portc(&self, n: usize) -> &Portc {
        &self.portc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc0 - PORTC Pin Configuration Register"]
    #[inline(always)]
    pub fn portc_iter(&self) -> impl Iterator<Item = &Portc> {
        self.portc.iter()
    }
    #[doc = "0xc0..0x100 - PORTD Pin Configuration Register"]
    #[inline(always)]
    pub const fn portd(&self, n: usize) -> &Portd {
        &self.portd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0x100 - PORTD Pin Configuration Register"]
    #[inline(always)]
    pub fn portd_iter(&self) -> impl Iterator<Item = &Portd> {
        self.portd.iter()
    }
    #[doc = "0x100..0x140 - PORTE Pin Configuration Register"]
    #[inline(always)]
    pub const fn porte(&self, n: usize) -> &Porte {
        &self.porte[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x140 - PORTE Pin Configuration Register"]
    #[inline(always)]
    pub fn porte_iter(&self) -> impl Iterator<Item = &Porte> {
        self.porte.iter()
    }
    #[doc = "0x140..0x180 - PORTF Pin Configuration Register"]
    #[inline(always)]
    pub const fn portf(&self, n: usize) -> &Portf {
        &self.portf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x180 - PORTF Pin Configuration Register"]
    #[inline(always)]
    pub fn portf_iter(&self) -> impl Iterator<Item = &Portf> {
        self.portf.iter()
    }
    #[doc = "0x180..0x1a0 - PORTG Pin Configuration Register"]
    #[inline(always)]
    pub const fn portg(&self, n: usize) -> &Portg {
        &self.portg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x1a0 - PORTG Pin Configuration Register"]
    #[inline(always)]
    pub fn portg_iter(&self) -> impl Iterator<Item = &Portg> {
        self.portg.iter()
    }
    #[doc = "0x1c0 - Clock divide value. 0 will disable the clock"]
    #[inline(always)]
    pub const fn clkdiv0(&self) -> &Clkdiv0 {
        &self.clkdiv0
    }
    #[doc = "0x1c4 - Clock divide value. 0 will disable the clock"]
    #[inline(always)]
    pub const fn clkdiv1(&self) -> &Clkdiv1 {
        &self.clkdiv1
    }
    #[doc = "0x1c8 - Clock divide value. 0 will disable the clock"]
    #[inline(always)]
    pub const fn clkdiv2(&self) -> &Clkdiv2 {
        &self.clkdiv2
    }
    #[doc = "0x1cc - Clock divide value. 0 will disable the clock"]
    #[inline(always)]
    pub const fn clkdiv3(&self) -> &Clkdiv3 {
        &self.clkdiv3
    }
    #[doc = "0x1d0 - Clock divide value. 0 will disable the clock"]
    #[inline(always)]
    pub const fn clkdiv4(&self) -> &Clkdiv4 {
        &self.clkdiv4
    }
    #[doc = "0x1d4 - Clock divide value. 0 will disable the clock"]
    #[inline(always)]
    pub const fn clkdiv5(&self) -> &Clkdiv5 {
        &self.clkdiv5
    }
    #[doc = "0x1d8 - Clock divide value. 0 will disable the clock"]
    #[inline(always)]
    pub const fn clkdiv6(&self) -> &Clkdiv6 {
        &self.clkdiv6
    }
    #[doc = "0x1dc - Clock divide value. 0 will disable the clock"]
    #[inline(always)]
    pub const fn clkdiv7(&self) -> &Clkdiv7 {
        &self.clkdiv7
    }
    #[doc = "0xffc - Peripheral ID Register"]
    #[inline(always)]
    pub const fn perid(&self) -> &Perid {
        &self.perid
    }
}
#[doc = "PORTA (rw) register accessor: PORTA Pin Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`porta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porta`] module"]
#[doc(alias = "PORTA")]
pub type Porta = crate::Reg<porta::PortaSpec>;
#[doc = "PORTA Pin Configuration Register"]
pub mod porta;
pub use porta as portb;
pub use porta as portc;
pub use porta as portd;
pub use porta as porte;
pub use porta as portf;
pub use porta as portg;
pub use Porta as Portb;
pub use Porta as Portc;
pub use Porta as Portd;
pub use Porta as Porte;
pub use Porta as Portf;
pub use Porta as Portg;
#[doc = "CLKDIV0 (r) register accessor: Clock divide value. 0 will disable the clock\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv0`] module"]
#[doc(alias = "CLKDIV0")]
pub type Clkdiv0 = crate::Reg<clkdiv0::Clkdiv0Spec>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv0;
#[doc = "CLKDIV1 (rw) register accessor: Clock divide value. 0 will disable the clock\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv1`] module"]
#[doc(alias = "CLKDIV1")]
pub type Clkdiv1 = crate::Reg<clkdiv1::Clkdiv1Spec>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv1;
#[doc = "CLKDIV2 (rw) register accessor: Clock divide value. 0 will disable the clock\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv2`] module"]
#[doc(alias = "CLKDIV2")]
pub type Clkdiv2 = crate::Reg<clkdiv2::Clkdiv2Spec>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv2;
#[doc = "CLKDIV3 (rw) register accessor: Clock divide value. 0 will disable the clock\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv3`] module"]
#[doc(alias = "CLKDIV3")]
pub type Clkdiv3 = crate::Reg<clkdiv3::Clkdiv3Spec>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv3;
#[doc = "CLKDIV4 (rw) register accessor: Clock divide value. 0 will disable the clock\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv4`] module"]
#[doc(alias = "CLKDIV4")]
pub type Clkdiv4 = crate::Reg<clkdiv4::Clkdiv4Spec>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv4;
#[doc = "CLKDIV5 (rw) register accessor: Clock divide value. 0 will disable the clock\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv5`] module"]
#[doc(alias = "CLKDIV5")]
pub type Clkdiv5 = crate::Reg<clkdiv5::Clkdiv5Spec>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv5;
#[doc = "CLKDIV6 (rw) register accessor: Clock divide value. 0 will disable the clock\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv6`] module"]
#[doc(alias = "CLKDIV6")]
pub type Clkdiv6 = crate::Reg<clkdiv6::Clkdiv6Spec>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv6;
#[doc = "CLKDIV7 (rw) register accessor: Clock divide value. 0 will disable the clock\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv7`] module"]
#[doc(alias = "CLKDIV7")]
pub type Clkdiv7 = crate::Reg<clkdiv7::Clkdiv7Spec>;
#[doc = "Clock divide value. 0 will disable the clock"]
pub mod clkdiv7;
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`] module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
