//! Vorago flashloader which can be used to flash image A and image B via a simple
//! low-level CCSDS memory interface via a UART wire.
//!
//! This flash loader can be used after the bootloader was flashed to flash the images.
//! You can also use this as an starting application for a software update mechanism.
//!
//! Bootloader memory map
//!
//! * <0x0>     Bootloader start                         <code up to 0x3FFE bytes>
//! * <0x3FFE>  Bootloader CRC                           <halfword>
//! * <0x4000>  App image A start                        <code up to 0x1DFFC (~120K) bytes>
//! * <0x21FFC> App image A CRC check length             <halfword>
//! * <0x21FFE> App image A CRC check value              <halfword>
//! * <0x22000> App image B start                        <code up to 0x1DFFC (~120K) bytes>
//! * <0x3FFFC> App image B CRC check length             <halfword>
//! * <0x3FFFE> App image B CRC check value              <halfword>
//! * <0x40000>                                          <end>
#![no_main]
#![no_std]

use once_cell::sync::OnceCell;
use va416xx_hal::{clock::Clocks, edac, pac, time::Hertz, wdt::Wdt};

const EXTCLK_FREQ: u32 = 40_000_000;

const MAX_TC_SIZE: usize = 1024;
const MAX_TC_FRAME_SIZE: usize = cobs::max_encoding_length(MAX_TC_SIZE);

const MAX_TM_SIZE: usize = 128;
const MAX_TM_FRAME_SIZE: usize = cobs::max_encoding_length(MAX_TM_SIZE);

const UART_BAUDRATE: u32 = 115200;
const BOOT_NVM_MEMORY_ID: u8 = 1;
const RX_DEBUGGING: bool = false;
const TX_DEBUGGING: bool = false;

pub enum ActionId {
    CorruptImageA = 128,
    CorruptImageB = 129,
}
pub trait WdtInterface {
    fn feed(&self);
}

pub struct OptWdt(Option<Wdt>);

impl WdtInterface for OptWdt {
    fn feed(&self) {
        if self.0.is_some() {
            self.0.as_ref().unwrap().feed();
        }
    }
}

use ringbuf::{
    traits::{Consumer, Observer, Producer, SplitRef},
    CachingCons, StaticProd, StaticRb,
};
use static_cell::StaticCell;

// Larger buffer for TC to be able to hold the possibly large memory write packets.
const BUF_RB_SIZE_TC: usize = 2048;
const SIZES_RB_SIZE_TC: usize = 16;

const BUF_RB_SIZE_TM: usize = 512;
const SIZES_RB_SIZE_TM: usize = 16;

// Ring buffers to handling variable sized telemetry
static BUF_RB_TM: StaticCell<StaticRb<u8, BUF_RB_SIZE_TM>> = StaticCell::new();
static SIZES_RB_TM: StaticCell<StaticRb<usize, SIZES_RB_SIZE_TM>> = StaticCell::new();

// Ring buffers to handling variable sized telecommands
static BUF_RB_TC: StaticCell<StaticRb<u8, BUF_RB_SIZE_TC>> = StaticCell::new();
static SIZES_RB_TC: StaticCell<StaticRb<usize, SIZES_RB_SIZE_TC>> = StaticCell::new();

pub struct DataProducer<const BUF_SIZE: usize, const SIZES_LEN: usize> {
    pub buf_prod: StaticProd<'static, u8, BUF_SIZE>,
    pub sizes_prod: StaticProd<'static, usize, SIZES_LEN>,
}

pub struct DataConsumer<const BUF_SIZE: usize, const SIZES_LEN: usize> {
    pub buf_cons: CachingCons<&'static StaticRb<u8, BUF_SIZE>>,
    pub sizes_cons: CachingCons<&'static StaticRb<usize, SIZES_LEN>>,
}

static CLOCKS: OnceCell<Clocks> = OnceCell::new();

pub const APP_A_START_ADDR: u32 = 0x4000;
pub const APP_A_END_ADDR: u32 = 0x22000;
pub const APP_B_START_ADDR: u32 = 0x22000;
pub const APP_B_END_ADDR: u32 = 0x40000;

#[rtic::app(device = pac, dispatchers = [U1, U2, U3])]
mod app {
    use super::*;
    use cortex_m::asm;
    use embedded_io::Write;
    // Import panic provider.
    use panic_probe as _;
    // Import logger.
    use defmt_rtt as _;
    use rtic::Mutex;
    use rtic_monotonics::systick::prelude::*;
    use satrs::pus::verification::VerificationReportCreator;
    use spacepackets::ecss::PusServiceId;
    use spacepackets::ecss::{
        tc::PusTcReader, tm::PusTmCreator, EcssEnumU8, PusPacket, WritablePusPacket,
    };
    use va416xx_hal::clock::ClockConfigurator;
    use va416xx_hal::irq_router::enable_and_init_irq_router;
    use va416xx_hal::uart::InterruptContextTimeoutOrMaxSize;
    use va416xx_hal::{
        edac,
        nvm::Nvm,
        pac,
        pins::PinsG,
        uart::{self, Uart},
    };

    use crate::{setup_edac, EXTCLK_FREQ};

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
        rom_spi: Option<pac::Spi3>,
        // We handle all TM in one task.
        tm_cons: DataConsumer<BUF_RB_SIZE_TM, SIZES_RB_SIZE_TM>,
        // We consume all TC in one task.
        tc_cons: DataConsumer<BUF_RB_SIZE_TC, SIZES_RB_SIZE_TC>,
        // We produce all TC in one task.
        tc_prod: DataProducer<BUF_RB_SIZE_TC, SIZES_RB_SIZE_TC>,
        verif_reporter: VerificationReportCreator,
    }

    #[shared]
    struct Shared {
        // Having this shared allows multiple tasks to generate telemetry.
        tm_prod: DataProducer<BUF_RB_SIZE_TM, SIZES_RB_SIZE_TM>,
    }

    rtic_monotonics::systick_monotonic!(Mono, 10_000);

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        defmt::println!("-- Vorago flashloader --");
        // Initialize the systick interrupt & obtain the token to prove that we did
        // Use the external clock connected to XTAL_N.
        let clocks = ClockConfigurator::new(cx.device.clkgen)
            .xtal_n_clk_with_src_freq(Hertz::from_raw(EXTCLK_FREQ))
            .freeze()
            .unwrap();

        enable_and_init_irq_router();
        setup_edac(&mut cx.device.sysconfig);

        let gpiog = PinsG::new(cx.device.portg);

        let uart0 = Uart::new(
            cx.device.uart0,
            gpiog.pg0,
            gpiog.pg1,
            &clocks,
            Hertz::from_raw(UART_BAUDRATE).into(),
        )
        .unwrap();
        let (tx, rx) = uart0.split();

        let verif_reporter = VerificationReportCreator::new(0).unwrap();

        let (buf_prod_tm, buf_cons_tm) = BUF_RB_TM
            .init(StaticRb::<u8, BUF_RB_SIZE_TM>::default())
            .split_ref();
        let (sizes_prod_tm, sizes_cons_tm) = SIZES_RB_TM
            .init(StaticRb::<usize, SIZES_RB_SIZE_TM>::default())
            .split_ref();

        let (buf_prod_tc, buf_cons_tc) = BUF_RB_TC
            .init(StaticRb::<u8, BUF_RB_SIZE_TC>::default())
            .split_ref();
        let (sizes_prod_tc, sizes_cons_tc) = SIZES_RB_TC
            .init(StaticRb::<usize, SIZES_RB_SIZE_TC>::default())
            .split_ref();

        Mono::start(cx.core.SYST, clocks.sysclk().raw());
        CLOCKS.set(clocks).unwrap();

        let mut rx = rx.into_rx_with_irq();
        let mut rx_context = InterruptContextTimeoutOrMaxSize::new(MAX_TC_FRAME_SIZE);
        rx.read_fixed_len_or_timeout_based_using_irq(&mut rx_context)
            .expect("initiating UART RX failed");
        pus_tc_handler::spawn().unwrap();
        pus_tm_tx_handler::spawn().unwrap();
        (
            Shared {
                tm_prod: DataProducer {
                    buf_prod: buf_prod_tm,
                    sizes_prod: sizes_prod_tm,
                },
            },
            Local {
                uart_rx: rx,
                uart_tx: tx,
                rx_context,
                rom_spi: Some(cx.device.spi3),
                tm_cons: DataConsumer {
                    buf_cons: buf_cons_tm,
                    sizes_cons: sizes_cons_tm,
                },
                tc_cons: DataConsumer {
                    buf_cons: buf_cons_tc,
                    sizes_cons: sizes_cons_tc,
                },
                tc_prod: DataProducer {
                    buf_prod: buf_prod_tc,
                    sizes_prod: sizes_prod_tc,
                },
                verif_reporter,
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
        binds = UART0_RX,
        local = [
            cnt: u32 = 0,
            rx_buf: [u8; MAX_TC_FRAME_SIZE] = [0; MAX_TC_FRAME_SIZE],
            rx_context,
            uart_rx,
            tc_prod
        ],
    )]
    fn uart_rx_irq(cx: uart_rx_irq::Context) {
        match cx
            .local
            .uart_rx
            .on_interrupt_max_size_or_timeout_based(cx.local.rx_context, cx.local.rx_buf)
        {
            Ok(result) => {
                if RX_DEBUGGING {
                    defmt::info!("RX Info: {:?}", cx.local.rx_context);
                    defmt::info!("RX Result: {:?}", result);
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
                            if cx.local.tc_prod.sizes_prod.vacant_len() >= 1
                                && cx.local.tc_prod.buf_prod.vacant_len() >= decoded_size
                            {
                                // Should never fail, we checked there is enough space.
                                cx.local.tc_prod.sizes_prod.try_push(decoded_size).unwrap();
                                cx.local
                                    .tc_prod
                                    .buf_prod
                                    .push_slice(&cx.local.rx_buf[1..1 + decoded_size]);
                            } else {
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
            src_data_buf: [u8; 16] = [0; 16],
            verif_buf: [u8; 32] = [0; 32],
            tc_cons,
            rom_spi,
            verif_reporter
        ],
        shared=[tm_prod]
    )]
    async fn pus_tc_handler(mut cx: pus_tc_handler::Context) {
        loop {
            // Try to read a TC from the ring buffer.
            let packet_len = cx.local.tc_cons.sizes_cons.try_pop();
            if packet_len.is_none() {
                // Small delay, TCs might arrive very quickly.
                Mono::delay(20.millis()).await;
                continue;
            }
            let packet_len = packet_len.unwrap();
            defmt::info!("received packet with length {}", packet_len);
            assert_eq!(
                cx.local
                    .tc_cons
                    .buf_cons
                    .pop_slice(&mut cx.local.tc_buf[0..packet_len]),
                packet_len
            );
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
            cx.shared.tm_prod.lock(|prod| {
                prod.sizes_prod.try_push(tm.len_written()).unwrap();
                prod.buf_prod
                    .push_slice(&cx.local.verif_buf[0..written_size]);
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
                // Safety: We only use this for NVM handling and we only do NVM
                // handling here.
                let nvm = Nvm::new(
                    cx.local.rom_spi.take().unwrap(),
                    CLOCKS.get().as_ref().unwrap(),
                );
                let mut buf = [0u8; 4];
                nvm.read_data(base_addr + 32, &mut buf);
                buf[0] += 1;
                nvm.write_data(base_addr + 32, &buf);
                *cx.local.rom_spi = Some(nvm.release());
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
                // Safety: We only use this for NVM handling and we only do NVM
                // handling here.
                let nvm = Nvm::new(
                    cx.local.rom_spi.take().unwrap(),
                    CLOCKS.get().as_ref().unwrap(),
                );
                nvm.write_data(offset, data);
                *cx.local.rom_spi = Some(nvm.release());
                let tm = cx
                    .local
                    .verif_reporter
                    .completion_success(cx.local.src_data_buf, &request_id, 0, 0, &[])
                    .expect("completion success failed");
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
            tm_cons
        ],
        shared=[]
    )]
    async fn pus_tm_tx_handler(cx: pus_tm_tx_handler::Context) {
        loop {
            while cx.local.tm_cons.sizes_cons.occupied_len() > 0 {
                let next_size = cx.local.tm_cons.sizes_cons.try_pop().unwrap();
                cx.local
                    .tm_cons
                    .buf_cons
                    .pop_slice(&mut cx.local.read_buf[0..next_size]);
                cx.local.encoded_buf[0] = 0;
                let send_size = cobs::encode(
                    &cx.local.read_buf[0..next_size],
                    &mut cx.local.encoded_buf[1..],
                );
                cx.local.encoded_buf[send_size + 1] = 0;
                if TX_DEBUGGING {
                    defmt::debug!("UART TX: Sending data with size {}", send_size + 2);
                }
                cx.local
                    .uart_tx
                    .write_all(&cx.local.encoded_buf[0..send_size + 2])
                    .unwrap();
                Mono::delay(2.millis()).await;
            }
            Mono::delay(50.millis()).await;
        }
    }

    #[task(binds = EDAC_SBE, priority = 1)]
    fn edac_sbe_isr(_cx: edac_sbe_isr::Context) {
        // TODO: Send some command via UART for notification purposes. Also identify the problematic
        // memory.
        edac::clear_sbe_irq();
    }

    #[task(binds = EDAC_MBE, priority = 1)]
    fn edac_mbe_isr(_cx: edac_mbe_isr::Context) {
        // TODO: Send some command via UART for notification purposes.
        edac::clear_mbe_irq();
        // TODO: Reset like the vorago example?
    }

    #[task(binds = WATCHDOG, priority = 1)]
    fn watchdog_isr(_cx: watchdog_isr::Context) {
        let wdt = unsafe { pac::WatchDog::steal() };
        // Clear interrupt.
        wdt.wdogintclr().write(|w| unsafe { w.bits(1) });
    }
}

fn setup_edac(syscfg: &mut pac::Sysconfig) {
    // The scrub values are based on the Vorago provided bootloader.
    edac::enable_rom_scrub(syscfg, 125);
    edac::enable_ram0_scrub(syscfg, 1000);
    edac::enable_ram1_scrub(syscfg, 1000);
    edac::enable_sbe_irq();
    edac::enable_mbe_irq();
}
