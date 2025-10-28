#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use gpio::Port;
pub use va108xx;
pub use va108xx as pac;

pub mod clock;
pub mod gpio;
pub mod i2c;
pub mod pins;
pub mod prelude;
pub mod pwm;
pub mod spi;
pub mod sysconfig;
pub mod time;
pub mod timer;
pub mod uart;

pub use vorago_shared_hal::{
    disable_nvic_interrupt, enable_nvic_interrupt, FunctionSelect, InterruptConfig,
    PeripheralSelect,
};

/// This is the NONE destination reigster value for the IRQSEL peripheral.
pub const IRQ_DST_NONE: u32 = 0xffffffff;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[error("invalid pin with number {0}")]
pub struct InvalidPinError(u8);

/// Can be used to manually manipulate the function select of port pins.
///
/// The function selection table can be found on p.36 of the programmers guide. Please note
/// that most of the structures and APIs in this library will automatically correctly configure
/// the pin or statically expect the correct pin type.
pub fn port_function_select(
    ioconfig: &mut pac::Ioconfig,
    port: Port,
    pin: u8,
    funsel: FunctionSelect,
) -> Result<(), InvalidPinError> {
    if (port == Port::A && pin >= 32) || (port == Port::B && pin >= 24) {
        return Err(InvalidPinError(pin));
    }

    let reg_block = match port {
        Port::A => ioconfig.porta(pin as usize),
        Port::B => ioconfig.portb(pin as usize),
    };

    reg_block.modify(|_, w| unsafe { w.funsel().bits(funsel as u8) });
    Ok(())
}

#[allow(dead_code)]
pub(crate) mod sealed {
    pub trait Sealed {}
}
