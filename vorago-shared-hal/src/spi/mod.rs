use crate::FunctionSelect;
use crate::gpio::{DynPinId, IoPeriphPin};
use crate::{PeripheralSelect, enable_peripheral_clock, pins::AnyPin, sealed::Sealed, time::Hertz};
use core::{convert::Infallible, fmt::Debug, marker::PhantomData};
use embedded_hal::spi::{MODE_0, Mode};

use regs::{ClockPrescaler, Data, FifoClear, WordSize};
#[cfg(feature = "vor1x")]
use va108xx as pac;
#[cfg(feature = "vor4x")]
use va416xx as pac;

pub use regs::{Bank, HwChipSelectId};

pub mod regs;

pub fn configure_pin_as_hw_cs_pin<P: AnyPin + HwCsProvider>(_pin: P) -> HwChipSelectId {
    IoPeriphPin::new(P::ID, P::FUN_SEL, None);
    P::CS_ID
}

//==================================================================================================
// Pins and traits.
//==================================================================================================

pub trait PinSck: AnyPin {
    const SPI_ID: Bank;
    const FUN_SEL: FunctionSelect;
}

pub trait PinMosi: AnyPin {
    const SPI_ID: Bank;
    const FUN_SEL: FunctionSelect;
}

pub trait PinMiso: AnyPin {
    const SPI_ID: Bank;
    const FUN_SEL: FunctionSelect;
}

pub trait HwCsProvider {
    const PIN_ID: DynPinId;
    const SPI_ID: Bank;
    const FUN_SEL: FunctionSelect;
    const CS_ID: HwChipSelectId;
}

#[macro_use]
mod macros {
    #[cfg(not(feature = "va41628"))]
    macro_rules! hw_cs_multi_pin {
    (
        // name of the newtype wrapper struct
        $name:ident,
        // Pb0
        $pin_id:ident,
        // SpiId::B
        $spi_id:path,
        // FunSel::Sel1
        $fun_sel:path,
        // HwChipSelectId::Id2
        $cs_id:path
    ) => {
            #[doc = concat!(
                "Newtype wrapper to use [Pin] [`", stringify!($pin_id), "`] as a HW CS pin for [`", stringify!($spi_id), "`] with [`", stringify!($cs_id), "`]."
            )]
            pub struct $name(Pin<$pin_id>);

            impl $name {
                pub fn new(pin: Pin<$pin_id>) -> Self {
                    Self(pin)
                }
            }

            impl crate::sealed::Sealed for $name {}

            impl HwCsProvider for $name {
                const PIN_ID: DynPinId = <$pin_id as PinId>::ID;
                const SPI_ID: Bank = $spi_id;
                const FUN_SEL: FunctionSelect = $fun_sel;
                const CS_ID: HwChipSelectId = $cs_id;
            }
        };
    }

    #[macro_export]
    macro_rules! hw_cs_pins {
        ($SpiId:path, $(($Px:ident, $FunSel:path, $HwCsIdent:path)$(,)?)+) => {
            $(
                impl HwCsProvider for Pin<$Px> {
                    const PIN_ID: DynPinId = $Px::ID;
                    const SPI_ID: Bank = $SpiId;
                    const FUN_SEL: FunctionSelect = $FunSel;
                    const CS_ID: HwChipSelectId = $HwCsIdent;
                }
            )+
        };
    }
}

#[cfg(feature = "vor1x")]
pub mod pins_vor1x;
#[cfg(feature = "vor4x")]
pub mod pins_vor4x;

//==================================================================================================
// Defintions
//==================================================================================================

// FIFO has a depth of 16.
const FILL_DEPTH: usize = 12;

pub const BMSTART_BMSTOP_MASK: u32 = 1 << 31;
pub const BMSKIPDATA_MASK: u32 = 1 << 30;

pub const DEFAULT_CLK_DIV: u16 = 2;

/// Common trait implemented by all PAC peripheral access structures. The register block
/// format is the same for all SPI blocks.
pub trait SpiInstance: Sealed {
    const ID: Bank;
    const PERIPH_SEL: PeripheralSelect;
}

#[cfg(feature = "vor1x")]
pub type Spi0 = pac::Spia;
#[cfg(feature = "vor4x")]
pub type Spi0 = pac::Spi0;

impl SpiInstance for Spi0 {
    const ID: Bank = Bank::Spi0;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Spi0;
}
impl Sealed for Spi0 {}

#[cfg(feature = "vor1x")]
pub type Spi1 = pac::Spib;
#[cfg(feature = "vor4x")]
pub type Spi1 = pac::Spi1;

impl SpiInstance for Spi1 {
    const ID: Bank = Bank::Spi1;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Spi1;
}
impl Sealed for Spi1 {}

#[cfg(feature = "vor1x")]
pub type Spi2 = pac::Spic;
#[cfg(feature = "vor4x")]
pub type Spi2 = pac::Spi2;

impl SpiInstance for Spi2 {
    const ID: Bank = Bank::Spi2;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Spi2;
}
impl Sealed for Spi2 {}

#[cfg(feature = "vor4x")]
impl SpiInstance for pac::Spi3 {
    const ID: Bank = Bank::Spi3;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Spi3;
}
#[cfg(feature = "vor4x")]
impl Sealed for pac::Spi3 {}

//==================================================================================================
// Config
//==================================================================================================

pub trait TransferConfigProvider {
    fn sod(&mut self, sod: bool);
    fn blockmode(&mut self, blockmode: bool);
    fn mode(&mut self, mode: Mode);
    fn clk_cfg(&mut self, clk_cfg: SpiClockConfig);
    fn hw_cs_id(&self) -> u8;
}

/// Type erased variant of the transfer configuration. This is required to avoid generics in
/// the SPI constructor.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TransferConfig {
    pub clk_cfg: Option<SpiClockConfig>,
    pub mode: Option<Mode>,
    pub sod: bool,
    /// If this is enabled, all data in the FIFO is transmitted in a single frame unless
    /// the BMSTOP bit is set on a dataword. A frame is defined as CSn being active for the
    /// duration of multiple data words
    pub blockmode: bool,
    /// Only used when blockmode is used. The SCK will be stalled until an explicit stop bit
    /// is set on a written word.
    pub bmstall: bool,
    pub hw_cs: Option<HwChipSelectId>,
}

impl TransferConfig {
    pub fn new_with_hw_cs(
        clk_cfg: Option<SpiClockConfig>,
        mode: Option<Mode>,
        blockmode: bool,
        bmstall: bool,
        sod: bool,
        hw_cs_id: HwChipSelectId,
    ) -> Self {
        TransferConfig {
            clk_cfg,
            mode,
            sod,
            blockmode,
            bmstall,
            hw_cs: Some(hw_cs_id),
        }
    }
}

/// Configuration options for the whole SPI bus. See Programmer Guide p.92 for more details
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SpiConfig {
    clk: SpiClockConfig,
    // SPI mode configuration
    pub init_mode: Mode,
    /// If this is enabled, all data in the FIFO is transmitted in a single frame unless
    /// the BMSTOP bit is set on a dataword. A frame is defined as CSn being active for the
    /// duration of multiple data words. Defaults to true.
    pub blockmode: bool,
    /// This enables the stalling of the SPI SCK if in blockmode and the FIFO is empty.
    /// Currently enabled by default.
    pub bmstall: bool,
    /// Slave output disable. Useful if separate GPIO pins or decoders are used for CS control
    pub slave_output_disable: bool,
    /// Loopback mode. If you use this, don't connect MISO to MOSI, they will be tied internally
    pub loopback_mode: bool,
    /// Enable Master Delayer Capture Mode. See Programmers Guide p.92 for more details
    pub master_delayer_capture: bool,
}

impl Default for SpiConfig {
    fn default() -> Self {
        Self {
            init_mode: MODE_0,
            blockmode: true,
            bmstall: true,
            // Default value is definitely valid.
            clk: SpiClockConfig::from_div(DEFAULT_CLK_DIV).unwrap(),
            slave_output_disable: Default::default(),
            loopback_mode: Default::default(),
            master_delayer_capture: Default::default(),
        }
    }
}

impl SpiConfig {
    pub fn loopback(mut self, enable: bool) -> Self {
        self.loopback_mode = enable;
        self
    }

    pub fn blockmode(mut self, enable: bool) -> Self {
        self.blockmode = enable;
        self
    }

    pub fn bmstall(mut self, enable: bool) -> Self {
        self.bmstall = enable;
        self
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.init_mode = mode;
        self
    }

    pub fn clk_cfg(mut self, clk_cfg: SpiClockConfig) -> Self {
        self.clk = clk_cfg;
        self
    }

    pub fn slave_output_disable(mut self, sod: bool) -> Self {
        self.slave_output_disable = sod;
        self
    }
}

//==================================================================================================
// Word Size
//==================================================================================================

/// Configuration trait for the Word Size
/// used by the SPI peripheral
pub trait SpiWord: Copy + Default + Into<u32> + TryFrom<u32> + 'static {
    const MASK: u32;
    const WORD_SIZE: regs::WordSize;
    fn word_reg() -> u8;
}

impl SpiWord for u8 {
    const MASK: u32 = 0xff;
    const WORD_SIZE: regs::WordSize = regs::WordSize::EightBits;
    fn word_reg() -> u8 {
        0x07
    }
}

impl SpiWord for u16 {
    const MASK: u32 = 0xffff;
    const WORD_SIZE: regs::WordSize = regs::WordSize::SixteenBits;
    fn word_reg() -> u8 {
        0x0f
    }
}

//==================================================================================================
// Spi
//==================================================================================================

/// Low level access trait for the SPI peripheral.
pub trait SpiLowLevel {
    /// Low level function to write a word to the SPI FIFO but also checks whether
    /// there is actually data in the FIFO.
    ///
    /// Uses the [nb] API to allow usage in blocking and non-blocking contexts.
    fn write_fifo(&mut self, data: u32) -> nb::Result<(), Infallible>;

    /// Low level function to write a word to the SPI FIFO without checking whether
    /// there FIFO is full.
    ///
    /// This does not necesarily mean there is a space in the FIFO available.
    /// Use [Self::write_fifo] function to write a word into the FIFO reliably.
    fn write_fifo_unchecked(&mut self, data: u32);

    /// Low level function to read a word from the SPI FIFO. Must be preceeded by a
    /// [Self::write_fifo] call.
    ///
    /// Uses the [nb] API to allow usage in blocking and non-blocking contexts.
    fn read_fifo(&mut self) -> nb::Result<u32, Infallible>;

    /// Low level function to read a word from from the SPI FIFO.
    ///
    /// This does not necesarily mean there is a word in the FIFO available.
    /// Use the [Self::read_fifo] function to read a word from the FIFO reliably using the [nb]
    /// API.
    /// You might also need to mask the value to ignore the BMSTART/BMSTOP bit.
    fn read_fifo_unchecked(&mut self) -> u32;
}

#[inline(always)]
pub fn mode_to_cpo_cph_bit(mode: embedded_hal::spi::Mode) -> (bool, bool) {
    match mode {
        embedded_hal::spi::MODE_0 => (false, false),
        embedded_hal::spi::MODE_1 => (false, true),
        embedded_hal::spi::MODE_2 => (true, false),
        embedded_hal::spi::MODE_3 => (true, true),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SpiClockConfig {
    prescale_val: u8,
    scrdv: u8,
}

impl SpiClockConfig {
    pub fn prescale_val(&self) -> u8 {
        self.prescale_val
    }
    pub fn scrdv(&self) -> u8 {
        self.scrdv
    }
}

impl SpiClockConfig {
    pub fn new(prescale_val: u8, scrdv: u8) -> Self {
        Self {
            prescale_val,
            scrdv,
        }
    }

    pub fn from_div(div: u16) -> Result<Self, SpiClockConfigError> {
        spi_clk_config_from_div(div)
    }

    #[cfg(feature = "vor1x")]
    pub fn from_clk(sys_clk: Hertz, spi_clk: Hertz) -> Option<Self> {
        clk_div_for_target_clock(sys_clk, spi_clk).map(|div| spi_clk_config_from_div(div).unwrap())
    }

    #[cfg(feature = "vor4x")]
    pub fn from_clks(clks: &crate::clock::Clocks, spi_clk: Hertz) -> Option<Self> {
        Self::from_apb1_clk(clks.apb1(), spi_clk)
    }

    #[cfg(feature = "vor4x")]
    pub fn from_apb1_clk(apb1_clk: Hertz, spi_clk: Hertz) -> Option<Self> {
        clk_div_for_target_clock(apb1_clk, spi_clk).map(|div| spi_clk_config_from_div(div).unwrap())
    }
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiClockConfigError {
    #[error("division by zero")]
    DivIsZero,
    #[error("divide value is not even")]
    DivideValueNotEven,
    #[error("scrdv value is too large")]
    ScrdvValueTooLarge,
}

#[inline]
pub fn spi_clk_config_from_div(mut div: u16) -> Result<SpiClockConfig, SpiClockConfigError> {
    if div == 0 {
        return Err(SpiClockConfigError::DivIsZero);
    }
    if div % 2 != 0 {
        return Err(SpiClockConfigError::DivideValueNotEven);
    }
    let mut prescale_val = 0;

    // find largest (even) prescale value that divides into div
    for i in (2..=0xfe).rev().step_by(2) {
        if div % i == 0 {
            prescale_val = i;
            break;
        }
    }

    if prescale_val == 0 {
        return Err(SpiClockConfigError::DivideValueNotEven);
    }

    div /= prescale_val;
    if div > u8::MAX as u16 + 1 {
        return Err(SpiClockConfigError::ScrdvValueTooLarge);
    }
    Ok(SpiClockConfig {
        prescale_val: prescale_val as u8,
        scrdv: (div - 1) as u8,
    })
}

#[inline]
pub fn clk_div_for_target_clock(sys_clk: Hertz, spi_clk: Hertz) -> Option<u16> {
    if spi_clk > sys_clk {
        return None;
    }

    // Step 1: Calculate raw divider.
    let raw_div = sys_clk.raw() / spi_clk.raw();
    let remainder = sys_clk.raw() % spi_clk.raw();

    // Step 2: Round up if necessary.
    let mut rounded_div = if remainder * 2 >= spi_clk.raw() {
        raw_div + 1
    } else {
        raw_div
    };

    if rounded_div % 2 != 0 {
        // Take slower clock conservatively.
        rounded_div += 1;
    }
    if rounded_div > u16::MAX as u32 {
        return None;
    }
    Some(rounded_div as u16)
}

#[derive(Debug, thiserror::Error)]
#[error("peripheral or peripheral pin ID is not consistent")]
pub struct SpiIdMissmatchError;

/// SPI peripheral driver structure.
pub struct Spi<Word = u8> {
    id: Bank,
    regs: regs::MmioSpi<'static>,
    cfg: SpiConfig,
    /// Fill word for read-only SPI transactions.
    fill_word: Word,
    blockmode: bool,
    bmstall: bool,
    word: PhantomData<Word>,
}

impl<Word: SpiWord> Spi<Word>
where
    <Word as TryFrom<u32>>::Error: core::fmt::Debug,
{
    /// Create a new SPI struct for using SPI with the fixed ROM SPI pins.
    ///
    /// ## Arguments
    ///
    /// * `spi` - SPI bus to use
    /// * `spi_cfg` - Configuration specific to the SPI bus
    pub fn new_for_rom<SpiI: SpiInstance>(
        spi: SpiI,
        spi_cfg: SpiConfig,
    ) -> Result<Self, SpiIdMissmatchError> {
        #[cfg(feature = "vor1x")]
        if SpiI::ID != Bank::Spi2 {
            return Err(SpiIdMissmatchError);
        }
        #[cfg(feature = "vor4x")]
        if SpiI::ID != Bank::Spi3 {
            return Err(SpiIdMissmatchError);
        }
        Ok(Self::new_generic(spi, spi_cfg))
    }

    /// Create a new SPI peripheral driver.
    ///
    /// ## Arguments
    ///
    /// * `spi` - SPI bus to use
    /// * `pins` - Pins to be used for SPI transactions. These pins are consumed
    ///   to ensure the pins can not be used for other purposes anymore
    /// * `spi_cfg` - Configuration specific to the SPI bus
    pub fn new<SpiI: SpiInstance, Sck: PinSck, Miso: PinMiso, Mosi: PinMosi>(
        spi: SpiI,
        _pins: (Sck, Miso, Mosi),
        spi_cfg: SpiConfig,
    ) -> Result<Self, SpiIdMissmatchError> {
        if SpiI::ID != Sck::SPI_ID || SpiI::ID != Miso::SPI_ID || SpiI::ID != Mosi::SPI_ID {
            return Err(SpiIdMissmatchError);
        }
        IoPeriphPin::new(Sck::ID, Sck::FUN_SEL, None);
        IoPeriphPin::new(Miso::ID, Miso::FUN_SEL, None);
        IoPeriphPin::new(Mosi::ID, Mosi::FUN_SEL, None);
        Ok(Self::new_generic(spi, spi_cfg))
    }

    pub fn new_generic<SpiI: SpiInstance>(_spi: SpiI, spi_cfg: SpiConfig) -> Self {
        enable_peripheral_clock(SpiI::PERIPH_SEL);
        let mut regs = regs::Spi::new_mmio(SpiI::ID);
        let (cpo_bit, cph_bit) = mode_to_cpo_cph_bit(spi_cfg.init_mode);
        regs.write_ctrl0(
            regs::Control0::builder()
                .with_scrdv(spi_cfg.clk.scrdv)
                .with_sph(cph_bit)
                .with_spo(cpo_bit)
                .with_word_size(Word::WORD_SIZE)
                .build(),
        );
        regs.write_ctrl1(
            regs::Control1::builder()
                .with_mtxpause(false)
                .with_mdlycap(spi_cfg.master_delayer_capture)
                .with_bm_stall(spi_cfg.bmstall)
                .with_bm_start(false)
                .with_blockmode(spi_cfg.blockmode)
                .with_ss(HwChipSelectId::Id0)
                .with_sod(spi_cfg.slave_output_disable)
                .with_slave_mode(false)
                .with_enable(false)
                .with_lbm(spi_cfg.loopback_mode)
                .build(),
        );
        regs.write_clkprescale(ClockPrescaler::new(spi_cfg.clk.prescale_val));
        regs.write_fifo_clear(
            FifoClear::builder()
                .with_tx_fifo(true)
                .with_rx_fifo(true)
                .build(),
        );
        // Enable the peripheral as the last step as recommended in the
        // programmers guide
        regs.modify_ctrl1(|mut value| {
            value.set_enable(true);
            value
        });
        Spi {
            id: SpiI::ID,
            regs: regs::Spi::new_mmio(SpiI::ID),
            cfg: spi_cfg,
            fill_word: Default::default(),
            bmstall: spi_cfg.bmstall,
            blockmode: spi_cfg.blockmode,
            word: PhantomData,
        }
    }

    #[inline]
    pub fn cfg_clock(&mut self, cfg: SpiClockConfig) {
        self.regs.modify_ctrl0(|mut value| {
            value.set_scrdv(cfg.scrdv);
            value
        });
        self.regs
            .write_clkprescale(regs::ClockPrescaler::new(cfg.prescale_val));
    }

    pub fn set_fill_word(&mut self, fill_word: Word) {
        self.fill_word = fill_word;
    }

    #[inline]
    pub fn cfg_clock_from_div(&mut self, div: u16) -> Result<(), SpiClockConfigError> {
        let val = spi_clk_config_from_div(div)?;
        self.cfg_clock(val);
        Ok(())
    }

    #[inline]
    pub fn cfg_mode(&mut self, mode: Mode) {
        let (cpo_bit, cph_bit) = mode_to_cpo_cph_bit(mode);
        self.regs.modify_ctrl0(|mut value| {
            value.set_spo(cpo_bit);
            value.set_sph(cph_bit);
            value
        });
    }

    #[inline]
    pub fn fill_word(&self) -> Word {
        self.fill_word
    }

    #[inline]
    pub fn clear_tx_fifo(&mut self) {
        self.regs.write_fifo_clear(
            regs::FifoClear::builder()
                .with_tx_fifo(true)
                .with_rx_fifo(false)
                .build(),
        );
    }

    #[inline]
    pub fn clear_rx_fifo(&mut self) {
        self.regs.write_fifo_clear(
            regs::FifoClear::builder()
                .with_tx_fifo(false)
                .with_rx_fifo(true)
                .build(),
        );
    }

    #[inline]
    pub fn perid(&self) -> u32 {
        self.regs.read_perid()
    }

    /// Configure the hardware chip select given a hardware chip select ID.
    ///
    /// The pin also needs to be configured to be used as a HW CS pin. This can be done
    /// by using the [configure_pin_as_hw_cs_pin] function which also returns the
    /// corresponding [HwChipSelectId].
    #[inline]
    pub fn cfg_hw_cs(&mut self, hw_cs: HwChipSelectId) {
        self.regs.modify_ctrl1(|mut value| {
            value.set_sod(false);
            value.set_ss(hw_cs);
            value
        });
    }

    /// Disables the hardware chip select functionality. This can be used when performing
    /// external chip select handling, for example with GPIO pins.
    #[inline]
    pub fn cfg_hw_cs_disable(&mut self) {
        self.regs.modify_ctrl1(|mut value| {
            value.set_sod(true);
            value
        });
    }

    /// Utility function to configure all relevant transfer parameters in one go.
    /// This is useful if multiple devices with different clock and mode configurations
    /// are connected to one bus.
    pub fn cfg_transfer(&mut self, transfer_cfg: &TransferConfig) {
        if let Some(trans_clk_div) = transfer_cfg.clk_cfg {
            self.cfg_clock(trans_clk_div);
        }
        if let Some(mode) = transfer_cfg.mode {
            self.cfg_mode(mode);
        }
        self.blockmode = transfer_cfg.blockmode;
        self.regs.modify_ctrl1(|mut value| {
            if transfer_cfg.sod {
                value.set_sod(transfer_cfg.sod);
            } else {
                value.set_sod(false);
                if let Some(hw_cs) = transfer_cfg.hw_cs {
                    value.set_ss(hw_cs);
                }
            }
            value.set_blockmode(transfer_cfg.blockmode);
            value.set_bm_stall(transfer_cfg.bmstall);
            value
        });
    }

    fn flush_internal(&mut self) {
        let mut status_reg = self.regs.read_status();
        while !status_reg.tx_empty() || status_reg.rx_not_empty() || status_reg.busy() {
            if status_reg.rx_not_empty() {
                self.read_fifo_unchecked();
            }
            status_reg = self.regs.read_status();
        }
    }

    fn transfer_preparation(&mut self, words: &[Word]) -> Result<(), Infallible> {
        if words.is_empty() {
            return Ok(());
        }
        self.flush_internal();
        Ok(())
    }

    // The FIFO can hold a guaranteed amount of data, so we can pump it on transfer
    // initialization. Returns the amount of written bytes.
    fn initial_send_fifo_pumping_with_words(&mut self, words: &[Word]) -> usize {
        //let reg_block = self.reg_block();
        if self.blockmode {
            self.regs.modify_ctrl1(|mut value| {
                value.set_mtxpause(true);
                value
            });
        }
        // Fill the first half of the write FIFO
        let mut current_write_idx = 0;
        let smaller_idx = core::cmp::min(FILL_DEPTH, words.len());
        for _ in 0..smaller_idx {
            if current_write_idx == smaller_idx.saturating_sub(1) && self.bmstall {
                self.write_fifo_unchecked(words[current_write_idx].into() | BMSTART_BMSTOP_MASK);
            } else {
                self.write_fifo_unchecked(words[current_write_idx].into());
            }
            current_write_idx += 1;
        }
        if self.blockmode {
            self.regs.modify_ctrl1(|mut value| {
                value.set_mtxpause(false);
                value
            });
        }
        current_write_idx
    }

    // The FIFO can hold a guaranteed amount of data, so we can pump it on transfer
    // initialization.
    fn initial_send_fifo_pumping_with_fill_words(&mut self, send_len: usize) -> usize {
        if self.blockmode {
            self.regs.modify_ctrl1(|mut value| {
                value.set_mtxpause(true);
                value
            });
        }
        // Fill the first half of the write FIFO
        let mut current_write_idx = 0;
        let smaller_idx = core::cmp::min(FILL_DEPTH, send_len);
        for _ in 0..smaller_idx {
            if current_write_idx == smaller_idx.saturating_sub(1) && self.bmstall {
                self.write_fifo_unchecked(self.fill_word.into() | BMSTART_BMSTOP_MASK);
            } else {
                self.write_fifo_unchecked(self.fill_word.into());
            }
            current_write_idx += 1;
        }
        if self.blockmode {
            self.regs.modify_ctrl1(|mut value| {
                value.set_mtxpause(false);
                value
            });
        }
        current_write_idx
    }
}

impl<W: SpiWord> SpiLowLevel for Spi<W>
where
    <W as TryFrom<u32>>::Error: core::fmt::Debug,
{
    #[inline(always)]
    fn write_fifo(&mut self, data: u32) -> nb::Result<(), Infallible> {
        if !self.regs.read_status().tx_not_full() {
            return Err(nb::Error::WouldBlock);
        }
        self.write_fifo_unchecked(data);
        Ok(())
    }

    #[inline(always)]
    fn write_fifo_unchecked(&mut self, data: u32) {
        self.regs.write_data(Data::new_with_raw_value(data));
    }

    #[inline(always)]
    fn read_fifo(&mut self) -> nb::Result<u32, Infallible> {
        if !self.regs.read_status().rx_not_empty() {
            return Err(nb::Error::WouldBlock);
        }
        Ok(self.read_fifo_unchecked())
    }

    #[inline(always)]
    fn read_fifo_unchecked(&mut self) -> u32 {
        self.regs.read_data().raw_value()
    }
}

impl<Word: SpiWord> embedded_hal::spi::ErrorType for Spi<Word> {
    type Error = Infallible;
}

impl<Word: SpiWord> embedded_hal::spi::SpiBus<Word> for Spi<Word>
where
    <Word as TryFrom<u32>>::Error: core::fmt::Debug,
{
    fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        self.transfer_preparation(words)?;
        let mut current_read_idx = 0;
        let mut current_write_idx = self.initial_send_fifo_pumping_with_fill_words(words.len());
        loop {
            if current_read_idx < words.len() {
                words[current_read_idx] = (nb::block!(self.read_fifo())? & Word::MASK)
                    .try_into()
                    .unwrap();
                current_read_idx += 1;
            }
            if current_write_idx < words.len() {
                if current_write_idx == words.len() - 1 && self.bmstall {
                    nb::block!(self.write_fifo(self.fill_word.into() | BMSTART_BMSTOP_MASK))?;
                } else {
                    nb::block!(self.write_fifo(self.fill_word.into()))?;
                }
                current_write_idx += 1;
            }
            if current_read_idx >= words.len() && current_write_idx >= words.len() {
                break;
            }
        }
        Ok(())
    }

    fn write(&mut self, words: &[Word]) -> Result<(), Self::Error> {
        self.transfer_preparation(words)?;
        let mut current_write_idx = self.initial_send_fifo_pumping_with_words(words);
        while current_write_idx < words.len() {
            if current_write_idx == words.len() - 1 && self.bmstall {
                nb::block!(self.write_fifo(words[current_write_idx].into() | BMSTART_BMSTOP_MASK))?;
            } else {
                nb::block!(self.write_fifo(words[current_write_idx].into()))?;
            }
            current_write_idx += 1;
            // Ignore received words.
            if self.regs.read_status().rx_not_empty() {
                self.clear_rx_fifo();
            }
        }
        Ok(())
    }

    fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error> {
        self.transfer_preparation(write)?;
        let mut current_read_idx = 0;
        let mut current_write_idx = self.initial_send_fifo_pumping_with_words(write);
        let max_idx = core::cmp::max(read.len(), write.len());
        while current_read_idx < read.len() || current_write_idx < write.len() {
            if current_write_idx < max_idx {
                if current_write_idx == write.len() - 1 && self.bmstall {
                    nb::block!(
                        self.write_fifo(write[current_write_idx].into() | BMSTART_BMSTOP_MASK)
                    )?;
                } else if current_write_idx < write.len() {
                    nb::block!(self.write_fifo(write[current_write_idx].into()))?;
                } else {
                    nb::block!(self.write_fifo(0))?;
                }
                current_write_idx += 1;
            }
            if current_read_idx < max_idx {
                if current_read_idx < read.len() {
                    read[current_read_idx] = (nb::block!(self.read_fifo())? & Word::MASK)
                        .try_into()
                        .unwrap();
                } else {
                    nb::block!(self.read_fifo()).unwrap();
                }
                current_read_idx += 1;
            }
        }

        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        self.transfer_preparation(words)?;
        let mut current_read_idx = 0;
        let mut current_write_idx = self.initial_send_fifo_pumping_with_words(words);

        while current_read_idx < words.len() || current_write_idx < words.len() {
            if current_write_idx < words.len() {
                if current_write_idx == words.len() - 1 && self.bmstall {
                    nb::block!(
                        self.write_fifo(words[current_write_idx].into() | BMSTART_BMSTOP_MASK)
                    )?;
                } else {
                    nb::block!(self.write_fifo(words[current_write_idx].into()))?;
                }
                current_write_idx += 1;
            }
            if current_read_idx < words.len() && current_read_idx < current_write_idx {
                words[current_read_idx] = (nb::block!(self.read_fifo())? & Word::MASK)
                    .try_into()
                    .unwrap();
                current_read_idx += 1;
            }
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush_internal();
        Ok(())
    }
}

/// Changing the word size also requires a type conversion
impl From<Spi<u8>> for Spi<u16> {
    fn from(mut old_spi: Spi<u8>) -> Self {
        old_spi.regs.modify_ctrl0(|mut value| {
            value.set_word_size(WordSize::SixteenBits);
            value
        });
        Spi {
            id: old_spi.id,
            regs: old_spi.regs,
            cfg: old_spi.cfg,
            blockmode: old_spi.blockmode,
            fill_word: Default::default(),
            bmstall: old_spi.bmstall,
            word: PhantomData,
        }
    }
}

impl From<Spi<u16>> for Spi<u8> {
    fn from(mut old_spi: Spi<u16>) -> Self {
        old_spi.regs.modify_ctrl0(|mut value| {
            value.set_word_size(WordSize::EightBits);
            value
        });
        Spi {
            id: old_spi.id,
            regs: old_spi.regs,
            cfg: old_spi.cfg,
            blockmode: old_spi.blockmode,
            fill_word: Default::default(),
            bmstall: old_spi.bmstall,
            word: PhantomData,
        }
    }
}
