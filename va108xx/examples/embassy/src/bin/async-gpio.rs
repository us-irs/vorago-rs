//! This example demonstrates the usage of async GPIO operations on VA108xx.
//!
//! You need to tie the PA0 to the PA1 pin for this example to work. You can optionally tie the PB22 to PB23 pins well
//! and then set the `CHECK_PB22_TO_PB23` to true to also test async operations on Port B.
#![no_std]
#![no_main]
// This imports the logger and the panic handler.
use embassy_example as _;

use embassy_executor::Spawner;
use embassy_sync::channel::{Receiver, Sender};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::{Duration, Instant, Timer};
use embedded_hal_async::digital::Wait;
use va108xx_hal::gpio::asynch::{on_interrupt_for_async_gpio_for_port, InputPinAsync};
use va108xx_hal::gpio::{Input, Output, PinState, Port};
use va108xx_hal::pins::{PinsA, PinsB};
use va108xx_hal::{
    pac::{self, interrupt},
    prelude::*,
};

const SYSCLK_FREQ: Hertz = Hertz::from_raw(50_000_000);

const CHECK_PA0_TO_PA1: bool = true;
const CHECK_PB22_TO_PB23: bool = false;

// Can also be set to OC10 and works as well.
const PB22_TO_PB23_IRQ: pac::Interrupt = pac::Interrupt::OC11;

#[derive(Clone, Copy)]
pub struct GpioCmd {
    cmd_type: GpioCmdType,
    after_delay: u32,
}

impl GpioCmd {
    pub fn new(cmd_type: GpioCmdType, after_delay: u32) -> Self {
        Self {
            cmd_type,
            after_delay,
        }
    }
}

#[derive(Clone, Copy)]
pub enum GpioCmdType {
    SetHigh,
    SetLow,
    RisingEdge,
    FallingEdge,
}

// Declare a bounded channel of 3 u32s.
static CHANNEL_PA0_PA1: Channel<ThreadModeRawMutex, GpioCmd, 3> = Channel::new();
static CHANNEL_PB22_TO_PB23: Channel<ThreadModeRawMutex, GpioCmd, 3> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::println!("-- VA108xx Async GPIO Demo --");

    let dp = pac::Peripherals::take().unwrap();

    // Safety: Only called once here.
    va108xx_embassy::init(dp.tim23, dp.tim22, SYSCLK_FREQ);

    let porta = PinsA::new(dp.porta);
    let portb = PinsB::new(dp.portb);
    let mut led0 = Output::new(porta.pa10, PinState::Low);
    let out_pa0 = Output::new(porta.pa0, PinState::Low);
    let in_pa1 = Input::new_floating(porta.pa1);
    let out_pb22 = Output::new(portb.pb22, PinState::Low);
    let in_pb23 = Input::new_floating(portb.pb23);

    let in_pa1_async = InputPinAsync::new(in_pa1, pac::Interrupt::OC10);
    let in_pb23_async = InputPinAsync::new(in_pb23, PB22_TO_PB23_IRQ);

    spawner
        .spawn(output_task(
            "PA0 to PA1",
            out_pa0,
            CHANNEL_PA0_PA1.receiver(),
        ))
        .unwrap();
    spawner
        .spawn(output_task(
            "PB22 to PB23",
            out_pb22,
            CHANNEL_PB22_TO_PB23.receiver(),
        ))
        .unwrap();

    if CHECK_PA0_TO_PA1 {
        check_pin_to_pin_async_ops("PA0 to PA1", CHANNEL_PA0_PA1.sender(), in_pa1_async).await;
        defmt::info!("Example PA0 to PA1 done");
    }
    if CHECK_PB22_TO_PB23 {
        check_pin_to_pin_async_ops("PB22 to PB23", CHANNEL_PB22_TO_PB23.sender(), in_pb23_async)
            .await;
        defmt::info!("Example PB22 to PB23 done");
    }

    defmt::info!("Example done, toggling LED0");
    loop {
        led0.toggle();
        Timer::after(Duration::from_millis(500)).await;
    }
}

async fn check_pin_to_pin_async_ops(
    ctx: &'static str,
    sender: Sender<'static, ThreadModeRawMutex, GpioCmd, 3>,
    mut async_input: impl Wait,
) {
    defmt::info!(
        "{}: sending SetHigh command ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
    sender.send(GpioCmd::new(GpioCmdType::SetHigh, 20)).await;
    async_input.wait_for_high().await.unwrap();
    defmt::info!(
        "{}: Input pin is high now ({} ms)",
        ctx,
        Instant::now().as_millis()
    );

    defmt::info!(
        "{}: sending SetLow command ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
    sender.send(GpioCmd::new(GpioCmdType::SetLow, 20)).await;
    async_input.wait_for_low().await.unwrap();
    defmt::info!(
        "{}: Input pin is low now ({} ms)",
        ctx,
        Instant::now().as_millis()
    );

    defmt::info!(
        "{}: sending RisingEdge command ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
    sender.send(GpioCmd::new(GpioCmdType::RisingEdge, 20)).await;
    async_input.wait_for_rising_edge().await.unwrap();
    defmt::info!(
        "{}: input pin had rising edge ({} ms)",
        ctx,
        Instant::now().as_millis()
    );

    defmt::info!(
        "{}: sending Falling command ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
    sender
        .send(GpioCmd::new(GpioCmdType::FallingEdge, 20))
        .await;
    async_input.wait_for_falling_edge().await.unwrap();
    defmt::info!(
        "{}: input pin had a falling edge ({} ms)",
        ctx,
        Instant::now().as_millis()
    );

    defmt::info!(
        "{}: sending Falling command ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
    sender
        .send(GpioCmd::new(GpioCmdType::FallingEdge, 20))
        .await;
    async_input.wait_for_any_edge().await.unwrap();
    defmt::info!(
        "{}: input pin had a falling (any) edge ({} ms)",
        ctx,
        Instant::now().as_millis()
    );

    defmt::info!(
        "{}: sending Falling command ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
    sender.send(GpioCmd::new(GpioCmdType::RisingEdge, 20)).await;
    async_input.wait_for_any_edge().await.unwrap();
    defmt::info!(
        "{}: input pin had a rising (any) edge ({} ms)",
        ctx,
        Instant::now().as_millis()
    );
}

#[embassy_executor::task(pool_size = 2)]
async fn output_task(
    ctx: &'static str,
    mut out: Output,
    receiver: Receiver<'static, ThreadModeRawMutex, GpioCmd, 3>,
) {
    loop {
        let next_cmd = receiver.receive().await;
        Timer::after(Duration::from_millis(next_cmd.after_delay.into())).await;
        match next_cmd.cmd_type {
            GpioCmdType::SetHigh => {
                defmt::info!("{}: Set output high", ctx);
                out.set_high();
            }
            GpioCmdType::SetLow => {
                defmt::info!("{}: Set output low", ctx);
                out.set_low();
            }
            GpioCmdType::RisingEdge => {
                defmt::info!("{}: Rising edge", ctx);
                if !out.is_set_low() {
                    out.set_low();
                }
                out.set_high();
            }
            GpioCmdType::FallingEdge => {
                defmt::info!("{}: Falling edge", ctx);
                if !out.is_set_high() {
                    out.set_high();
                }
                out.set_low();
            }
        }
    }
}

// PB22 to PB23 can be handled by both OC10 and OC11 depending on configuration.
#[interrupt]
#[allow(non_snake_case)]
fn OC10() {
    on_interrupt_for_async_gpio_for_port(Port::A);
    on_interrupt_for_async_gpio_for_port(Port::B);
}

// This interrupt only handles PORT B interrupts.
#[interrupt]
#[allow(non_snake_case)]
fn OC11() {
    on_interrupt_for_async_gpio_for_port(Port::B);
}
