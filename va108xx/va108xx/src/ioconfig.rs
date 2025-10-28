#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    porta: [Porta; 32],
    portb: [Portb; 32],
    _reserved2: [u8; 0x0efc],
    perid: Perid,
}
impl RegisterBlock {
    #[doc = "0x00..0x80 - PORTA Pin Configuration Register"]
    #[inline(always)]
    pub const fn porta(&self, n: usize) -> &Porta {
        &self.porta[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - PORTA Pin Configuration Register"]
    #[inline(always)]
    pub fn porta_iter(&self) -> impl Iterator<Item = &Porta> {
        self.porta.iter()
    }
    #[doc = "0x80..0x100 - PORTB Pin Configuration Register"]
    #[inline(always)]
    pub const fn portb(&self, n: usize) -> &Portb {
        &self.portb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - PORTB Pin Configuration Register"]
    #[inline(always)]
    pub fn portb_iter(&self) -> impl Iterator<Item = &Portb> {
        self.portb.iter()
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
pub use Porta as Portb;
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`] module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
