//! Example application which interfaces with the boot EEPROM.
#![no_main]
#![no_std]
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::{pac, spi::SpiClockConfig, time::Hertz, timer::CountdownTimer};
use vorago_reb1::m95m01::{M95M01, PAGE_SIZE};

const CLOCK_FREQ: Hertz = Hertz::from_raw(50_000_000);

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- VA108XX REB1 NVM example --");

    let dp = pac::Peripherals::take().unwrap();

    let mut delay = CountdownTimer::new(dp.tim0, CLOCK_FREQ);
    let clk_config = SpiClockConfig::new(2, 4);
    let mut nvm = M95M01::new(dp.spic, clk_config);
    let status_reg = nvm.read_status_reg().expect("reading status reg failed");
    if status_reg.zero_segment().value() == 0b111 {
        panic!("status register unexpected values");
    }

    let mut orig_content: [u8; 512] = [0; 512];
    let mut read_buf: [u8; 512] = [0; 512];
    let mut write_buf: [u8; 512] = [0; 512];
    for (idx, val) in write_buf.iter_mut().enumerate() {
        *val = ((idx as u16) % (u8::MAX as u16 + 1)) as u8;
    }
    nvm.read(0, &mut orig_content).unwrap();

    nvm.write_page(0, 0, &[1, 2, 3, 4]).unwrap();
    nvm.read(0, &mut read_buf[0..4]).unwrap();

    // Read the whole content. Write will internally be split across two page bounaries.
    nvm.write(0, &write_buf).unwrap();
    // Memory can be read in one go.
    nvm.read(0, &mut read_buf).unwrap();
    assert_eq!(&read_buf, &write_buf);
    assert!(nvm.verify(0, &write_buf).unwrap());
    read_buf.fill(0);

    // Write along page boundary
    nvm.write(PAGE_SIZE - 2, &write_buf[0..8]).unwrap();
    nvm.read(PAGE_SIZE - 2, &mut read_buf[0..8]).unwrap();
    assert_eq!(&read_buf[0..8], &write_buf[0..8]);
    assert!(nvm.verify(PAGE_SIZE - 2, &write_buf[0..8]).unwrap());

    nvm.write(0, &orig_content).unwrap();
    loop {
        delay.delay_ms(500);
    }
}
