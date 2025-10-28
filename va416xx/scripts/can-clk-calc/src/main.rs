use va416xx_hal::can::calculate_all_viable_clock_configs;
use va416xx_hal::time::Hertz;

fn main() {
    let cfgs = calculate_all_viable_clock_configs(
        Hertz::from_raw(20_000_000),
        Hertz::from_raw(250_000),
        0.75,
    )
    .unwrap();
    for cfg in &cfgs {
        println!("Config: {:#?}", cfg);
    }
}
