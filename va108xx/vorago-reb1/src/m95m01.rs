//! Basic driver for the ST M95M01 EEPROM memory.
//!
//! This driver is used by the provided bootloader application for the REB1
//! board. It provides a convenient wrapper around the HAL SPI to interface
//! with the EEPROM memory of the REB1 board.
//!
//! # Example
//!
//! - [REB1 EEPROM example](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/vorago-reb1/examples/nvm.rs)
use arbitrary_int::{u2, u3};
use core::convert::Infallible;
use embedded_hal::spi::SpiBus;

pub const PAGE_SIZE: usize = 256;

#[bitbybit::bitfield(u8)]
#[derive(Debug)]
pub struct StatusReg {
    #[bit(7, r)]
    status_register_write_protect: bool,
    #[bits(4..=6, r)]
    zero_segment: u3,
    #[bits(2..=3, rw)]
    block_protection_bits: u2,
    #[bit(1, r)]
    write_enable_latch: bool,
    #[bit(0, r)]
    write_in_progress: bool,
}

// Registers.
pub mod regs {
    /// Write status register command.
    pub const WRSR: u8 = 0x01;
    // Write command.
    pub const WRITE: u8 = 0x02;
    // Read command.
    pub const READ: u8 = 0x03;
    /// Write disable command.
    pub const WRDI: u8 = 0x04;
    /// Read status register command.
    pub const RDSR: u8 = 0x05;
    /// Write enable command.
    pub const WREN: u8 = 0x06;
}

use regs::*;
use va108xx_hal::{
    pac,
    spi::{Spi, SpiClockConfig, SpiConfig, SpiLowLevel, BMSTART_BMSTOP_MASK},
};

pub type RomSpi = Spi<u8>;

/// Driver for the ST device M95M01 EEPROM memory.
///
/// Specialized for the requirements of the VA108XX MCUs.
pub struct M95M01 {
    pub spi: RomSpi,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PageBoundaryExceededError;

impl M95M01 {
    pub fn new(spi: pac::Spic, clk_config: SpiClockConfig) -> Self {
        let spi = RomSpi::new_for_rom(spi, SpiConfig::default().clk_cfg(clk_config)).unwrap();
        let mut spi_dev = Self { spi };
        spi_dev.clear_block_protection().unwrap();
        spi_dev
    }

    pub fn release(mut self) -> pac::Spic {
        self.set_block_protection().unwrap();
        unsafe { pac::Spic::steal() }
    }

    // Wait until the write-in-progress state is cleared. This exposes a [nb] API, so this function
    // will return [nb::Error::WouldBlock] if the EEPROM is still busy.
    pub fn writes_are_done(&mut self) -> nb::Result<(), Infallible> {
        let rdsr = self.read_status_reg()?;
        if rdsr.write_in_progress() {
            return Err(nb::Error::WouldBlock);
        }
        Ok(())
    }

    pub fn read_status_reg(&mut self) -> Result<StatusReg, Infallible> {
        let mut write_read: [u8; 2] = [regs::RDSR, 0x00];
        self.spi.transfer_in_place(&mut write_read)?;
        Ok(StatusReg::new_with_raw_value(write_read[1]))
    }

    pub fn write_enable(&mut self) -> Result<(), Infallible> {
        self.spi.write(&[regs::WREN])
    }

    pub fn clear_block_protection(&mut self) -> Result<(), Infallible> {
        // Has to be written separately.
        self.write_enable()?;
        self.spi.write(&[WRSR, 0x00])
    }

    pub fn set_block_protection(&mut self) -> Result<(), Infallible> {
        let mut reg = StatusReg::new_with_raw_value(0);
        reg.set_block_protection_bits(u2::new(0b11));
        self.write_enable()?;
        self.spi.write(&[WRSR, reg.raw_value()])
    }

    fn common_init_write_and_read(&mut self, address: usize, reg: u8) -> Result<(), Infallible> {
        nb::block!(self.writes_are_done())?;
        self.spi.flush()?;
        if reg == WRITE {
            self.write_enable()?;
            self.spi.write_fifo_unchecked(WRITE as u32);
        } else {
            self.spi.write_fifo_unchecked(READ as u32);
        }
        self.spi.write_fifo_unchecked((address as u32 >> 16) & 0xff);
        self.spi
            .write_fifo_unchecked((address as u32 & 0x00ff00) >> 8);
        self.spi.write_fifo_unchecked(address as u32 & 0xff);
        Ok(())
    }

    fn common_read(&mut self, address: usize) -> Result<(), Infallible> {
        self.common_init_write_and_read(address, READ)?;
        for _ in 0..4 {
            // Pump the FIFO.
            self.spi.write_fifo_unchecked(0);
            // Ignore the first 4 bytes.
            nb::block!(self.spi.read_fifo())?;
        }
        Ok(())
    }

    pub fn write(&mut self, mut address: usize, mut data: &[u8]) -> Result<(), Infallible> {
        // Loop until all data is written
        while !data.is_empty() {
            // Calculate the page and the offset within the page from the address
            let page = address / PAGE_SIZE;
            let offset = address % PAGE_SIZE;

            // Calculate how much space is left in the current page
            let space_left = PAGE_SIZE - offset;

            // Determine how much data to write in the current page
            let to_write = data.len().min(space_left);

            // Write the current portion of the data
            self.write_page(page, offset, &data[..to_write]).unwrap();

            // Update the address and data for the next iteration
            address += to_write;
            data = &data[to_write..];
        }

        Ok(())
    }

    pub fn write_page(
        &mut self,
        page: usize,
        offset: usize,
        data: &[u8],
    ) -> Result<(), PageBoundaryExceededError> {
        // Check that the total data to be written does not exceed the page boundary
        if offset + data.len() > PAGE_SIZE {
            return Err(PageBoundaryExceededError);
        }

        self.common_init_write_and_read(page * PAGE_SIZE + offset, WRITE)
            .unwrap();
        for val in data.iter().take(data.len() - 1) {
            nb::block!(self.spi.write_fifo(*val as u32)).unwrap();
            nb::block!(self.spi.read_fifo()).unwrap();
        }
        nb::block!(self
            .spi
            .write_fifo(*data.last().unwrap() as u32 | BMSTART_BMSTOP_MASK))
        .unwrap();
        self.spi.flush().unwrap();
        nb::block!(self.writes_are_done()).unwrap();
        Ok(())
    }

    pub fn read(&mut self, address: usize, buf: &mut [u8]) -> Result<(), Infallible> {
        self.common_read(address)?;
        for val in buf.iter_mut() {
            self.spi.write_fifo_unchecked(0);
            *val = (nb::block!(self.spi.read_fifo()).unwrap() & 0xff) as u8;
        }
        nb::block!(self.spi.write_fifo(BMSTART_BMSTOP_MASK))?;
        self.spi.flush()?;
        Ok(())
    }

    pub fn verify(&mut self, address: usize, data: &[u8]) -> Result<bool, Infallible> {
        self.common_read(address)?;
        for val in data.iter() {
            self.spi.write_fifo_unchecked(0);
            let read_val = (nb::block!(self.spi.read_fifo()).unwrap() & 0xff) as u8;
            if read_val != *val {
                return Ok(false);
            }
        }
        nb::block!(self.spi.write_fifo(BMSTART_BMSTOP_MASK))?;
        self.spi.flush()?;
        Ok(true)
    }
}
