pub mod regs;

use crate::{
    PeripheralSelect, enable_peripheral_clock, sealed::Sealed,
    sysconfig::reset_peripheral_for_cycles, time::Hertz,
};
use arbitrary_int::{u4, u10, u11, u20};
use core::marker::PhantomData;
use embedded_hal::i2c::{self, Operation, SevenBitAddress, TenBitAddress};
use regs::ClockTimeoutLimit;
pub use regs::{Bank, I2cSpeed, RxFifoFullMode, TxFifoEmptyMode};

#[cfg(feature = "vor1x")]
use va108xx as pac;
#[cfg(feature = "vor4x")]
use va416xx as pac;

//==================================================================================================
// Defintions
//==================================================================================================

const CLK_100K: Hertz = Hertz::from_raw(100_000);
const CLK_400K: Hertz = Hertz::from_raw(400_000);
const MIN_CLK_400K: Hertz = Hertz::from_raw(8_000_000);

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("clock too slow for fast I2C mode")]
pub struct ClockTooSlowForFastI2cError;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("invalid timing parameters")]
pub struct InvalidTimingParamsError;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    #[error("arbitration lost")]
    ArbitrationLost,
    #[error("nack address")]
    NackAddr,
    /// Data not acknowledged in write operation
    #[error("data not acknowledged in write operation")]
    NackData,
    /// Not enough data received in read operation
    #[error("insufficient data received")]
    InsufficientDataReceived,
    /// Number of bytes in transfer too large (larger than 0x7fe)
    #[error("data too large (larger than 0x7fe)")]
    DataTooLarge,
    #[error("clock timeout, SCL was low for {0} clock cycles")]
    ClockTimeout(u20),
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InitError {
    /// Wrong address used in constructor
    #[error("wrong address mode")]
    WrongAddrMode,
    /// APB1 clock is too slow for fast I2C mode.
    #[error("clock too slow for fast I2C mode: {0}")]
    ClockTooSlow(#[from] ClockTooSlowForFastI2cError),
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        match self {
            Error::ArbitrationLost => embedded_hal::i2c::ErrorKind::ArbitrationLoss,
            Error::NackAddr => {
                embedded_hal::i2c::ErrorKind::NoAcknowledge(i2c::NoAcknowledgeSource::Address)
            }
            Error::NackData => {
                embedded_hal::i2c::ErrorKind::NoAcknowledge(i2c::NoAcknowledgeSource::Data)
            }
            Error::DataTooLarge | Error::InsufficientDataReceived | Error::ClockTimeout(_) => {
                embedded_hal::i2c::ErrorKind::Other
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cCommand {
    Start = 0b01,
    Stop = 0b10,
    StartWithStop = 0b11,
    Cancel = 0b100,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cAddress {
    Regular(u8),
    TenBit(u16),
}

impl I2cAddress {
    pub fn ten_bit_addr(&self) -> bool {
        match self {
            I2cAddress::Regular(_) => false,
            I2cAddress::TenBit(_) => true,
        }
    }
    pub fn raw(&self) -> u16 {
        match self {
            I2cAddress::Regular(addr) => *addr as u16,
            I2cAddress::TenBit(addr) => *addr,
        }
    }
}

/// Common trait implemented by all PAC peripheral access structures. The register block
/// format is the same for all SPI blocks.
pub trait I2cInstance: Sealed {
    const ID: Bank;
    const PERIPH_SEL: PeripheralSelect;
}

#[cfg(feature = "vor1x")]
pub type I2c0 = pac::I2ca;
#[cfg(feature = "vor4x")]
pub type I2c0 = pac::I2c0;

impl I2cInstance for I2c0 {
    const ID: Bank = Bank::I2c0;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::I2c0;
}
impl Sealed for I2c0 {}

#[cfg(feature = "vor1x")]
pub type I2c1 = pac::I2cb;
#[cfg(feature = "vor4x")]
pub type I2c1 = pac::I2c1;

impl I2cInstance for I2c1 {
    const ID: Bank = Bank::I2c1;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::I2c1;
}
impl Sealed for I2c1 {}

//==================================================================================================
// Config
//==================================================================================================

fn calc_clk_div_generic(
    ref_clk: Hertz,
    speed_mode: I2cSpeed,
) -> Result<u8, ClockTooSlowForFastI2cError> {
    if speed_mode == I2cSpeed::Regular100khz {
        Ok(((ref_clk.raw() / CLK_100K.raw() / 20) - 1) as u8)
    } else {
        if ref_clk.raw() < MIN_CLK_400K.raw() {
            return Err(ClockTooSlowForFastI2cError);
        }
        Ok(((ref_clk.raw() / CLK_400K.raw() / 25) - 1) as u8)
    }
}

#[cfg(feature = "vor4x")]
fn calc_clk_div(
    clks: &crate::clock::Clocks,
    speed_mode: I2cSpeed,
) -> Result<u8, ClockTooSlowForFastI2cError> {
    calc_clk_div_generic(clks.apb1(), speed_mode)
}

#[cfg(feature = "vor1x")]
fn calc_clk_div(sys_clk: Hertz, speed_mode: I2cSpeed) -> Result<u8, ClockTooSlowForFastI2cError> {
    calc_clk_div_generic(sys_clk, speed_mode)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TimingConfig {
    pub t_rise: u4,
    pub t_fall: u4,
    pub t_high: u4,
    pub t_low: u4,
    pub tsu_stop: u4,
    pub tsu_start: u4,
    pub thd_start: u4,
    pub t_buf: u4,
}

/// Default configuration are the register reset value which are used by default.
impl Default for TimingConfig {
    fn default() -> Self {
        TimingConfig {
            t_rise: u4::new(0b0010),
            t_fall: u4::new(0b0001),
            t_high: u4::new(0b1000),
            t_low: u4::new(0b1001),
            tsu_stop: u4::new(0b1000),
            tsu_start: u4::new(0b1010),
            thd_start: u4::new(0b1000),
            t_buf: u4::new(0b1010),
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MasterConfig {
    pub tx_empty_mode: TxFifoEmptyMode,
    pub rx_full_mode: RxFifoFullMode,
    /// Enable the analog delay glitch filter
    pub alg_filt: bool,
    /// Enable the digital glitch filter
    pub dlg_filt: bool,
    pub timing_config: Option<TimingConfig>,
    /// See [I2cMaster::set_clock_low_timeout] documentation.
    pub timeout: Option<u20>,
    // Loopback mode
    // lbm: bool,
}

impl Default for MasterConfig {
    fn default() -> Self {
        MasterConfig {
            tx_empty_mode: TxFifoEmptyMode::Stall,
            rx_full_mode: RxFifoFullMode::Stall,
            alg_filt: false,
            dlg_filt: false,
            timeout: None,
            timing_config: None,
        }
    }
}

impl Sealed for MasterConfig {}

#[derive(Debug, PartialEq, Eq)]
enum WriteCompletionCondition {
    Idle,
    Waiting,
}

struct TimeoutGuard {
    clk_timeout_enabled: bool,
    regs: regs::MmioI2c<'static>,
}

impl TimeoutGuard {
    fn new(regs: &regs::MmioI2c<'static>) -> Self {
        let clk_timeout_enabled = regs.read_clk_timeout_limit().value().value() > 0;
        let mut guard = TimeoutGuard {
            clk_timeout_enabled,
            regs: unsafe { regs.clone() },
        };
        if clk_timeout_enabled {
            // Clear any interrupts which might be pending.
            guard.regs.write_irq_clear(
                regs::InterruptClear::builder()
                    .with_clock_timeout(true)
                    .with_tx_overflow(false)
                    .with_rx_overflow(false)
                    .build(),
            );
            guard.regs.modify_irq_enb(|mut value| {
                value.set_clock_timeout(true);
                value
            });
        }
        guard
    }

    fn timeout_enabled(&self) -> bool {
        self.clk_timeout_enabled
    }
}

impl Drop for TimeoutGuard {
    fn drop(&mut self) {
        if self.clk_timeout_enabled {
            self.regs.modify_irq_enb(|mut value| {
                value.set_clock_timeout(false);
                value
            });
        }
    }
}
//==================================================================================================
// I2C Master
//==================================================================================================

pub struct I2cMaster<Addr = SevenBitAddress> {
    id: Bank,
    regs: regs::MmioI2c<'static>,
    addr: PhantomData<Addr>,
}

impl<Addr> I2cMaster<Addr> {
    pub fn new<I2c: I2cInstance>(
        _i2c: I2c,
        #[cfg(feature = "vor1x")] sysclk: Hertz,
        #[cfg(feature = "vor4x")] clks: &crate::clock::Clocks,
        cfg: MasterConfig,
        speed_mode: I2cSpeed,
    ) -> Result<Self, ClockTooSlowForFastI2cError> {
        reset_peripheral_for_cycles(I2c::PERIPH_SEL, 2);
        enable_peripheral_clock(I2c::PERIPH_SEL);
        let mut regs = regs::I2c::new_mmio(I2c::ID);
        #[cfg(feature = "vor1x")]
        let clk_div = calc_clk_div(sysclk, speed_mode)?;
        #[cfg(feature = "vor4x")]
        let clk_div = calc_clk_div(clks, speed_mode)?;
        regs.write_clkscale(
            regs::ClockScale::builder()
                .with_div(clk_div)
                .with_fastmode(speed_mode)
                .build(),
        );
        regs.modify_control(|mut value| {
            value.set_tx_fifo_empty_mode(cfg.tx_empty_mode);
            value.set_rx_fifo_full_mode(cfg.rx_full_mode);
            value.set_analog_filter(cfg.alg_filt);
            value.set_digital_filter(cfg.dlg_filt);
            value
        });

        if let Some(ref timing_cfg) = cfg.timing_config {
            regs.modify_control(|mut value| {
                value.set_enable_timing_config(true);
                value
            });
            regs.write_timing_config(
                regs::TimingConfig::builder()
                    .with_t_rise(timing_cfg.t_rise)
                    .with_t_fall(timing_cfg.t_fall)
                    .with_t_high(timing_cfg.t_high)
                    .with_t_low(timing_cfg.t_low)
                    .with_tsu_stop(timing_cfg.tsu_stop)
                    .with_tsu_start(timing_cfg.tsu_start)
                    .with_thd_start(timing_cfg.thd_start)
                    .with_t_buf(timing_cfg.t_buf)
                    .build(),
            );
        }
        regs.write_fifo_clear(
            regs::FifoClear::builder()
                .with_tx_fifo(true)
                .with_rx_fifo(true)
                .build(),
        );
        if let Some(timeout) = cfg.timeout {
            regs.write_clk_timeout_limit(ClockTimeoutLimit::new(timeout));
        }
        let mut i2c_master = I2cMaster {
            addr: PhantomData,
            id: I2c::ID,
            regs,
        };
        i2c_master.enable();
        Ok(i2c_master)
    }

    pub const fn id(&self) -> Bank {
        self.id
    }

    #[inline]
    pub fn perid(&self) -> u32 {
        self.regs.read_perid()
    }

    /// Configures the clock scale for a given speed mode setting
    pub fn set_clk_scale(
        &mut self,
        #[cfg(feature = "vor1x")] sys_clk: Hertz,
        #[cfg(feature = "vor4x")] clks: &crate::clock::Clocks,
        speed_mode: I2cSpeed,
    ) -> Result<(), ClockTooSlowForFastI2cError> {
        self.disable();
        #[cfg(feature = "vor1x")]
        let clk_div = calc_clk_div(sys_clk, speed_mode)?;
        #[cfg(feature = "vor4x")]
        let clk_div = calc_clk_div(clks, speed_mode)?;
        self.regs.write_clkscale(
            regs::ClockScale::builder()
                .with_div(clk_div)
                .with_fastmode(speed_mode)
                .build(),
        );
        self.enable();
        Ok(())
    }

    #[inline]
    pub fn cancel_transfer(&mut self) {
        self.regs.write_cmd(
            regs::Command::builder()
                .with_start(false)
                .with_stop(false)
                .with_cancel(true)
                .build(),
        );
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

    /// Configure a timeout limit on the amount of time the I2C clock is seen to be low.
    /// The timeout is specified as I2C clock cycles.
    ///
    /// If the timeout is enabled, the blocking transaction handlers provided by the [I2cMaster]
    /// will poll the interrupt status register to check for timeouts. This can be used to avoid
    /// hang-ups of the I2C bus.
    #[inline]
    pub fn set_clock_low_timeout(&mut self, clock_cycles: u20) {
        self.regs
            .write_clk_timeout_limit(ClockTimeoutLimit::new(clock_cycles));
    }

    #[inline]
    pub fn disable_clock_low_timeout(&mut self) {
        self.regs
            .write_clk_timeout_limit(ClockTimeoutLimit::new(u20::new(0)));
    }

    #[inline]
    pub fn enable(&mut self) {
        self.regs.modify_control(|mut value| {
            value.set_enable(true);
            value
        });
    }

    #[inline]
    pub fn disable(&mut self) {
        self.regs.modify_control(|mut value| {
            value.set_enable(false);
            value
        });
    }

    #[inline(always)]
    fn write_fifo_unchecked(&mut self, word: u8) {
        self.regs.write_data(regs::Data::new(word));
    }

    #[inline(always)]
    fn read_fifo_unchecked(&self) -> u8 {
        self.regs.read_data().data()
    }

    #[inline]
    pub fn read_status(&mut self) -> regs::Status {
        self.regs.read_status()
    }

    #[inline]
    pub fn write_command(&mut self, cmd: I2cCommand) {
        self.regs
            .write_cmd(regs::Command::new_with_raw_value(cmd as u32));
    }

    #[inline]
    pub fn write_address(&mut self, addr: I2cAddress, dir: regs::Direction) {
        self.regs.write_address(
            regs::Address::builder()
                .with_direction(dir)
                .with_address(u10::new(addr.raw()))
                .with_a10_mode(addr.ten_bit_addr())
                .build(),
        );
    }

    fn error_handler_write(&mut self, init_cmd: I2cCommand) {
        if init_cmd == I2cCommand::Start {
            self.write_command(I2cCommand::Stop);
        }
        // The other case is start with stop where, so a CANCEL command should not be necessary
        // because the hardware takes care of it.
        self.clear_tx_fifo();
    }

    /// Blocking write transaction on the I2C bus.
    pub fn write_blocking(&mut self, addr: I2cAddress, output: &[u8]) -> Result<(), Error> {
        self.write_blocking_generic(
            I2cCommand::StartWithStop,
            addr,
            output,
            WriteCompletionCondition::Idle,
        )
    }

    /// Blocking read transaction on the I2C bus.
    pub fn read_blocking(&mut self, addr: I2cAddress, buffer: &mut [u8]) -> Result<(), Error> {
        let len = buffer.len();
        if len > 0x7fe {
            return Err(Error::DataTooLarge);
        }
        // Clear the receive FIFO
        self.clear_rx_fifo();

        let timeout_guard = TimeoutGuard::new(&self.regs);

        // Load number of words
        self.regs
            .write_words(regs::Words::new(u11::new(len as u16)));
        // Load address
        self.write_address(addr, regs::Direction::Receive);

        let mut buf_iter = buffer.iter_mut();
        let mut read_bytes = 0;
        // Start receive transfer
        self.write_command(I2cCommand::StartWithStop);
        loop {
            let status = self.read_status();
            if status.arb_lost() {
                self.clear_rx_fifo();
                return Err(Error::ArbitrationLost);
            }
            if status.nack_addr() {
                self.clear_rx_fifo();
                return Err(Error::NackAddr);
            }
            if status.idle() {
                if read_bytes != len {
                    return Err(Error::InsufficientDataReceived);
                }
                return Ok(());
            }
            if timeout_guard.timeout_enabled() && self.regs.read_irq_status().clock_timeout() {
                return Err(Error::ClockTimeout(
                    self.regs.read_clk_timeout_limit().value(),
                ));
            }
            if status.rx_not_empty() {
                if let Some(next_byte) = buf_iter.next() {
                    *next_byte = self.read_fifo_unchecked();
                }
                read_bytes += 1;
            }
        }
    }

    fn write_blocking_generic(
        &mut self,
        init_cmd: I2cCommand,
        addr: I2cAddress,
        output: &[u8],
        end_condition: WriteCompletionCondition,
    ) -> Result<(), Error> {
        let len = output.len();
        if len > 0x7fe {
            return Err(Error::DataTooLarge);
        }
        // Clear the send FIFO
        self.clear_tx_fifo();

        let timeout_guard = TimeoutGuard::new(&self.regs);

        // Load number of words
        self.regs
            .write_words(regs::Words::new(u11::new(len as u16)));
        let mut bytes = output.iter();
        // FIFO has a depth of 16. We load slightly above the trigger level
        // but not all of it because the transaction might fail immediately
        const FILL_DEPTH: usize = 12;

        let mut current_index = core::cmp::min(FILL_DEPTH, len);
        // load the FIFO
        for _ in 0..current_index {
            self.write_fifo_unchecked(*bytes.next().unwrap());
        }
        self.write_address(addr, regs::Direction::Send);
        self.write_command(init_cmd);
        loop {
            let status = self.regs.read_status();
            if status.arb_lost() {
                self.error_handler_write(init_cmd);
                return Err(Error::ArbitrationLost);
            }
            if status.nack_addr() {
                self.error_handler_write(init_cmd);
                return Err(Error::NackAddr);
            }
            if status.nack_data() {
                self.error_handler_write(init_cmd);
                return Err(Error::NackData);
            }
            match end_condition {
                WriteCompletionCondition::Idle => {
                    if status.idle() {
                        return Ok(());
                    }
                }

                WriteCompletionCondition::Waiting => {
                    if status.waiting() {
                        return Ok(());
                    }
                }
            }
            if timeout_guard.timeout_enabled() && self.regs.read_irq_status().clock_timeout() {
                return Err(Error::ClockTimeout(
                    self.regs.read_clk_timeout_limit().value(),
                ));
            }
            if status.tx_not_full() && current_index < len {
                self.write_fifo_unchecked(output[current_index]);
                current_index += 1;
            }
        }
    }

    /// Blocking write-read transaction on the I2C bus.
    pub fn write_read_blocking(
        &mut self,
        address: I2cAddress,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Error> {
        self.write_blocking_generic(
            I2cCommand::Start,
            address,
            write,
            WriteCompletionCondition::Waiting,
        )?;
        self.read_blocking(address, read)
    }
}

//======================================================================================
// Embedded HAL I2C implementations
//======================================================================================

impl embedded_hal::i2c::ErrorType for I2cMaster<SevenBitAddress> {
    type Error = Error;
}

impl embedded_hal::i2c::I2c for I2cMaster<SevenBitAddress> {
    fn transaction(
        &mut self,
        address: SevenBitAddress,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        for operation in operations {
            match operation {
                Operation::Read(buf) => self.read_blocking(I2cAddress::Regular(address), buf)?,
                Operation::Write(buf) => self.write_blocking(I2cAddress::Regular(address), buf)?,
            }
        }
        Ok(())
    }

    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        let addr = I2cAddress::Regular(address);
        self.write_read_blocking(addr, write, read)
    }
}

impl embedded_hal::i2c::ErrorType for I2cMaster<TenBitAddress> {
    type Error = Error;
}

impl embedded_hal::i2c::I2c<TenBitAddress> for I2cMaster<TenBitAddress> {
    fn transaction(
        &mut self,
        address: TenBitAddress,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        for operation in operations {
            match operation {
                Operation::Read(buf) => self.read_blocking(I2cAddress::TenBit(address), buf)?,
                Operation::Write(buf) => self.write_blocking(I2cAddress::TenBit(address), buf)?,
            }
        }
        Ok(())
    }

    fn write_read(
        &mut self,
        address: TenBitAddress,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        let addr = I2cAddress::TenBit(address);
        self.write_read_blocking(addr, write, read)
    }
}
