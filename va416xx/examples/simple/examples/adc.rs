//! Simple ADC example.
#![no_main]
#![no_std]

// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use simple_examples::peb1;
use va416xx_hal::{
    adc::{Adc, ChannelSelect, ChannelValue, MultiChannelSelect},
    clock::ClockConfigurator,
    pac,
    timer::CountdownTimer,
};

// Quite spammy and disabled by default.
const ENABLE_BUF_PRINTOUT: bool = false;

#[entry]
fn main() -> ! {
    defmt::println!("VA416xx ADC example");

    let dp = pac::Peripherals::take().unwrap();
    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze()
        .unwrap();

    let adc = Adc::new_with_channel_tag(dp.adc, &clocks);
    let mut delay = CountdownTimer::new(dp.tim0, &clocks);
    let mut read_buf: [ChannelValue; 8] = [ChannelValue::default(); 8];
    loop {
        let single_value = adc
            .trigger_and_read_single_channel(va416xx_hal::adc::ChannelSelect::TempSensor)
            .expect("reading single channel value failed");
        defmt::info!(
            "Read single ADC value on temperature sensor channel: {:?}",
            single_value
        );
        let read_num = adc
            .sweep_and_read_range(0, 7, &mut read_buf)
            .expect("ADC range read failed");
        if ENABLE_BUF_PRINTOUT {
            defmt::info!("ADC Range Read (0-8) read {} values", read_num);
            defmt::info!("ADC Range Read (0-8): {:?}", read_buf);
        }
        assert_eq!(read_num, 8);
        for (idx, ch_val) in read_buf.iter().enumerate() {
            assert_eq!(
                ch_val.channel(),
                ChannelSelect::try_from(idx as u8).unwrap()
            );
        }

        adc.sweep_and_read_multiselect(
            MultiChannelSelect::AnIn0 | MultiChannelSelect::AnIn2 | MultiChannelSelect::TempSensor,
            &mut read_buf[0..3],
        )
        .expect("ADC multiselect read failed");
        if ENABLE_BUF_PRINTOUT {
            defmt::info!("ADC Multiselect Read(0, 2 and 10): {:?}", &read_buf[0..3]);
        }
        assert_eq!(read_buf[0].channel(), ChannelSelect::AnIn0);
        assert_eq!(read_buf[1].channel(), ChannelSelect::AnIn2);
        assert_eq!(read_buf[2].channel(), ChannelSelect::TempSensor);
        delay.delay_ms(500);
    }
}
