//! Vorago flashloader which can be used to flash image A and image B via a simple
//! low-level CCSDS memory interface via a UART interface.
#![no_main]
#![no_std]

use defmt_rtt as _; // global logger
use num_enum::TryFromPrimitive;
use panic_probe as _;
use ringbuf::{
    traits::{Consumer, Observer, Producer},
    StaticRb,
};
use va108xx_hal::prelude::*;

const SYSCLK_FREQ: Hertz = Hertz::from_raw(50_000_000);

const MAX_TC_SIZE: usize = 524;
const MAX_TC_FRAME_SIZE: usize = cobs::max_encoding_length(MAX_TC_SIZE);

const MAX_TM_SIZE: usize = 128;
const MAX_TM_FRAME_SIZE: usize = cobs::max_encoding_length(MAX_TM_SIZE);

const UART_BAUDRATE: u32 = 115200;
const BOOT_NVM_MEMORY_ID: u8 = 1;
const RX_DEBUGGING: bool = false;

pub enum ActionId {
    CorruptImageA = 128,
    CorruptImageB = 129,
    SetBootSlot = 130,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive, defmt::Format)]
#[repr(u8)]
enum AppSel {
    A = 0,
    B = 1,
}

// Larger buffer for TC to be able to hold the possibly large memory write packets.
const BUF_RB_SIZE_TC: usize = 1024;
const SIZES_RB_SIZE_TC: usize = 16;

const BUF_RB_SIZE_TM: usize = 256;
const SIZES_RB_SIZE_TM: usize = 16;

pub struct RingBufWrapper<const BUF_SIZE: usize, const SIZES_LEN: usize> {
    pub buf: StaticRb<u8, BUF_SIZE>,
    pub sizes: StaticRb<usize, SIZES_LEN>,
}

pub const APP_A_START_ADDR: u32 = 0x3000;
pub const APP_A_END_ADDR: u32 = 0x117FC;
pub const APP_B_START_ADDR: u32 = APP_A_END_ADDR;
pub const APP_B_END_ADDR: u32 = 0x20000;

pub const PREFERRED_SLOT_OFFSET: u32 = 0x20000 - 1;

#[rtic::app(device = pac, dispatchers = [OC20, OC21, OC22])]
mod app {
    use super::*;
    use cortex_m::asm;
    use embedded_io::Write;
    use rtic::Mutex;
    use rtic_monotonics::systick::prelude::*;
    use satrs::pus::verification::{FailParams, VerificationReportCreator};
    use spacepackets::ecss::PusServiceId;
    use spacepackets::ecss::{
        tc::PusTcReader, tm::PusTmCreator, EcssEnumU8, PusPacket, WritablePusPacket,
    };
    use va108xx_hal::pins::PinsA;
    use va108xx_hal::spi::SpiClockConfig;
    use va108xx_hal::uart::InterruptContextTimeoutOrMaxSize;
    use va108xx_hal::{pac, uart, InterruptConfig};
    use vorago_reb1::m95m01::M95M01;

    #[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CobsReaderStates {
        #[default]
        WaitingForStart,
        WatingForEnd,
        FrameOverflow,
    }

    #[local]
    struct Local {
        uart_rx: uart::RxWithInterrupt,
        uart_tx: uart::Tx,
        rx_context: InterruptContextTimeoutOrMaxSize,
        verif_reporter: VerificationReportCreator,
        nvm: M95M01,
    }

    #[shared]
    struct Shared {
        // Having this shared allows multiple tasks to generate telemetry.
        tm_rb: RingBufWrapper<BUF_RB_SIZE_TM, SIZES_RB_SIZE_TM>,
        tc_rb: RingBufWrapper<BUF_RB_SIZE_TC, SIZES_RB_SIZE_TC>,
    }

    rtic_monotonics::systick_monotonic!(Mono, 1000);

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::println!("-- Vorago flashloader --");

        Mono::start(cx.core.SYST, SYSCLK_FREQ.raw());

        let dp = cx.device;
        let spi_clock_config = SpiClockConfig::new(2, 4);
        let nvm = M95M01::new(dp.spic, spi_clock_config);

        let gpioa = PinsA::new(dp.porta);
        let tx = gpioa.pa9;
        let rx = gpioa.pa8;

        let irq_uart = uart::Uart::new_with_interrupt(
            dp.uarta,
            tx,
            rx,
            SYSCLK_FREQ,
            UART_BAUDRATE.Hz().into(),
            InterruptConfig::new(pac::Interrupt::OC0, true, true),
        )
        .unwrap();
        let (tx, rx) = irq_uart.split();
        // Unwrap is okay, we explicitely set the interrupt ID.
        let mut rx = rx.into_rx_with_irq();

        let verif_reporter = VerificationReportCreator::new(0).unwrap();

        let mut rx_context = InterruptContextTimeoutOrMaxSize::new(MAX_TC_FRAME_SIZE);
        rx.read_fixed_len_or_timeout_based_using_irq(&mut rx_context)
            .expect("initiating UART RX failed");
        pus_tc_handler::spawn().unwrap();
        pus_tm_tx_handler::spawn().unwrap();
        (
            Shared {
                tc_rb: RingBufWrapper {
                    buf: StaticRb::default(),
                    sizes: StaticRb::default(),
                },
                tm_rb: RingBufWrapper {
                    buf: StaticRb::default(),
                    sizes: StaticRb::default(),
                },
            },
            Local {
                uart_rx: rx,
                uart_tx: tx,
                rx_context,
                verif_reporter,
                nvm,
            },
        )
    }

    // `shared` cannot be accessed from this context
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            asm::nop();
        }
    }

    // This is the interrupt handler to read all bytes received on the UART0.
    #[task(
        binds = OC0,
        local = [
            cnt: u32 = 0,
            rx_buf: [u8; MAX_TC_FRAME_SIZE] = [0; MAX_TC_FRAME_SIZE],
            rx_context,
            uart_rx,
        ],
        shared = [tc_rb]
    )]
    fn uart_rx_irq(mut cx: uart_rx_irq::Context) {
        match cx
            .local
            .uart_rx
            .on_interrupt_max_size_or_timeout_based(cx.local.rx_context, cx.local.rx_buf)
        {
            Ok(result) => {
                if RX_DEBUGGING {
                    defmt::debug!("RX Info: {:?}", cx.local.rx_context);
                    defmt::debug!("RX Result: {:?}", result);
                }
                if result.complete() {
                    // Check frame validity (must have COBS format) and decode the frame.
                    // Currently, we expect a full frame or a frame received through a timeout
                    // to be one COBS frame. We could parse for multiple COBS packets in one
                    // frame, but the additional complexity is not necessary here..
                    if cx.local.rx_buf[0] == 0 && cx.local.rx_buf[result.bytes_read - 1] == 0 {
                        let decoded_size =
                            cobs::decode_in_place(&mut cx.local.rx_buf[1..result.bytes_read]);
                        if decoded_size.is_err() {
                            defmt::warn!("COBS decoding failed");
                        } else {
                            let decoded_size = decoded_size.unwrap();
                            let mut tc_rb_full = false;
                            cx.shared.tc_rb.lock(|rb| {
                                if rb.sizes.vacant_len() >= 1 && rb.buf.vacant_len() >= decoded_size
                                {
                                    rb.sizes.try_push(decoded_size).unwrap();
                                    rb.buf.push_slice(&cx.local.rx_buf[1..1 + decoded_size]);
                                } else {
                                    tc_rb_full = true;
                                }
                            });
                            if tc_rb_full {
                                defmt::warn!("COBS TC queue full");
                            }
                        }
                    } else {
                        defmt::warn!(
                            "COBS frame with invalid format, start and end bytes are not 0"
                        );
                    }

                    // Initiate next transfer.
                    cx.local
                        .uart_rx
                        .read_fixed_len_or_timeout_based_using_irq(cx.local.rx_context)
                        .expect("read operation failed");
                }
                if result.has_errors() {
                    defmt::warn!("UART error: {:?}", result.errors.unwrap());
                }
            }
            Err(e) => {
                defmt::warn!("UART error: {:?}", e);
            }
        }
    }

    #[task(
        priority = 2,
        local=[
            tc_buf: [u8; MAX_TC_SIZE] = [0; MAX_TC_SIZE],
            readback_buf: [u8; MAX_TC_SIZE] = [0; MAX_TC_SIZE],
            src_data_buf: [u8; 16] = [0; 16],
            verif_buf: [u8; 32] = [0; 32],
            nvm,
            verif_reporter
        ],
        shared=[tm_rb, tc_rb]
    )]
    async fn pus_tc_handler(mut cx: pus_tc_handler::Context) {
        loop {
            // Try to read a TC from the ring buffer.
            let packet_len = cx.shared.tc_rb.lock(|rb| rb.sizes.try_pop());
            if packet_len.is_none() {
                // Small delay, TCs might arrive very quickly.
                Mono::delay(20.millis()).await;
                continue;
            }
            let packet_len = packet_len.unwrap();
            defmt::info!("received packet with length {}", packet_len);
            let popped_packet_len = cx
                .shared
                .tc_rb
                .lock(|rb| rb.buf.pop_slice(&mut cx.local.tc_buf[0..packet_len]));
            assert_eq!(popped_packet_len, packet_len);
            // Read a telecommand, now handle it.
            handle_valid_pus_tc(&mut cx);
        }
    }

    fn handle_valid_pus_tc(cx: &mut pus_tc_handler::Context) {
        let pus_tc = PusTcReader::new(cx.local.tc_buf);
        if pus_tc.is_err() {
            defmt::warn!("PUS TC error: {}", pus_tc.unwrap_err());
            return;
        }
        let pus_tc = pus_tc.unwrap();
        let mut write_and_send = |tm: &PusTmCreator| {
            let written_size = tm.write_to_bytes(cx.local.verif_buf).unwrap();
            cx.shared.tm_rb.lock(|prod| {
                prod.sizes.try_push(tm.len_written()).unwrap();
                prod.buf.push_slice(&cx.local.verif_buf[0..written_size]);
            });
        };
        let request_id = VerificationReportCreator::read_request_id_from_tc(&pus_tc);
        let tm = cx
            .local
            .verif_reporter
            .acceptance_success(cx.local.src_data_buf, &request_id, 0, 0, &[])
            .expect("acceptance success failed");
        write_and_send(&tm);

        let tm = cx
            .local
            .verif_reporter
            .start_success(cx.local.src_data_buf, &request_id, 0, 0, &[])
            .expect("acceptance success failed");
        write_and_send(&tm);

        if pus_tc.service() == PusServiceId::Action as u8 {
            let mut corrupt_image = |base_addr: u32| {
                let mut buf = [0u8; 4];
                cx.local
                    .nvm
                    .read(base_addr as usize + 32, &mut buf)
                    .expect("reading from NVM failed");
                buf[0] += 1;
                cx.local
                    .nvm
                    .write(base_addr as usize + 32, &buf)
                    .expect("writing to NVM failed");
                let tm = cx
                    .local
                    .verif_reporter
                    .completion_success(cx.local.src_data_buf, &request_id, 0, 0, &[])
                    .expect("completion success failed");
                write_and_send(&tm);
            };
            if pus_tc.subservice() == ActionId::CorruptImageA as u8 {
                defmt::info!("corrupting App Image A");
                corrupt_image(APP_A_START_ADDR);
            }
            if pus_tc.subservice() == ActionId::CorruptImageB as u8 {
                defmt::info!("corrupting App Image B");
                corrupt_image(APP_B_START_ADDR);
            }
            if pus_tc.subservice() == ActionId::SetBootSlot as u8 {
                if pus_tc.app_data().is_empty() {
                    defmt::warn!("App data for preferred image command too short");
                }
                let app_sel_result = AppSel::try_from(pus_tc.app_data()[0]);
                if app_sel_result.is_err() {
                    defmt::warn!("Invalid app selection value: {}", pus_tc.app_data()[0]);
                }
                defmt::info!(
                    "received boot selection command with app select: {:?}",
                    app_sel_result.unwrap()
                );
                cx.local
                    .nvm
                    .write(PREFERRED_SLOT_OFFSET as usize, &[pus_tc.app_data()[0]])
                    .expect("writing to NVM failed");
                let tm = cx
                    .local
                    .verif_reporter
                    .completion_success(cx.local.src_data_buf, &request_id, 0, 0, &[])
                    .expect("completion success failed");
                write_and_send(&tm);
            }
        }
        if pus_tc.service() == PusServiceId::Test as u8 && pus_tc.subservice() == 1 {
            defmt::info!("received ping TC");
            let tm = cx
                .local
                .verif_reporter
                .completion_success(cx.local.src_data_buf, &request_id, 0, 0, &[])
                .expect("completion success failed");
            write_and_send(&tm);
        } else if pus_tc.service() == PusServiceId::MemoryManagement as u8 {
            let tm = cx
                .local
                .verif_reporter
                .step_success(
                    cx.local.src_data_buf,
                    &request_id,
                    0,
                    0,
                    &[],
                    EcssEnumU8::new(0),
                )
                .expect("step success failed");
            write_and_send(&tm);
            // Raw memory write TC
            if pus_tc.subservice() == 2 {
                let app_data = pus_tc.app_data();
                if app_data.len() < 10 {
                    defmt::warn!(
                        "app data for raw memory write is too short: {}",
                        app_data.len()
                    );
                }
                let memory_id = app_data[0];
                if memory_id != BOOT_NVM_MEMORY_ID {
                    defmt::warn!("memory ID {} not supported", memory_id);
                    // TODO: Error reporting
                    return;
                }
                let offset = u32::from_be_bytes(app_data[2..6].try_into().unwrap());
                let data_len = u32::from_be_bytes(app_data[6..10].try_into().unwrap());
                if 10 + data_len as usize > app_data.len() {
                    defmt::warn!(
                        "invalid data length {} for raw mem write detected",
                        data_len
                    );
                    // TODO: Error reporting
                    return;
                }
                let data = &app_data[10..10 + data_len as usize];
                defmt::info!("writing {} bytes at offset {} to NVM", data_len, offset);
                cx.local
                    .nvm
                    .write(offset as usize, data)
                    .expect("writing to NVM failed");
                let tm = if !cx
                    .local
                    .nvm
                    .verify(offset as usize, data)
                    .expect("NVM verification failed")
                {
                    defmt::warn!("verification of data written to NVM failed");
                    cx.local
                        .verif_reporter
                        .completion_failure(
                            cx.local.src_data_buf,
                            &request_id,
                            0,
                            0,
                            FailParams::new(&[], &EcssEnumU8::new(0), &[]),
                        )
                        .expect("completion success failed")
                } else {
                    cx.local
                        .verif_reporter
                        .completion_success(cx.local.src_data_buf, &request_id, 0, 0, &[])
                        .expect("completion success failed")
                };
                write_and_send(&tm);
                defmt::info!("NVM operation done");
            }
        }
    }

    #[task(
        priority = 1,
        local=[
            read_buf: [u8;MAX_TM_SIZE] = [0; MAX_TM_SIZE],
            encoded_buf: [u8;MAX_TM_FRAME_SIZE] = [0; MAX_TM_FRAME_SIZE],
            uart_tx,
        ],
        shared=[tm_rb]
    )]
    async fn pus_tm_tx_handler(mut cx: pus_tm_tx_handler::Context) {
        loop {
            let mut occupied_len = cx.shared.tm_rb.lock(|rb| rb.sizes.occupied_len());
            while occupied_len > 0 {
                let next_size = cx.shared.tm_rb.lock(|rb| {
                    let next_size = rb.sizes.try_pop().unwrap();
                    rb.buf.pop_slice(&mut cx.local.read_buf[0..next_size]);
                    next_size
                });
                cx.local.encoded_buf[0] = 0;
                let send_size = cobs::encode(
                    &cx.local.read_buf[0..next_size],
                    &mut cx.local.encoded_buf[1..],
                );
                cx.local.encoded_buf[send_size + 1] = 0;
                cx.local
                    .uart_tx
                    .write_all(&cx.local.encoded_buf[0..send_size + 2])
                    .unwrap();
                occupied_len -= 1;
                Mono::delay(2.millis()).await;
            }
            Mono::delay(50.millis()).await;
        }
    }
}
