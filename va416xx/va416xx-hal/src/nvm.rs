//! Non-volatile memory (NVM) driver.
//!
//! Provides a basic API to work with the internal NVM of the VA41630 MCU.
//!
//! # Examples
//!
//! - [Flashloader application](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/flashloader)
use embedded_hal::spi::MODE_0;
use vorago_shared_hal::{
    disable_peripheral_clock, enable_peripheral_clock, reset_peripheral_for_cycles,
};

use crate::clock::Clocks;
use crate::pac;
use crate::spi::{
    mode_to_cpo_cph_bit, spi_clk_config_from_div, SpiInstance, SpiWord, BMSTART_BMSTOP_MASK,
};

const NVM_CLOCK_DIV: u16 = 2;

// Commands. The internal FRAM is based on the Cypress FM25V20A device.

/// Write enable register.
pub const FRAM_WREN: u8 = 0x06;
pub const FRAM_WRDI: u8 = 0x04;
pub const FRAM_RDSR: u8 = 0x05;
/// Write single status register
pub const FRAM_WRSR: u8 = 0x01;
pub const FRAM_READ: u8 = 0x03;
pub const FRAM_WRITE: u8 = 0x02;
pub const FRAM_RDID: u8 = 0x9F;
pub const FRAM_SLEEP: u8 = 0xB9;

// Address Masks

const ADDR_MSB_MASK: u32 = 0xFF0000;
const ADDR_MID_MASK: u32 = 0x00FF00;
const ADDR_LSB_MASK: u32 = 0x0000FF;

#[inline(always)]
const fn msb_addr_byte(addr: u32) -> u8 {
    ((addr & ADDR_MSB_MASK) >> 16) as u8
}

#[inline(always)]
const fn mid_addr_byte(addr: u32) -> u8 {
    ((addr & ADDR_MID_MASK) >> 8) as u8
}

#[inline(always)]
const fn lsb_addr_byte(addr: u32) -> u8 {
    (addr & ADDR_LSB_MASK) as u8
}

pub const WPEN_ENABLE_MASK: u8 = 1 << 7;
pub const BP_0_ENABLE_MASK: u8 = 1 << 2;
pub const BP_1_ENABLE_MASK: u8 = 1 << 3;

pub struct Nvm {
    spi: Option<pac::Spi3>,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct VerifyError {
    addr: u32,
    found: u8,
    expected: u8,
}

impl Nvm {
    pub fn new(spi: pac::Spi3, _clocks: &Clocks) -> Self {
        enable_peripheral_clock(pac::Spi3::PERIPH_SEL);
        // This is done in the C HAL.
        reset_peripheral_for_cycles(pac::Spi3::PERIPH_SEL, 2);

        let spi_clk_cfg = spi_clk_config_from_div(NVM_CLOCK_DIV).unwrap();
        let (cpo_bit, cph_bit) = mode_to_cpo_cph_bit(MODE_0);
        spi.ctrl0().write(|w| {
            unsafe {
                w.size().bits(u8::word_reg());
                w.scrdv().bits(spi_clk_cfg.scrdv());
                // Clear clock phase and polarity. Will be set to correct value for each
                // transfer
                w.spo().bit(cpo_bit);
                w.sph().bit(cph_bit)
            }
        });
        spi.ctrl1().write(|w| {
            w.blockmode().set_bit();
            unsafe { w.ss().bits(0) };
            w.bmstart().set_bit();
            w.bmstall().set_bit()
        });
        spi.clkprescale()
            .write(|w| unsafe { w.bits(spi_clk_cfg.prescale_val() as u32) });

        spi.fifo_clr().write(|w| {
            w.rxfifo().set_bit();
            w.txfifo().set_bit()
        });
        // Enable the peripheral as the last step as recommended in the
        // programmers guide
        spi.ctrl1().modify(|_, w| w.enable().set_bit());

        let mut nvm = Self { spi: Some(spi) };
        nvm.disable_write_prot();
        nvm
    }

    pub fn disable_write_prot(&mut self) {
        self.wait_for_tx_idle();
        self.write_with_bmstop(FRAM_WREN);
        self.wait_for_tx_idle();
        self.write_single(FRAM_WRSR);
        self.write_with_bmstop(0x00);
        self.wait_for_tx_idle();
    }

    pub fn read_rdsr(&self) -> u8 {
        self.write_single(FRAM_RDSR);
        self.write_with_bmstop(0x00);
        self.wait_for_rx_available();
        self.read_single_word();
        self.wait_for_rx_available();
        (self.read_single_word() & 0xff) as u8
    }

    pub fn enable_write_prot(&mut self) {
        self.wait_for_tx_idle();
        self.write_with_bmstop(FRAM_WREN);
        self.wait_for_tx_idle();
        self.write_single(FRAM_WRSR);
        self.write_with_bmstop(0x00);
    }
    #[inline(always)]
    pub fn spi(&self) -> &pac::Spi3 {
        self.spi.as_ref().unwrap()
    }

    #[inline(always)]
    pub fn write_single(&self, word: u8) {
        self.spi().data().write(|w| unsafe { w.bits(word as u32) });
    }

    #[inline(always)]
    pub fn write_with_bmstop(&self, word: u8) {
        self.spi()
            .data()
            .write(|w| unsafe { w.bits(BMSTART_BMSTOP_MASK | word as u32) });
    }

    #[inline(always)]
    pub fn wait_for_tx_idle(&self) {
        while self.spi().status().read().tfe().bit_is_clear() {
            cortex_m::asm::nop();
        }
        while self.spi().status().read().busy().bit_is_set() {
            cortex_m::asm::nop();
        }
        self.clear_fifos()
    }

    #[inline(always)]
    pub fn clear_fifos(&self) {
        self.spi().fifo_clr().write(|w| {
            w.rxfifo().set_bit();
            w.txfifo().set_bit()
        });
    }

    #[inline(always)]
    pub fn wait_for_rx_available(&self) {
        while !self.spi().status().read().rne().bit_is_set() {
            cortex_m::asm::nop();
        }
    }

    #[inline(always)]
    pub fn read_single_word(&self) -> u32 {
        self.spi().data().read().bits()
    }

    pub fn write_data(&self, addr: u32, data: &[u8]) {
        self.wait_for_tx_idle();
        self.write_with_bmstop(FRAM_WREN);
        self.wait_for_tx_idle();
        self.write_single(FRAM_WRITE);
        self.write_single(msb_addr_byte(addr));
        self.write_single(mid_addr_byte(addr));
        self.write_single(lsb_addr_byte(addr));
        for byte in data.iter().take(data.len() - 1) {
            while self.spi().status().read().tnf().bit_is_clear() {
                cortex_m::asm::nop();
            }
            self.write_single(*byte);
            self.read_single_word();
        }
        while self.spi().status().read().tnf().bit_is_clear() {
            cortex_m::asm::nop();
        }
        self.write_with_bmstop(*data.last().unwrap());
        self.wait_for_tx_idle();
    }

    pub fn read_data(&self, addr: u32, buf: &mut [u8]) {
        self.common_read_start(addr);
        for byte in buf {
            // Pump the SPI.
            self.write_single(0);
            self.wait_for_rx_available();
            *byte = self.read_single_word() as u8;
        }
        self.write_with_bmstop(0);
        self.wait_for_tx_idle();
    }

    pub fn verify_data(&self, addr: u32, comp_buf: &[u8]) -> Result<(), VerifyError> {
        self.common_read_start(addr);
        for (idx, byte) in comp_buf.iter().enumerate() {
            // Pump the SPI.
            self.write_single(0);
            self.wait_for_rx_available();
            let next_word = self.read_single_word() as u8;
            if next_word != *byte {
                self.write_with_bmstop(0);
                self.wait_for_tx_idle();
                return Err(VerifyError {
                    addr: addr + idx as u32,
                    found: next_word,
                    expected: *byte,
                });
            }
        }
        self.write_with_bmstop(0);
        self.wait_for_tx_idle();
        Ok(())
    }

    /// Enable write-protection and disables the peripheral clock.
    pub fn shutdown(&mut self) {
        self.wait_for_tx_idle();
        self.write_with_bmstop(FRAM_WREN);
        self.wait_for_tx_idle();
        self.write_single(WPEN_ENABLE_MASK | BP_0_ENABLE_MASK | BP_1_ENABLE_MASK);
        disable_peripheral_clock(pac::Spi3::PERIPH_SEL);
    }

    /// This function calls [Self::shutdown] and gives back the peripheral structure.
    pub fn release(mut self) -> pac::Spi3 {
        self.shutdown();
        self.spi.take().unwrap()
    }

    fn common_read_start(&self, addr: u32) {
        self.wait_for_tx_idle();
        self.write_single(FRAM_READ);
        self.write_single(msb_addr_byte(addr));
        self.write_single(mid_addr_byte(addr));
        self.write_single(lsb_addr_byte(addr));
        for _ in 0..4 {
            // Pump the SPI.
            self.write_single(0);
            self.wait_for_rx_available();
            // The first 4 data bytes received need to be ignored.
            self.read_single_word();
        }
    }
}

/// Call [Self::shutdown] on drop.
impl Drop for Nvm {
    fn drop(&mut self) {
        if self.spi.is_some() {
            self.shutdown();
        }
    }
}
