//! Simple DMA example
#![no_main]
#![no_std]

// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;
use va416xx_hal::clock::ClockConfigurator;

use core::cell::Cell;

use cortex_m_rt::entry;
use critical_section::Mutex;
use embedded_hal::delay::DelayNs;
use simple_examples::peb1;
use va416xx_hal::dma::{Dma, DmaChannel, DmaConfig, DmaCtrlBlock};
use va416xx_hal::irq_router::enable_and_init_irq_router;
use va416xx_hal::pac::{self, interrupt};
use va416xx_hal::timer::CountdownTimer;

static DMA_DONE_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static DMA_ACTIVE_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

// Place the DMA control block into SRAM1 statically. This section needs to be defined in
// memory.x
#[link_section = ".sram1"]
static mut DMA_CTRL_BLOCK: DmaCtrlBlock = DmaCtrlBlock::new();

// We can use statically allocated buffers for DMA transfers as well, and we can also place
// those into SRAM1.
#[link_section = ".sram1"]
static mut DMA_SRC_BUF: [u16; 36] = [0; 36];
#[link_section = ".sram1"]
static mut DMA_DEST_BUF: [u16; 36] = [0; 36];

#[entry]
fn main() -> ! {
    defmt::println!("VA416xx DMA example");

    let dp = pac::Peripherals::take().unwrap();
    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze()
        .unwrap();
    enable_and_init_irq_router();
    // Safety: The DMA control block has an alignment rule of 128 and we constructed it directly
    // statically.
    let dma = Dma::new(
        dp.dma,
        DmaConfig::default(),
        core::ptr::addr_of_mut!(DMA_CTRL_BLOCK),
    )
    .expect("error creating DMA");
    let (mut dma0, _, _, _) = dma.split();
    let mut delay_ms = CountdownTimer::new(dp.tim0, &clocks);
    let mut src_buf_8_bit: [u8; 65] = [0; 65];
    let mut dest_buf_8_bit: [u8; 65] = [0; 65];
    let mut src_buf_32_bit: [u32; 17] = [0; 17];
    let mut dest_buf_32_bit: [u32; 17] = [0; 17];
    loop {
        // This example uses stack-allocated buffers.
        transfer_example_8_bit(
            &mut src_buf_8_bit,
            &mut dest_buf_8_bit,
            &mut dma0,
            &mut delay_ms,
        );
        delay_ms.delay_ms(500);
        // This example uses statically allocated buffers.
        transfer_example_16_bit(&mut dma0, &mut delay_ms);
        delay_ms.delay_ms(500);
        transfer_example_32_bit(
            &mut src_buf_32_bit,
            &mut dest_buf_32_bit,
            &mut dma0,
            &mut delay_ms,
        );
        delay_ms.delay_ms(500);
    }
}

fn transfer_example_8_bit(
    src_buf: &mut [u8; 65],
    dest_buf: &mut [u8; 65],
    dma0: &mut DmaChannel,
    delay: &mut CountdownTimer,
) {
    (0..64).for_each(|i| {
        src_buf[i] = i as u8;
    });
    critical_section::with(|cs| {
        DMA_DONE_FLAG.borrow(cs).set(false);
    });
    critical_section::with(|cs| {
        DMA_ACTIVE_FLAG.borrow(cs).set(false);
    });
    // Safety: The source and destination buffer are valid for the duration of the DMA transfer.
    unsafe {
        dma0.prepare_mem_to_mem_transfer_8_bit(src_buf, dest_buf)
            .expect("error preparing transfer");
    }
    // Enable all interrupts.
    // Safety: Not using mask based critical sections.
    unsafe {
        dma0.enable_done_interrupt();
        dma0.enable_active_interrupt();
    };
    // Enable the individual channel.
    dma0.enable();
    // We still need to manually trigger the DMA request.
    dma0.trigger_with_sw_request();
    // Use polling for completion status.
    loop {
        let mut dma_done = false;
        critical_section::with(|cs| {
            if DMA_ACTIVE_FLAG.borrow(cs).get() {
                defmt::info!("DMA0 is active with 8 bit transfer");
                DMA_ACTIVE_FLAG.borrow(cs).set(false);
            }
            if DMA_DONE_FLAG.borrow(cs).get() {
                dma_done = true;
            }
        });
        if dma_done {
            defmt::info!("8-bit transfer done");
            break;
        }
        delay.delay_ms(1);
    }
    (0..64).for_each(|i| {
        assert_eq!(dest_buf[i], i as u8);
    });
    // Sentinel value, should be 0.
    assert_eq!(dest_buf[64], 0);
    dest_buf.fill(0);
}

fn transfer_example_16_bit(dma0: &mut DmaChannel, delay_ms: &mut CountdownTimer) {
    let dest_buf_ref = unsafe { &mut *core::ptr::addr_of_mut!(DMA_DEST_BUF[0..33]) };
    unsafe {
        // Set values scaled from 0 to 65535 to verify this is really a 16-bit transfer.
        (0..32).for_each(|i| {
            DMA_SRC_BUF[i] = (i as u32 * u16::MAX as u32 / (dest_buf_ref.len() as u32 - 1)) as u16;
        });
    }
    critical_section::with(|cs| {
        DMA_DONE_FLAG.borrow(cs).set(false);
    });
    critical_section::with(|cs| {
        DMA_ACTIVE_FLAG.borrow(cs).set(false);
    });
    // Safety: The source and destination buffer are valid for the duration of the DMA transfer.
    unsafe {
        dma0.prepare_mem_to_mem_transfer_16_bit(
            &*core::ptr::addr_of!(DMA_SRC_BUF[0..32]),
            &mut dest_buf_ref[0..32],
        )
        .expect("error preparing transfer");
    }
    // Enable all interrupts.
    // Safety: Not using mask based critical sections.
    unsafe {
        dma0.enable_done_interrupt();
        dma0.enable_active_interrupt();
    };
    // Enable the individual channel.
    dma0.enable();
    // We still need to manually trigger the DMA request.
    dma0.trigger_with_sw_request();
    // Use polling for completion status.
    loop {
        let mut dma_done = false;
        critical_section::with(|cs| {
            if DMA_ACTIVE_FLAG.borrow(cs).get() {
                defmt::info!("DMA0 is active with 16-bit transfer");
                DMA_ACTIVE_FLAG.borrow(cs).set(false);
            }
            if DMA_DONE_FLAG.borrow(cs).get() {
                dma_done = true;
            }
        });
        if dma_done {
            defmt::info!("16-bit transfer done");
            break;
        }
        delay_ms.delay_ms(1);
    }
    (0..32).for_each(|i| {
        assert_eq!(
            dest_buf_ref[i],
            (i as u32 * u16::MAX as u32 / (dest_buf_ref.len() as u32 - 1)) as u16
        );
    });
    // Sentinel value, should be 0.
    assert_eq!(dest_buf_ref[32], 0);
    dest_buf_ref.fill(0);
}

fn transfer_example_32_bit(
    src_buf: &mut [u32; 17],
    dest_buf: &mut [u32; 17],
    dma0: &mut DmaChannel,
    delay_ms: &mut CountdownTimer,
) {
    // Set values scaled from 0 to 65535 to verify this is really a 16-bit transfer.
    (0..16).for_each(|i| {
        src_buf[i] = (i as u64 * u32::MAX as u64 / (src_buf.len() - 1) as u64) as u32;
    });
    critical_section::with(|cs| {
        DMA_DONE_FLAG.borrow(cs).set(false);
    });
    critical_section::with(|cs| {
        DMA_ACTIVE_FLAG.borrow(cs).set(false);
    });
    // Safety: The source and destination buffer are valid for the duration of the DMA transfer.
    unsafe {
        dma0.prepare_mem_to_mem_transfer_32_bit(src_buf, dest_buf)
            .expect("error preparing transfer");
    }
    // Enable all interrupts.
    // Safety: Not using mask based critical sections.
    unsafe {
        dma0.enable_done_interrupt();
        dma0.enable_active_interrupt();
    };
    // Enable the individual channel.
    dma0.enable();
    // We still need to manually trigger the DMA request.
    dma0.trigger_with_sw_request();
    // Use polling for completion status.
    loop {
        let mut dma_done = false;
        critical_section::with(|cs| {
            if DMA_ACTIVE_FLAG.borrow(cs).get() {
                defmt::info!("DMA0 is active with 32-bit transfer");
                DMA_ACTIVE_FLAG.borrow(cs).set(false);
            }
            if DMA_DONE_FLAG.borrow(cs).get() {
                dma_done = true;
            }
        });
        if dma_done {
            defmt::info!("32-bit transfer done");
            break;
        }
        delay_ms.delay_ms(1);
    }
    (0..16).for_each(|i| {
        assert_eq!(
            dest_buf[i],
            (i as u64 * u32::MAX as u64 / (src_buf.len() - 1) as u64) as u32
        );
    });
    // Sentinel value, should be 0.
    assert_eq!(dest_buf[16], 0);
    dest_buf.fill(0);
}

#[interrupt]
#[allow(non_snake_case)]
fn DMA_DONE0() {
    // Notify the main loop that the DMA transfer is finished.
    critical_section::with(|cs| {
        DMA_DONE_FLAG.borrow(cs).set(true);
    });
}

#[interrupt]
#[allow(non_snake_case)]
fn DMA_ACTIVE0() {
    // Notify the main loop that the DMA 0 is active now.
    critical_section::with(|cs| {
        DMA_ACTIVE_FLAG.borrow(cs).set(true);
    });
}
