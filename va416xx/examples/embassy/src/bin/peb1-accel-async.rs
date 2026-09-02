//! Async I2C example which reads the LIS2DH12 accelerometer on the PEB1 board.
#![no_std]
#![no_main]
use defmt_rtt as _;
use panic_probe as _;

use embassy_example::EXTCLK_FREQ;
use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker};
use lis2dh12::{asynch::Lis2dh12, FullScale, Mode, Odr};
use once_cell::sync::OnceCell;

use va416xx_hal::{
    clock::ClockConfigurator,
    i2c::{self, asynch::I2c},
    pac::{self, interrupt},
    time::Hertz,
};

/// Token identifying the I2C0 peripheral, set once at construction and read by the interrupt
/// handler, which does not have access to the [I2c] driver itself.
static I2C_TOKEN: OnceCell<i2c::Bank> = OnceCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::println!("-- VA416xx Async I2C Accelerometer Example --");

    let dp = pac::Peripherals::take().unwrap();

    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(Hertz::from_raw(EXTCLK_FREQ))
        .freeze()
        .unwrap();
    // Safety: Only called once here.
    va416xx_hal::embassy_time::init(dp.tim15, dp.tim14, &clocks);

    let i2c_master = i2c::I2cMaster::new(
        dp.i2c0,
        &clocks,
        i2c::MasterConfig::default(),
        i2c::I2cSpeed::Regular100khz,
    )
    .expect("creating I2C master failed");

    let bank = i2c_master.id();
    let mut i2c = i2c_master.into_async();

    I2C_TOKEN.set(bank).unwrap();

    // Detect the accelerometer's address by scanning all possible values.
    let slave_addr = lis2dh12::asynch::detect_i2c_addr(&mut i2c)
        .await
        .expect("detecting I2C address failed");
    match &slave_addr {
        lis2dh12::SlaveAddr::Default => defmt::info!("Accelerometer slave address: Default"),
        lis2dh12::SlaveAddr::Alternative(a0) => {
            defmt::info!("Accelerometer slave address: Alternative({})", a0)
        }
    }
    let mut accelerometer = Lis2dh12::new(i2c, slave_addr)
        .await
        .expect("creating accelerometer driver failed");
    let device_id = accelerometer
        .get_device_id()
        .await
        .expect("reading device ID failed");
    defmt::info!("Device ID: 0x{:02X}", device_id);
    accelerometer
        .set_mode(Mode::Normal)
        .await
        .expect("setting mode failed");
    accelerometer
        .set_odr(Odr::Hz100)
        .await
        .expect("setting ODR failed");
    accelerometer
        .set_fs(FullScale::G4)
        .await
        .expect("setting full scale failed");
    // This function also enables BDU.
    accelerometer
        .enable_temp(true)
        .await
        .expect("enabling temperature sensor failed");

    let mut ticker = Ticker::every(Duration::from_millis(500));
    loop {
        let temperature = accelerometer
            .get_temp_outf()
            .await
            .expect("reading temperature failed");
        let value = accelerometer
            .accel_norm()
            .await
            .expect("reading accelerometer data failed");
        defmt::info!(
            "Accel Norm F32x3 {{ x: {}, y: {}, z: {} }} | Temp {} °C",
            value.x,
            value.y,
            value.z,
            temperature
        );
        ticker.next().await;
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn I2C0_MS() {
    I2c::on_interrupt(*I2C_TOKEN.get().unwrap());
}
