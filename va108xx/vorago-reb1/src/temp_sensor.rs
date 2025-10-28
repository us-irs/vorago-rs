//! # API for the On-Board Analog Devices ADT75 temperature sensor
//!
//! [Datasheet](https://www.analog.com/media/en/technical-documentation/data-sheets/ADT75.pdf)
//!
//! ## Examples
//!
//! - [Temperature Sensor example](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/vorago-reb1/examples/adt75-temp-sensor.rs)
use embedded_hal::i2c::{I2c, SevenBitAddress};
use va108xx_hal::{
    i2c::{Error, I2cMaster, I2cSpeed, InitError, MasterConfig},
    pac,
    time::Hertz,
};

const ADT75_I2C_ADDR: u8 = 0b1001000;

pub struct Adt75TempSensor {
    sensor_if: I2cMaster<SevenBitAddress>,
    cmd_buf: [u8; 1],
    current_reg: RegAddresses,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RegAddresses {
    Temperature = 0x00,
    Configuration = 0x01,
    THystSetpoint = 0x02,
    TOsSetPoint = 0x03,
    OneShot = 0x04,
}

#[derive(Debug)]
pub enum AdtInitError {
    Init(InitError),
    I2c(Error),
}

impl From<InitError> for AdtInitError {
    fn from(value: InitError) -> Self {
        Self::Init(value)
    }
}

impl From<Error> for AdtInitError {
    fn from(value: Error) -> Self {
        Self::I2c(value)
    }
}

impl Adt75TempSensor {
    pub fn new(sys_clk: Hertz, i2ca: pac::I2ca) -> Result<Self, Error> {
        let mut sensor = Adt75TempSensor {
            // The master construction can not fail for regular I2C speed.
            sensor_if: I2cMaster::new(
                i2ca,
                sys_clk,
                MasterConfig::default(),
                I2cSpeed::Regular100khz,
            )
            .unwrap(),
            cmd_buf: [RegAddresses::Temperature as u8],
            current_reg: RegAddresses::Temperature,
        };
        sensor.select_reg(RegAddresses::Temperature)?;
        Ok(sensor)
    }

    pub fn select_reg(&mut self, reg: RegAddresses) -> Result<(), Error> {
        if reg != self.current_reg {
            self.cmd_buf[0] = reg as u8;
            self.current_reg = reg;
            self.sensor_if.write(ADT75_I2C_ADDR, &self.cmd_buf[0..1])?;
        }
        Ok(())
    }

    pub fn read_temperature(&mut self) -> Result<f32, Error> {
        if self.current_reg != RegAddresses::Temperature {
            self.select_reg(RegAddresses::Temperature)?;
        }
        let mut reply: [u8; 2] = [0; 2];
        self.sensor_if.read(ADT75_I2C_ADDR, &mut reply)?;
        let adc_code = (((reply[0] as u16) << 8) | reply[1] as u16) >> 4;
        let temp_celcius: f32 = if ((adc_code >> 11) & 0x01) == 0 {
            // Sign bit not set, positiv value
            // Divide ADC code by 16 according to datasheet
            adc_code as f32 / 16.0
        } else {
            // Calculation for negative values, assuming all 12 bits are used
            (adc_code - 4096) as f32 / 16.0
        };
        Ok(temp_celcius)
    }
}
