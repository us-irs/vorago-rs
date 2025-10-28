//! Board support crate for the VORAGO PEB1 board.
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub use lis2dh12;

/// Support for the LIS2DH12 accelerometer on the GPIO board.
///
/// # Example
///
/// - [PEB1 Accelerometer](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/peb1-accelerometer.rs)
pub mod accelerometer {

    use lis2dh12::{self, detect_i2c_addr, AddrDetectionError, Lis2dh12};
    use va416xx_hal::{
        clock::Clocks,
        i2c::{self, ClockTooSlowForFastI2cError, I2cMaster, I2cSpeed, MasterConfig},
        pac,
    };

    // Accelerometer located on the GPIO board.
    pub type Accelerometer = Lis2dh12<I2cMaster>;

    #[derive(Debug)]
    pub enum ConstructorError {
        ClkError(ClockTooSlowForFastI2cError),
        AddrDetectionError(AddrDetectionError<i2c::Error>),
        AccelerometerError(lis2dh12::Error<i2c::Error>),
    }

    pub fn new_with_addr_detection(
        i2c: pac::I2c0,
        clocks: &Clocks,
    ) -> Result<Accelerometer, ConstructorError> {
        let mut i2c_master = I2cMaster::new(
            i2c,
            clocks,
            MasterConfig::default(),
            I2cSpeed::Regular100khz,
        )
        .map_err(ConstructorError::ClkError)?;
        let slave_addr =
            detect_i2c_addr(&mut i2c_master).map_err(ConstructorError::AddrDetectionError)?;
        Lis2dh12::new(i2c_master, slave_addr).map_err(ConstructorError::AccelerometerError)
    }

    pub fn new_with_i2cm(
        i2c: I2cMaster,
        addr: lis2dh12::SlaveAddr,
    ) -> Result<Accelerometer, lis2dh12::Error<i2c::Error>> {
        Lis2dh12::new(i2c, addr)
    }
}
