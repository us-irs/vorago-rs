// UART example application. Sends a test string over a UART and then enters
// echo mode
#![no_main]
#![no_std]
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use cortex_m_rt::entry;
use embedded_hal_nb::serial::Read;
use embedded_io::Write;
use simple_examples::peb1;
use va416xx_hal::clock::ClockConfigurator;
use va416xx_hal::pins::PinsG;
use va416xx_hal::time::Hertz;
use va416xx_hal::{pac, uart};

#[entry]
fn main() -> ! {
    defmt::println!("-- VA416xx UART example application--");

    let dp = pac::Peripherals::take().unwrap();

    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze()
        .unwrap();

    let gpiog = PinsG::new(dp.portg);

    let uart0 = uart::Uart::new(
        dp.uart0,
        gpiog.pg0,
        gpiog.pg1,
        &clocks,
        Hertz::from_raw(115200).into(),
    )
    .unwrap();
    let (mut tx, mut rx) = uart0.split();
    writeln!(tx, "Hello World\n\r").unwrap();
    loop {
        // Echo what is received on the serial link.
        match nb::block!(rx.read()) {
            Ok(recvd) => {
                // Infallible operation.
                embedded_hal_nb::serial::Write::write(&mut tx, recvd).unwrap();
            }
            Err(e) => {
                defmt::info!("UART RX error {:?}", e);
            }
        }
    }
}
