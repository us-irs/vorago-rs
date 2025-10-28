#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    porta: [Porta; 32],
    portb: [Portb; 32],
    tim: [Tim; 32],
    uart: [Uart; 4],
    spi: [Spi; 4],
    i2c_ms: [I2cMs; 4],
    i2c_sl: [I2cSl; 4],
    int_ram_sbe: IntRamSbe,
    int_ram_mbe: IntRamMbe,
    int_rom_sbe: IntRomSbe,
    int_rom_mbe: IntRomMbe,
    txev: Txev,
    _reserved12: [u8; 0x062c],
    irqs: [Irqs; 32],
    _reserved13: [u8; 0x68],
    edbgrq: Edbgrq,
    mereset: Mereset,
    watchdog: Watchdog,
    rxev: Rxev,
    nmi: Nmi,
    _reserved18: [u8; 0x0700],
    perid: Perid,
}
impl RegisterBlock {
    #[doc = "0x00..0x80 - PORTA Interrupt Redirect Selection"]
    #[inline(always)]
    pub const fn porta(&self, n: usize) -> &Porta {
        &self.porta[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - PORTA Interrupt Redirect Selection"]
    #[inline(always)]
    pub fn porta_iter(&self) -> impl Iterator<Item = &Porta> {
        self.porta.iter()
    }
    #[doc = "0x80..0x100 - PORTB Interrupt Redirect Selection"]
    #[inline(always)]
    pub const fn portb(&self, n: usize) -> &Portb {
        &self.portb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - PORTB Interrupt Redirect Selection"]
    #[inline(always)]
    pub fn portb_iter(&self) -> impl Iterator<Item = &Portb> {
        self.portb.iter()
    }
    #[doc = "0x100..0x180 - TIM Interrupt Redirect Selection"]
    #[inline(always)]
    pub const fn tim(&self, n: usize) -> &Tim {
        &self.tim[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x180 - TIM Interrupt Redirect Selection"]
    #[inline(always)]
    pub fn tim_iter(&self) -> impl Iterator<Item = &Tim> {
        self.tim.iter()
    }
    #[doc = "0x180..0x190 - UART Interrupt Redirect Selection"]
    #[inline(always)]
    pub const fn uart(&self, n: usize) -> &Uart {
        &self.uart[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x190 - UART Interrupt Redirect Selection"]
    #[inline(always)]
    pub fn uart_iter(&self) -> impl Iterator<Item = &Uart> {
        self.uart.iter()
    }
    #[doc = "0x190..0x1a0 - SPI Interrupt Redirect Selection"]
    #[inline(always)]
    pub const fn spi(&self, n: usize) -> &Spi {
        &self.spi[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x1a0 - SPI Interrupt Redirect Selection"]
    #[inline(always)]
    pub fn spi_iter(&self) -> impl Iterator<Item = &Spi> {
        self.spi.iter()
    }
    #[doc = "0x1a0..0x1b0 - Master I2C Interrupt Redirect Selection"]
    #[inline(always)]
    pub const fn i2c_ms(&self, n: usize) -> &I2cMs {
        &self.i2c_ms[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1a0..0x1b0 - Master I2C Interrupt Redirect Selection"]
    #[inline(always)]
    pub fn i2c_ms_iter(&self) -> impl Iterator<Item = &I2cMs> {
        self.i2c_ms.iter()
    }
    #[doc = "0x1b0..0x1c0 - Slave I2C Interrupt Redirect Selection"]
    #[inline(always)]
    pub const fn i2c_sl(&self, n: usize) -> &I2cSl {
        &self.i2c_sl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1b0..0x1c0 - Slave I2C Interrupt Redirect Selection"]
    #[inline(always)]
    pub fn i2c_sl_iter(&self) -> impl Iterator<Item = &I2cSl> {
        self.i2c_sl.iter()
    }
    #[doc = "0x1c0 - Internal Memory RAM SBE Interrupt Redirect Selection"]
    #[inline(always)]
    pub const fn int_ram_sbe(&self) -> &IntRamSbe {
        &self.int_ram_sbe
    }
    #[doc = "0x1c4 - Internal Memory RAM MBE Interrupt Redirect Selection"]
    #[inline(always)]
    pub const fn int_ram_mbe(&self) -> &IntRamMbe {
        &self.int_ram_mbe
    }
    #[doc = "0x1c8 - Internal Memory ROM SBE Interrupt Redirect Selection"]
    #[inline(always)]
    pub const fn int_rom_sbe(&self) -> &IntRomSbe {
        &self.int_rom_sbe
    }
    #[doc = "0x1cc - Internal Memory ROM MBE Interrupt Redirect Selection"]
    #[inline(always)]
    pub const fn int_rom_mbe(&self) -> &IntRomMbe {
        &self.int_rom_mbe
    }
    #[doc = "0x1d0 - Processor TXEV Interrupt Redirect Selection"]
    #[inline(always)]
    pub const fn txev(&self) -> &Txev {
        &self.txev
    }
    #[doc = "0x800..0x880 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn irqs(&self, n: usize) -> &Irqs {
        &self.irqs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x880 - Interrupt Status Register"]
    #[inline(always)]
    pub fn irqs_iter(&self) -> impl Iterator<Item = &Irqs> {
        self.irqs.iter()
    }
    #[doc = "0x8e8 - EDBGRQ Status Register"]
    #[inline(always)]
    pub const fn edbgrq(&self) -> &Edbgrq {
        &self.edbgrq
    }
    #[doc = "0x8ec - MERESET Status Register"]
    #[inline(always)]
    pub const fn mereset(&self) -> &Mereset {
        &self.mereset
    }
    #[doc = "0x8f0 - WATCHDOG Status Register"]
    #[inline(always)]
    pub const fn watchdog(&self) -> &Watchdog {
        &self.watchdog
    }
    #[doc = "0x8f4 - RXEV Status Register"]
    #[inline(always)]
    pub const fn rxev(&self) -> &Rxev {
        &self.rxev
    }
    #[doc = "0x8f8 - NMI Status Register"]
    #[inline(always)]
    pub const fn nmi(&self) -> &Nmi {
        &self.nmi
    }
    #[doc = "0xffc - Peripheral ID Register"]
    #[inline(always)]
    pub const fn perid(&self) -> &Perid {
        &self.perid
    }
}
#[doc = "INT_RAM_SBE (rw) register accessor: Internal Memory RAM SBE Interrupt Redirect Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ram_sbe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ram_sbe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ram_sbe`] module"]
#[doc(alias = "INT_RAM_SBE")]
pub type IntRamSbe = crate::Reg<int_ram_sbe::IntRamSbeSpec>;
#[doc = "Internal Memory RAM SBE Interrupt Redirect Selection"]
pub mod int_ram_sbe;
pub use int_ram_sbe as porta;
pub use int_ram_sbe as portb;
pub use int_ram_sbe as tim;
pub use int_ram_sbe as uart;
pub use int_ram_sbe as spi;
pub use int_ram_sbe as i2c_ms;
pub use int_ram_sbe as i2c_sl;
pub use int_ram_sbe as int_ram_mbe;
pub use int_ram_sbe as int_rom_sbe;
pub use int_ram_sbe as int_rom_mbe;
pub use int_ram_sbe as txev;
pub use IntRamSbe as Porta;
pub use IntRamSbe as Portb;
pub use IntRamSbe as Tim;
pub use IntRamSbe as Uart;
pub use IntRamSbe as Spi;
pub use IntRamSbe as I2cMs;
pub use IntRamSbe as I2cSl;
pub use IntRamSbe as IntRamMbe;
pub use IntRamSbe as IntRomSbe;
pub use IntRamSbe as IntRomMbe;
pub use IntRamSbe as Txev;
#[doc = "NMI (r) register accessor: NMI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmi`] module"]
#[doc(alias = "NMI")]
pub type Nmi = crate::Reg<nmi::NmiSpec>;
#[doc = "NMI Status Register"]
pub mod nmi;
pub use nmi as rxev;
pub use nmi as watchdog;
pub use nmi as mereset;
pub use nmi as edbgrq;
pub use nmi as irqs;
pub use Nmi as Rxev;
pub use Nmi as Watchdog;
pub use Nmi as Mereset;
pub use Nmi as Edbgrq;
pub use Nmi as Irqs;
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`] module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
