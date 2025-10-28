//! API for the DMA peripheral
//!
//! ## Examples
//!
//! - [Simple DMA example](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/dma.rs)
use arbitrary_int::{u10, u3};
use vorago_shared_hal::{enable_peripheral_clock, reset_peripheral_for_cycles, PeripheralSelect};

use crate::{enable_nvic_interrupt, pac};

const MAX_DMA_TRANSFERS_PER_CYCLE: usize = 1024;
const BASE_PTR_ADDR_MASK: u32 = 0b1111111;

/// DMA cycle control values.
///
/// Refer to chapter 6.3.1 and 6.6.3 of the datasheet for more details.
#[bitbybit::bitenum(u3, exhaustive = true)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CycleControl {
    /// Indicates that the data structure is invalid.
    Stop = 0b000,
    /// The controller must receive a new request prior to entering the arbitration
    /// process, to enable the DMA cycle to complete. This means that the DMA will only
    /// continue to do transfers as long as a trigger signal is still active. Therefore,
    /// this should not be used for momentary triggers like a timer.
    Basic = 0b001,
    /// The controller automatically inserts a request for the appropriate channel during the
    /// arbitration process. This means that the initial request is sufficient to enable the
    /// DMA cycle to complete.
    Auto = 0b010,
    /// This is used to support continuous data flow. Both primary and alternate data structure
    /// are used. The primary data structure is used first. When the first transfer is complete, an
    /// interrupt can be generated, and the DMA switches to the alternate data structure. When the
    /// second transfer is complete, the primary data structure is used. This pattern continues
    /// until software disables the channel.
    PingPong = 0b011,
    MemScatterGatherPrimary = 0b100,
    MemScatterGatherAlternate = 0b101,
    PeriphScatterGatherPrimary = 0b110,
    PeriphScatterGatherAlternate = 0b111,
}

#[bitbybit::bitenum(u2, exhaustive = true)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrIncrement {
    Byte = 0b00,
    Halfword = 0b01,
    Word = 0b10,
    None = 0b11,
}

#[bitbybit::bitenum(u2, exhaustive = false)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataSize {
    Byte = 0b00,
    Halfword = 0b01,
    Word = 0b10,
}

/// This configuration controls how many DMA transfers can occur before the controller arbitrates.
#[bitbybit::bitenum(u4, exhaustive = true)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RPower {
    EachTransfer = 0b0000,
    Every2 = 0b0001,
    Every4 = 0b0010,
    Every8 = 0b0011,
    Every16 = 0b0100,
    Every32 = 0b0101,
    Every64 = 0b0110,
    Every128 = 0b0111,
    Every256 = 0b1000,
    Every512 = 0b1001,
    Every1024 = 0b1010,
    Every1024Alt0 = 0b1011,
    Every1024Alt1 = 0b1100,
    Every1024Alt2 = 0b1101,
    Every1024Alt3 = 0b1110,
    Every1024Alt4 = 0b1111,
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Invalid DMA control block address")]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidCtrlBlockAddrError;

#[bitbybit::bitfield(u32, default = 0x0, debug, defmt_fields(feature = "defmt"))]
pub struct ChannelConfig {
    #[bits(30..=31, rw)]
    dst_inc: AddrIncrement,
    #[bits(28..=29, rw)]
    dst_size: Option<DataSize>,
    #[bits(26..=27, rw)]
    src_inc: AddrIncrement,
    #[bits(24..=25, rw)]
    src_size: Option<DataSize>,
    #[bits(21..=23, rw)]
    dest_prot_ctrl: u3,
    #[bits(18..=20, rw)]
    src_prot_ctrl: u3,
    #[bits(14..=17, rw)]
    r_power: RPower,
    #[bits(4..=13, rw)]
    n_minus_1: u10,
    #[bit(3, rw)]
    next_useburst: bool,
    #[bits(0..=2, rw)]
    cycle_ctrl: CycleControl,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DmaChannelControl {
    pub src_end_ptr: u32,
    pub dest_end_ptr: u32,
    pub cfg: ChannelConfig,
    padding: u32,
}

impl DmaChannelControl {
    const fn new() -> Self {
        Self {
            src_end_ptr: 0,
            dest_end_ptr: 0,
            cfg: ChannelConfig::new_with_raw_value(0),
            padding: 0,
        }
    }
}
impl Default for DmaChannelControl {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[repr(align(128))]
pub struct DmaCtrlBlock {
    pub pri: [DmaChannelControl; 4],
    pub alt: [DmaChannelControl; 4],
}

impl DmaCtrlBlock {
    pub const fn new() -> Self {
        Self {
            pri: [DmaChannelControl::new(); 4],
            alt: [DmaChannelControl::new(); 4],
        }
    }
}
impl Default for DmaCtrlBlock {
    fn default() -> Self {
        Self::new()
    }
}

impl DmaCtrlBlock {
    /// This function creates a DMA control block at the specified memory address.
    ///
    /// The passed address must be 128-byte aligned. The user must also take care of specifying
    /// a valid memory address for the DMA control block which is accessible by the system as well.
    /// For example, the control block can be placed in the SRAM1.
    pub fn new_at_addr(addr: u32) -> Result<*mut DmaCtrlBlock, InvalidCtrlBlockAddrError> {
        if addr & BASE_PTR_ADDR_MASK > 0 {
            return Err(InvalidCtrlBlockAddrError);
        }
        let ctrl_block_ptr = addr as *mut DmaCtrlBlock;
        unsafe { core::ptr::write(ctrl_block_ptr, DmaCtrlBlock::default()) }
        Ok(ctrl_block_ptr)
    }
}

pub struct Dma {
    dma: pac::Dma,
    ctrl_block: *mut DmaCtrlBlock,
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaTransferInitError {
    #[error("source and destination buffer length mismatch: {src_len} != {dest_len}")]
    SourceDestLenMissmatch { src_len: usize, dest_len: usize },
    /// Overflow when calculating the source or destination end address.
    #[error("address overflow")]
    AddrOverflow,
    /// Transfer size larger than 1024 units.
    #[error("transfer size too large: {0}, 1024 is the allowed maximum")]
    TransferSizeTooLarge(usize),
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DmaConfig {
    pub bufferable: bool,
    pub cacheable: bool,
    pub privileged: bool,
}

pub struct DmaChannel {
    channel: u8,
    done_interrupt: pac::Interrupt,
    active_interrupt: pac::Interrupt,
    pub dma: pac::Dma,
    pub ch_ctrl_pri: &'static mut DmaChannelControl,
    pub ch_ctrl_alt: &'static mut DmaChannelControl,
}

impl DmaChannel {
    #[inline(always)]
    pub fn channel(&self) -> u8 {
        self.channel
    }

    #[inline(always)]
    pub fn enable(&mut self) {
        self.dma
            .chnl_enable_set()
            .write(|w| unsafe { w.bits(1 << self.channel) });
    }

    #[inline(always)]
    pub fn is_enabled(&mut self) -> bool {
        ((self.dma.chnl_enable_set().read().bits() >> self.channel) & 0b1) != 0
    }

    #[inline(always)]
    pub fn disable(&mut self) {
        self.dma
            .chnl_enable_clr()
            .write(|w| unsafe { w.bits(1 << self.channel) });
    }

    #[inline(always)]
    pub fn trigger_with_sw_request(&mut self) {
        self.dma
            .chnl_sw_request()
            .write(|w| unsafe { w.bits(1 << self.channel) });
    }

    #[inline(always)]
    pub fn state_raw(&self) -> u8 {
        self.dma.status().read().state().bits()
    }

    #[inline(always)]
    pub fn select_primary_structure(&self) {
        self.dma
            .chnl_pri_alt_clr()
            .write(|w| unsafe { w.bits(1 << self.channel) });
    }

    #[inline(always)]
    pub fn select_alternate_structure(&self) {
        self.dma
            .chnl_pri_alt_set()
            .write(|w| unsafe { w.bits(1 << self.channel) });
    }

    /// Enables the DMA_DONE interrupt for the DMA channel.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it can break mask-based critical sections.
    pub unsafe fn enable_done_interrupt(&mut self) {
        enable_nvic_interrupt(self.done_interrupt);
    }

    /// Enables the DMA_ACTIVE interrupt for the DMA channel.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it can break mask-based critical sections.
    pub unsafe fn enable_active_interrupt(&mut self) {
        enable_nvic_interrupt(self.active_interrupt);
    }

    /// Prepares a 8-bit DMA transfer from memory to memory.
    ///
    /// This function does not enable the DMA channel and interrupts and only prepares
    /// the DMA control block parameters for the transfer. It configures the primary channel control
    /// structure to perform the transfer.
    ///
    /// You can use [Self::enable], [Self::enable_done_interrupt], [Self::enable_active_interrupt]
    /// to finish the transfer preparation and then use [Self::trigger_with_sw_request] to
    /// start the DMA transfer.
    ///
    /// # Safety
    ///
    /// You must ensure that the destination buffer is safe for DMA writes and the source buffer
    /// is safe for DMA reads. The specific requirements can be read here:
    ///
    ///  - [DMA source buffer](https://docs.rs/embedded-dma/latest/embedded_dma/trait.ReadBuffer.html)
    ///  - [DMA destination buffer](https://docs.rs/embedded-dma/latest/embedded_dma/trait.WriteBuffer.html)
    ///
    /// More specifically, you must ensure that the passed slice remains valid while the DMA is
    /// active or until the DMA is stopped.
    pub unsafe fn prepare_mem_to_mem_transfer_8_bit(
        &mut self,
        source: &[u8],
        dest: &mut [u8],
    ) -> Result<(), DmaTransferInitError> {
        let len = Self::common_mem_transfer_checks(source.len(), dest.len())?;
        self.generic_mem_to_mem_transfer_init(
            len,
            (source.as_ptr() as u32)
                .checked_add(len as u32)
                .ok_or(DmaTransferInitError::AddrOverflow)?,
            (dest.as_ptr() as u32)
                .checked_add(len as u32)
                .ok_or(DmaTransferInitError::AddrOverflow)?,
            DataSize::Byte,
            AddrIncrement::Byte,
        );
        Ok(())
    }

    /// Prepares a 16-bit DMA transfer from memory to memory.
    ///
    /// This function does not enable the DMA channel and interrupts and only prepares
    /// the DMA control block parameters for the transfer. It configures the primary channel control
    /// structure to perform the transfer.
    ///
    /// You can use [Self::enable], [Self::enable_done_interrupt], [Self::enable_active_interrupt]
    /// to finish the transfer preparation and then use [Self::trigger_with_sw_request] to
    /// start the DMA transfer.
    ///
    /// # Safety
    ///
    /// You must ensure that the destination buffer is safe for DMA writes and the source buffer
    /// is safe for DMA reads. The specific requirements can be read here:
    ///
    ///  - [DMA source buffer](https://docs.rs/embedded-dma/latest/embedded_dma/trait.ReadBuffer.html)
    ///  - [DMA destination buffer](https://docs.rs/embedded-dma/latest/embedded_dma/trait.WriteBuffer.html)
    ///
    /// More specifically, you must ensure that the passed slice remains valid while the DMA is
    /// active or until the DMA is stopped.
    pub unsafe fn prepare_mem_to_mem_transfer_16_bit(
        &mut self,
        source: &[u16],
        dest: &mut [u16],
    ) -> Result<(), DmaTransferInitError> {
        let len = Self::common_mem_transfer_checks(source.len(), dest.len())?;
        self.generic_mem_to_mem_transfer_init(
            len,
            (source.as_ptr() as u32)
                .checked_add(len as u32 * core::mem::size_of::<u16>() as u32)
                .ok_or(DmaTransferInitError::AddrOverflow)?,
            (dest.as_ptr() as u32)
                .checked_add(len as u32 * core::mem::size_of::<u16>() as u32)
                .ok_or(DmaTransferInitError::AddrOverflow)?,
            DataSize::Halfword,
            AddrIncrement::Halfword,
        );
        Ok(())
    }

    /// Prepares a 32-bit DMA transfer from memory to memory.
    ///
    /// This function does not enable the DMA channel and interrupts and only prepares
    /// the DMA control block parameters for the transfer. It configures the primary channel control
    /// structure to perform the transfer.
    ///
    /// You can use [Self::enable], [Self::enable_done_interrupt], [Self::enable_active_interrupt]
    /// to finish the transfer preparation and then use [Self::trigger_with_sw_request] to
    /// start the DMA transfer.
    ///
    /// # Safety
    ///
    /// You must ensure that the destination buffer is safe for DMA writes and the source buffer
    /// is safe for DMA reads. The specific requirements can be read here:
    ///
    ///  - [DMA source buffer](https://docs.rs/embedded-dma/latest/embedded_dma/trait.ReadBuffer.html)
    ///  - [DMA destination buffer](https://docs.rs/embedded-dma/latest/embedded_dma/trait.WriteBuffer.html)
    ///
    /// More specifically, you must ensure that the passed slice remains valid while the DMA is
    /// active or until the DMA is stopped.
    pub unsafe fn prepare_mem_to_mem_transfer_32_bit(
        &mut self,
        source: &[u32],
        dest: &mut [u32],
    ) -> Result<(), DmaTransferInitError> {
        let len = Self::common_mem_transfer_checks(source.len(), dest.len())?;
        self.generic_mem_to_mem_transfer_init(
            len,
            (source.as_ptr() as u32)
                .checked_add(len as u32 * core::mem::size_of::<u32>() as u32)
                .ok_or(DmaTransferInitError::AddrOverflow)?,
            (dest.as_ptr() as u32)
                .checked_add(len as u32 * core::mem::size_of::<u32>() as u32)
                .ok_or(DmaTransferInitError::AddrOverflow)?,
            DataSize::Word,
            AddrIncrement::Word,
        );
        Ok(())
    }

    /// Prepares a 8-bit DMA transfer from memory to a peripheral.
    ///
    /// It is assumed that a peripheral with a 16-byte FIFO is used here and that the
    /// transfer is activated by an IRQ trigger when the half-full interrupt of the peripheral
    /// is fired. Therefore, this function configured the DMA in [CycleControl::Basic] mode with
    /// rearbitration happening every 8 DMA cycles. It also configures the primary channel control
    /// structure to perform the transfer.
    ///
    /// # Safety
    ///
    /// You must ensure that the source buffer is safe for DMA reads. The specific requirements
    /// can be read here:
    ///
    ///  - [DMA source buffer](https://docs.rs/embedded-dma/latest/embedded_dma/trait.ReadBuffer.html)
    ///
    /// More specifically, you must ensure that the passed slice remains valid while the DMA is
    /// active or until the DMA is stopped.
    ///
    /// The destination address must be the pointer address of a peripheral FIFO register address.
    /// You must also ensure that the regular synchronous transfer API of the peripheral is NOT
    /// used to perform transfers.
    pub unsafe fn prepare_mem_to_periph_transfer_8_bit(
        &mut self,
        source: &[u8],
        dest: *mut u32,
    ) -> Result<(), DmaTransferInitError> {
        if source.len() > MAX_DMA_TRANSFERS_PER_CYCLE {
            return Err(DmaTransferInitError::TransferSizeTooLarge(source.len()));
        }
        let len = source.len() - 1;
        self.ch_ctrl_pri.cfg = ChannelConfig::new_with_raw_value(0);
        self.ch_ctrl_pri.src_end_ptr = (source.as_ptr() as u32)
            .checked_add(len as u32)
            .ok_or(DmaTransferInitError::AddrOverflow)?;
        self.ch_ctrl_pri.dest_end_ptr = dest as u32;
        self.ch_ctrl_pri.cfg.set_cycle_ctrl(CycleControl::Basic);
        self.ch_ctrl_pri.cfg.set_src_size(DataSize::Byte);
        self.ch_ctrl_pri.cfg.set_src_inc(AddrIncrement::Byte);
        self.ch_ctrl_pri.cfg.set_dst_size(DataSize::Byte);
        self.ch_ctrl_pri.cfg.set_dst_inc(AddrIncrement::None);
        self.ch_ctrl_pri.cfg.set_n_minus_1(u10::new(len as u16));
        self.ch_ctrl_pri.cfg.set_r_power(RPower::Every8);
        self.select_primary_structure();
        Ok(())
    }

    // This function performs common checks and returns the source length minus one which is
    // relevant for further configuration of the DMA. This is because the DMA API expects N minus
    // 1 and the source and end pointer need to point to the last transfer address.
    fn common_mem_transfer_checks(
        src_len: usize,
        dest_len: usize,
    ) -> Result<usize, DmaTransferInitError> {
        if src_len != dest_len {
            return Err(DmaTransferInitError::SourceDestLenMissmatch { src_len, dest_len });
        }
        if src_len > MAX_DMA_TRANSFERS_PER_CYCLE {
            return Err(DmaTransferInitError::TransferSizeTooLarge(src_len));
        }
        Ok(src_len - 1)
    }

    fn generic_mem_to_mem_transfer_init(
        &mut self,
        n_minus_one: usize,
        src_end_ptr: u32,
        dest_end_ptr: u32,
        data_size: DataSize,
        addr_incr: AddrIncrement,
    ) {
        self.ch_ctrl_pri.cfg = ChannelConfig::new_with_raw_value(0);
        self.ch_ctrl_pri.src_end_ptr = src_end_ptr;
        self.ch_ctrl_pri.dest_end_ptr = dest_end_ptr;
        self.ch_ctrl_pri.cfg.set_cycle_ctrl(CycleControl::Auto);
        self.ch_ctrl_pri.cfg.set_src_size(data_size);
        self.ch_ctrl_pri.cfg.set_src_inc(addr_incr);
        self.ch_ctrl_pri.cfg.set_dst_size(data_size);
        self.ch_ctrl_pri.cfg.set_dst_inc(addr_incr);
        self.ch_ctrl_pri
            .cfg
            .set_n_minus_1(u10::new(n_minus_one as u16));
        self.ch_ctrl_pri.cfg.set_r_power(RPower::Every4);
        self.select_primary_structure();
    }
}

impl Dma {
    /// Create a new DMA instance.
    ///
    /// You can also place the [DmaCtrlBlock] statically using a global static mutable
    /// instance and the [DmaCtrlBlock::new] const constructor This also allows to place the control
    /// block in a memory section using the [link_section](https://doc.rust-lang.org/reference/abi.html#the-link_section-attribute)
    /// attribute and then creating a mutable pointer to it using [core::ptr::addr_of_mut].
    ///
    /// Alternatively, the [DmaCtrlBlock::new_at_addr] function can be used to create the DMA
    /// control block at a specific address.
    pub fn new(
        dma: pac::Dma,
        cfg: DmaConfig,
        ctrl_block: *mut DmaCtrlBlock,
    ) -> Result<Self, InvalidCtrlBlockAddrError> {
        // The conversion to u32 is safe here because we are on a 32-bit system.
        let raw_addr = ctrl_block as u32;
        if raw_addr & BASE_PTR_ADDR_MASK > 0 {
            return Err(InvalidCtrlBlockAddrError);
        }
        enable_peripheral_clock(PeripheralSelect::Dma);
        reset_peripheral_for_cycles(PeripheralSelect::Dma, 2);
        let dma = Dma { dma, ctrl_block };
        dma.dma
            .ctrl_base_ptr()
            .write(|w| unsafe { w.bits(raw_addr) });
        dma.set_protection_bits(&cfg);
        dma.enable();
        Ok(dma)
    }

    #[inline(always)]
    pub fn enable(&self) {
        self.dma.cfg().write(|w| w.master_enable().set_bit());
    }

    #[inline(always)]
    pub fn disable(&self) {
        self.dma.cfg().write(|w| w.master_enable().clear_bit());
    }

    #[inline(always)]
    pub fn set_protection_bits(&self, cfg: &DmaConfig) {
        self.dma.cfg().write(|w| unsafe {
            w.chnl_prot_ctrl().bits(
                cfg.privileged as u8 | ((cfg.bufferable as u8) << 1) | ((cfg.cacheable as u8) << 2),
            )
        });
    }

    /// Split the DMA instance into four DMA channels which can be used individually. This allows
    /// using the inidividual DMA channels in separate tasks.
    pub fn split(self) -> (DmaChannel, DmaChannel, DmaChannel, DmaChannel) {
        // Safety: The DMA channel API only operates on its respective channels.
        (
            DmaChannel {
                channel: 0,
                done_interrupt: pac::Interrupt::DMA_DONE0,
                active_interrupt: pac::Interrupt::DMA_ACTIVE0,
                dma: unsafe { pac::Dma::steal() },
                ch_ctrl_pri: unsafe { &mut (*self.ctrl_block).pri[0] },
                ch_ctrl_alt: unsafe { &mut (*self.ctrl_block).alt[0] },
            },
            DmaChannel {
                channel: 1,
                done_interrupt: pac::Interrupt::DMA_DONE1,
                active_interrupt: pac::Interrupt::DMA_ACTIVE1,
                dma: unsafe { pac::Dma::steal() },
                ch_ctrl_pri: unsafe { &mut (*self.ctrl_block).pri[1] },
                ch_ctrl_alt: unsafe { &mut (*self.ctrl_block).alt[1] },
            },
            DmaChannel {
                channel: 2,
                done_interrupt: pac::Interrupt::DMA_DONE2,
                active_interrupt: pac::Interrupt::DMA_ACTIVE2,
                dma: unsafe { pac::Dma::steal() },
                ch_ctrl_pri: unsafe { &mut (*self.ctrl_block).pri[2] },
                ch_ctrl_alt: unsafe { &mut (*self.ctrl_block).alt[2] },
            },
            DmaChannel {
                channel: 3,
                done_interrupt: pac::Interrupt::DMA_DONE3,
                active_interrupt: pac::Interrupt::DMA_ACTIVE3,
                dma: unsafe { pac::Dma::steal() },
                ch_ctrl_pri: unsafe { &mut (*self.ctrl_block).pri[3] },
                ch_ctrl_alt: unsafe { &mut (*self.ctrl_block).alt[3] },
            },
        )
    }
}
