#![no_std]
#![no_main]

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use embassy_example::EXTCLK_FREQ;
use embassy_executor::Spawner;
use va416xx_hal::can::asynch::{on_interrupt_can, CanTxAsync};
use va416xx_hal::can::{
    Can, CanFrame, CanFrameNormal, CanFrameRtr, CanId, CanRx, CanTx, ClockConfig,
};
use va416xx_hal::clock::ClockConfigurator;
use va416xx_hal::pac::{self, interrupt};
use va416xx_hal::time::Hertz;
use va416xx_hal::{can, prelude::*};

const STANDARD_ID_0: can::StandardId = can::StandardId::new(0x42).unwrap();
const STANDARD_ID_1: can::StandardId = can::StandardId::new(0x5).unwrap();
const EXTENDED_ID_0: can::ExtendedId = can::ExtendedId::new(0x10).unwrap();

// Declare a bounded channel of 3 u32s.
static CAN_RX_CHANNEL: embassy_sync::channel::Channel<
    CriticalSectionRawMutex,
    (usize, CanFrame),
    3,
> = embassy_sync::channel::Channel::<CriticalSectionRawMutex, (usize, CanFrame), 3>::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::println!("-- VA416xx CAN Demo --");

    let dp = pac::Peripherals::take().unwrap();

    // Initialize the systick interrupt & obtain the token to prove that we did
    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(Hertz::from_raw(EXTCLK_FREQ))
        .freeze()
        .unwrap();
    // Safety: Only called once here.
    va416xx_embassy::init(dp.tim15, dp.tim14, &clocks);
    defmt::info!("creating CAN peripheral driver");
    defmt::info!("clocks: {}", clocks);
    let clk_config = ClockConfig::from_bitrate_and_segments(&clocks, 250.kHz(), 14, 5, 4)
        .expect("CAN clock config error");
    let mut can = Can::new(dp.can0, clk_config);
    can.modify_control(|mut val| {
        val.set_loopback(true);
        val.set_ignore_ack(true);
        val.set_internal(true);
        val.set_bufflock(true);
        val.set_diag_enable(true);
        val
    });
    can.set_global_mask_for_exact_id_match_with_rtr_masked();
    can.set_base_mask_for_all_match();
    can.enable();
    let mut channels = can.take_channels().unwrap();
    // Transmit channel.
    let mut tx = CanTx::new(channels.take(0).unwrap(), None);
    // Base channel which has dedicated mask.
    let mut rx_dedicated = CanRx::new(channels.take(1).unwrap());
    // Base channel which has dedicated mask.
    let mut rx_base = CanRx::new(channels.take(14).unwrap());
    rx_base.configure_for_reception();

    defmt::info!("Running blocking examples");

    send_and_receive_on_dedicated_channel(&mut can, &mut tx, &mut rx_dedicated);
    send_and_receive_rtr_on_dedicated_channel(&mut can, &mut tx, &mut rx_dedicated);
    send_extended_on_base_channel(&mut can, &mut tx, &mut rx_base);

    defmt::info!("Running non-blocking (asycnhronous) examples");

    non_blocking_example(&mut can, &mut rx_dedicated, &mut rx_base).await;

    defmt::info!("Non-blocking (asycnhronous) examples done");

    loop {
        cortex_m::asm::nop();
    }
}

fn send_and_receive_on_dedicated_channel(can: &mut Can, tx: &mut CanTx, rx_dedicated: &mut CanRx) {
    let send_data = &[1, 2, 3, 4];
    let sent_frame =
        CanFrame::Normal(CanFrameNormal::new(can::Id::Standard(STANDARD_ID_0), send_data).unwrap());
    defmt::info!(
        "sending CAN frame with ID {:#X} and data {}",
        STANDARD_ID_0.as_raw(),
        send_data
    );
    rx_dedicated.configure_for_reception_with_standard_id(STANDARD_ID_0, false);
    tx.transmit_frame(sent_frame).unwrap();
    // Await frame transmission completion.
    nb::block!(tx.transfer_done()).unwrap();
    check_and_handle_errors(can);
    let received_frame = nb::block!(rx_dedicated.receive(true)).expect("invalid CAN rx state");
    check_and_handle_errors(can);
    assert_eq!(received_frame, sent_frame);
    if let CanFrame::Normal(can_frame_normal) = received_frame {
        if let can::Id::Standard(standard_id) = can_frame_normal.id() {
            defmt::info!(
                "received CAN frame with ID {:#X} and data {}",
                standard_id.as_raw(),
                can_frame_normal.data()
            );
        } else {
            panic!("unexpected CAN extended frame ID");
        }
    } else {
        defmt::error!("received unexpected CAN remote frame");
    }
}

fn send_and_receive_rtr_on_dedicated_channel(
    can: &mut Can,
    tx: &mut CanTx,
    rx_dedicated: &mut CanRx,
) {
    let rtr_frame = CanFrame::Rtr(CanFrameRtr::new(can::Id::Standard(STANDARD_ID_1), 0));
    // RTR bit is masked, so the setting should not matter.
    rx_dedicated.configure_for_reception_with_standard_id(STANDARD_ID_1, false);
    tx.transmit_frame(rtr_frame).unwrap();
    // Await frame transmission completion.
    nb::block!(tx.remote_transfer_done_with_tx_reconfig()).unwrap();
    check_and_handle_errors(can);
    let received_frame = nb::block!(rx_dedicated.receive(true)).expect("invalid CAN rx state");
    check_and_handle_errors(can);
    assert_eq!(received_frame, rtr_frame);
    if let CanFrame::Rtr(can_frame_rtr) = received_frame {
        if let can::Id::Standard(standard_id) = can_frame_rtr.id() {
            defmt::info!("received CAN RTR frame with ID {:#X}", standard_id.as_raw(),);
        } else {
            panic!("unexpected CAN extended frame ID");
        }
    } else {
        defmt::error!("received unexpected CAN data frame");
    }
}

fn check_and_handle_errors(can: &mut Can) {
    let err_counter = can.read_error_counters();
    if err_counter.transmit() > 0 || err_counter.receive() > 0 {
        defmt::warn!(
            "error count tx {}, error count rx {}",
            err_counter.transmit(),
            err_counter.receive()
        );
        let diag = can.read_error_diagnostics();
        defmt::warn!("EFID: {}, EBID: {}", diag.efid(), diag.ebid());
    }
}

fn send_extended_on_base_channel(can: &mut Can, tx: &mut CanTx, rx: &mut CanRx) {
    let send_data = &[4, 3, 2, 1];
    let sent_frame =
        CanFrame::Normal(CanFrameNormal::new(can::Id::Extended(EXTENDED_ID_0), send_data).unwrap());
    tx.transmit_frame(sent_frame).unwrap();
    // Await frame transmission completion.
    nb::block!(tx.transfer_done()).unwrap();
    check_and_handle_errors(can);
    let received_frame = nb::block!(rx.receive(true)).expect("invalid CAN rx state");
    check_and_handle_errors(can);
    assert_eq!(sent_frame, received_frame);
    if let CanFrame::Normal(can_frame_normal) = received_frame {
        if let can::Id::Extended(extended_id) = can_frame_normal.id() {
            defmt::info!(
                "received CAN frame with ID {:#X} and data {}",
                extended_id.as_raw(),
                can_frame_normal.data()
            );
        } else {
            panic!("unexpected CAN extended frame ID");
        }
    } else {
        defmt::error!("received unexpected CAN data frame");
    }
}

async fn non_blocking_example(can: &mut Can, rx_dedicated: &mut CanRx, rx_base: &mut CanRx) {
    let mut tx_async = CanTxAsync::new(can);
    // Enable interrupts for RX channels.
    rx_dedicated.enable_interrupt(true);
    rx_base.enable_interrupt(true);

    // For asynchronous mode, all TX channels needs to be configured explicitely. Configuring more
    // channels allows multiple active transfers when using the async API.
    tx_async.configure_channel(0).unwrap();
    let send_data = &[1, 2, 3, 4];
    let send_frame =
        CanFrame::Normal(CanFrameNormal::new(can::Id::Standard(STANDARD_ID_0), send_data).unwrap());
    let fut = tx_async.start_transmit(send_frame).unwrap();
    fut.await;
    let (ch_idx, frame) = CAN_RX_CHANNEL.receive().await;
    assert_eq!(send_frame, frame);
    // Received on base channel.
    assert_eq!(ch_idx, 14);
    if let CanFrame::Normal(can_frame_normal) = frame {
        if let can::Id::Standard(standard_id) = can_frame_normal.id() {
            defmt::info!(
                "received CAN frame with ID {:#X} and data {}",
                standard_id.as_raw(),
                can_frame_normal.data()
            );
        } else {
            panic!("unexpected CAN extended frame ID");
        }
    } else {
        defmt::error!("received unexpected CAN remote frame");
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn CAN0() {
    match on_interrupt_can(CanId::Can0, false).unwrap() {
        can::asynch::InterruptResult::NoInterrupt => {
            defmt::warn!("unexpected interrupt on CAN0");
        }
        can::asynch::InterruptResult::ReceivedFrame {
            channel_index,
            frame,
        } => {
            CAN_RX_CHANNEL.try_send((channel_index, frame)).unwrap();
        }
        can::asynch::InterruptResult::TransmissionEvent { channel_index, id } => {
            defmt::info!(
                "transmission event on channel {} with event ID {}",
                channel_index,
                id
            );
        }
    }
}
