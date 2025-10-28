//! Vorago bootloader which can boot from two images.
#![no_main]
#![no_std]
use bootloader::NvmInterface;
use cortex_m_rt::entry;
use crc::{Crc, CRC_16_IBM_3740};
use embedded_hal::delay::DelayNs;
use num_enum::TryFromPrimitive;
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;
use va108xx_hal::{pac, spi::SpiClockConfig, time::Hertz, timer::CountdownTimer};
use vorago_reb1::m95m01::M95M01;

// Useful for debugging and see what the bootloader is doing. Enabled currently, because
// the binary stays small enough.
const DEFMT_PRINTOUT: bool = true;
const DEBUG_PRINTOUTS: bool = true;
// Small delay, allows RTT printout to catch up.
const BOOT_DELAY_MS: u32 = 2000;

// Dangerous option! An image with this option set to true will flash itself from RAM directly
// into the NVM. This can be used as a recovery option from a direct RAM flash to fix the NVM
// boot process. Please note that this will flash an image which will also always perform the
// self-flash itself. It is recommended that you use a tool like probe-rs, Keil IDE, or a flash
// loader to boot a bootloader without this feature.
const FLASH_SELF: bool = false;

// Register definitions for Cortex-M0 SCB register.
pub const SCB_AIRCR_VECTKEY_POS: u32 = 16;
pub const SCB_AIRCR_VECTKEY_MSK: u32 = 0xFFFF << SCB_AIRCR_VECTKEY_POS;

pub const SCB_AIRCR_SYSRESETREQ_POS: u32 = 2;
pub const SCB_AIRCR_SYSRESETREQ_MSK: u32 = 1 << SCB_AIRCR_SYSRESETREQ_POS;

const CLOCK_FREQ: Hertz = Hertz::from_raw(50_000_000);

// Important bootloader addresses and offsets, vector table information.

const NVM_SIZE: u32 = 0x20000;
const BOOTLOADER_START_ADDR: u32 = 0x0;
const BOOTLOADER_CRC_ADDR: u32 = BOOTLOADER_END_ADDR - 2;
// This is also the maximum size of the bootloader.
const BOOTLOADER_END_ADDR: u32 = 0x3000;
const APP_A_START_ADDR: u32 = BOOTLOADER_END_ADDR;
// 0x117F8
const APP_A_SIZE_ADDR: u32 = APP_A_END_ADDR - 8;
// Four bytes reserved, even when only 2 byte CRC is used. Leaves flexibility to switch to CRC32.
// 0x117FC
const APP_A_CRC_ADDR: u32 = APP_A_END_ADDR - 4;
// 0x11800
pub const APP_A_END_ADDR: u32 = APP_A_START_ADDR + APP_IMG_SZ;
// The actual size of the image which is relevant for CRC calculation.
const APP_B_START_ADDR: u32 = APP_A_END_ADDR;
// The actual size of the image which is relevant for CRC calculation.
// 0x1FFF8
const APP_B_SIZE_ADDR: u32 = APP_B_END_ADDR - 8;
// Four bytes reserved, even when only 2 byte CRC is used. Leaves flexibility to switch to CRC32.
// 0x1FFFC
const APP_B_CRC_ADDR: u32 = APP_B_END_ADDR - 4;
// 0x20000. 8 bytes at end of EEPROM reserved for preferred image parameter. This reserved
// size should be a multiple of 8 due to alignment requirements.
pub const APP_B_END_ADDR: u32 = NVM_SIZE - 8;
pub const APP_IMG_SZ: u32 = (APP_B_END_ADDR - APP_A_START_ADDR) / 2;

static_assertions::const_assert!((APP_B_END_ADDR - BOOTLOADER_END_ADDR) % 2 == 0);

pub const VECTOR_TABLE_OFFSET: u32 = 0x0;
pub const VECTOR_TABLE_LEN: u32 = 0xC0;
pub const RESET_VECTOR_OFFSET: u32 = 0x4;
pub const PREFERRED_SLOT_OFFSET: u32 = 0x20000 - 1;

const CRC_ALGO: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_3740);

#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive, defmt::Format)]
#[repr(u8)]
enum AppSel {
    A = 0,
    B = 1,
}

pub struct NvmWrapper(pub M95M01);

// Newtype pattern. We could now more easily swap the used NVM type.
impl NvmInterface for NvmWrapper {
    fn write(&mut self, address: usize, data: &[u8]) -> Result<(), core::convert::Infallible> {
        self.0.write(address, data)
    }

    fn read(&mut self, address: usize, buf: &mut [u8]) -> Result<(), core::convert::Infallible> {
        self.0.read(address, buf)
    }

    fn verify(&mut self, address: usize, data: &[u8]) -> Result<bool, core::convert::Infallible> {
        self.0.verify(address, data)
    }
}

#[entry]
fn main() -> ! {
    if DEFMT_PRINTOUT {
        defmt::println!("-- VA108xx bootloader --");
    }
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut timer = CountdownTimer::new(dp.tim0, CLOCK_FREQ);

    let clk_config = SpiClockConfig::new(2, 4);
    let mut nvm = M95M01::new(dp.spic, clk_config);

    if FLASH_SELF {
        let mut first_four_bytes: [u8; 4] = [0; 4];
        read_four_bytes_at_addr_zero(&mut first_four_bytes);
        let bootloader_data = {
            unsafe {
                &*core::ptr::slice_from_raw_parts(
                    (BOOTLOADER_START_ADDR + 4) as *const u8,
                    (BOOTLOADER_END_ADDR - BOOTLOADER_START_ADDR - 6) as usize,
                )
            }
        };
        let mut digest = CRC_ALGO.digest();
        digest.update(&first_four_bytes);
        digest.update(bootloader_data);
        let bootloader_crc = digest.finalize();

        nvm.write(0x0, &first_four_bytes)
            .expect("writing to NVM failed");
        nvm.write(0x4, bootloader_data)
            .expect("writing to NVM failed");
        if let Err(e) = nvm.verify(0x0, &first_four_bytes) {
            if DEFMT_PRINTOUT {
                defmt::error!("verification of self-flash to NVM failed: {:?}", e);
            }
        }
        if let Err(e) = nvm.verify(0x4, bootloader_data) {
            if DEFMT_PRINTOUT {
                defmt::error!("verification of self-flash to NVM failed: {:?}", e);
            }
        }

        nvm.write(BOOTLOADER_CRC_ADDR as usize, &bootloader_crc.to_be_bytes())
            .expect("writing CRC failed");
        if let Err(e) = nvm.verify(BOOTLOADER_CRC_ADDR as usize, &bootloader_crc.to_be_bytes()) {
            if DEFMT_PRINTOUT {
                defmt::error!(
                    "error: CRC verification for bootloader self-flash failed: {:?}",
                    e
                );
            }
        }
    }

    let mut nvm = NvmWrapper(nvm);

    // Check bootloader's CRC (and write it if blank)
    check_own_crc(&dp.sysconfig, &cp, &mut nvm, &mut timer);

    let mut preferred_app_raw = [0; 1];
    nvm.read(PREFERRED_SLOT_OFFSET as usize, &mut preferred_app_raw)
        .expect("reading preferred slot failed");
    let preferred_app = AppSel::try_from(preferred_app_raw[0]).unwrap_or(AppSel::A);
    let other_app = if preferred_app == AppSel::A {
        AppSel::B
    } else {
        AppSel::A
    };

    if check_app_crc(preferred_app) {
        boot_app(&dp.sysconfig, &cp, preferred_app, &mut timer)
    } else if check_app_crc(other_app) {
        boot_app(&dp.sysconfig, &cp, other_app, &mut timer)
    } else {
        if DEBUG_PRINTOUTS && DEFMT_PRINTOUT {
            defmt::error!("both images corrupt! booting image A");
        }
        // TODO: Shift a CCSDS packet out to inform host/OBC about image corruption.
        // Both images seem to be corrupt. Boot default image A.
        boot_app(&dp.sysconfig, &cp, AppSel::A, &mut timer)
    }
}

fn check_own_crc(
    sysconfig: &pac::Sysconfig,
    cp: &cortex_m::Peripherals,
    nvm: &mut NvmWrapper,
    timer: &mut CountdownTimer,
) {
    let crc_exp = unsafe { (BOOTLOADER_CRC_ADDR as *const u16).read_unaligned().to_be() };
    // I'd prefer to use [core::slice::from_raw_parts], but that is problematic
    // because the address of the bootloader is 0x0, so the NULL check fails and the functions
    // panics.
    let mut first_four_bytes: [u8; 4] = [0; 4];
    read_four_bytes_at_addr_zero(&mut first_four_bytes);
    let mut digest = CRC_ALGO.digest();
    digest.update(&first_four_bytes);
    digest.update(unsafe {
        &*core::ptr::slice_from_raw_parts(
            (BOOTLOADER_START_ADDR + 4) as *const u8,
            (BOOTLOADER_END_ADDR - BOOTLOADER_START_ADDR - 6) as usize,
        )
    });
    let crc_calc = digest.finalize();
    if crc_exp == 0x0000 || crc_exp == 0xffff {
        if DEBUG_PRINTOUTS && DEFMT_PRINTOUT {
            defmt::info!("BL CRC blank - prog new CRC");
        }
        // Blank CRC, write it to NVM.
        nvm.write(BOOTLOADER_CRC_ADDR as usize, &crc_calc.to_be_bytes())
            .expect("writing CRC failed");
        // The Vorago bootloader resets here. I am not sure why this is done but I think it is
        // necessary because somehow the boot will not work if we just continue as usual.
        // cortex_m::peripheral::SCB::sys_reset();
    } else if crc_exp != crc_calc {
        // Bootloader is corrupted. Try to run App A.
        if DEBUG_PRINTOUTS && DEFMT_PRINTOUT {
            defmt::warn!(
                "bootloader CRC corrupt, read {} and expected {}. booting image A immediately",
                crc_calc,
                crc_exp
            );
        }
        // TODO: Shift out minimal CCSDS frame to notify about bootloader corruption.
        boot_app(sysconfig, cp, AppSel::A, timer);
    }
}

// Reading from address 0x0 is problematic in Rust.
// See https://users.rust-lang.org/t/reading-from-physical-address-0x0/117408/5.
// This solution falls back to assembler to deal with this.
fn read_four_bytes_at_addr_zero(buf: &mut [u8; 4]) {
    unsafe {
        core::arch::asm!(
            "ldr r0, [{0}]",    // Load 4 bytes from src into r0 register
            "str r0, [{1}]",    // Store r0 register into first_four_bytes
            in(reg) BOOTLOADER_START_ADDR as *const u8,         // Input: src pointer (0x0)
            in(reg) buf as *mut [u8; 4],  // Input: destination pointer
        );
    }
}
fn check_app_crc(app_sel: AppSel) -> bool {
    if DEBUG_PRINTOUTS && DEFMT_PRINTOUT {
        defmt::info!("Checking image {:?}", app_sel);
    }
    if app_sel == AppSel::A {
        check_app_given_addr(APP_A_CRC_ADDR, APP_A_START_ADDR, APP_A_SIZE_ADDR)
    } else {
        check_app_given_addr(APP_B_CRC_ADDR, APP_B_START_ADDR, APP_B_SIZE_ADDR)
    }
}

fn check_app_given_addr(crc_addr: u32, start_addr: u32, image_size_addr: u32) -> bool {
    let crc_exp = unsafe { (crc_addr as *const u16).read_unaligned().to_be() };
    let image_size = unsafe { (image_size_addr as *const u32).read_unaligned().to_be() };
    // Sanity check.
    if image_size > APP_A_END_ADDR - APP_A_START_ADDR - 8 {
        if DEFMT_PRINTOUT {
            defmt::error!("detected invalid app size {}", image_size);
        }
        return false;
    }
    let crc_calc = CRC_ALGO.checksum(unsafe {
        core::slice::from_raw_parts(start_addr as *const u8, image_size as usize)
    });
    if crc_calc == crc_exp {
        return true;
    }
    false
}

// The boot works by copying the interrupt vector table (IVT) of the respective app to the
// base address in code RAM (0x0) and then performing a soft reset.
fn boot_app(
    syscfg: &pac::Sysconfig,
    cp: &cortex_m::Peripherals,
    app_sel: AppSel,
    timer: &mut CountdownTimer,
) -> ! {
    if DEBUG_PRINTOUTS && DEFMT_PRINTOUT {
        defmt::info!("booting app {:?}", app_sel);
    }
    timer.delay_ms(BOOT_DELAY_MS);

    // Clear all interrupts set.
    unsafe {
        cp.NVIC.icer[0].write(0xFFFFFFFF);
        cp.NVIC.icpr[0].write(0xFFFFFFFF);
    }
    // Disable ROM protection.
    syscfg.rom_prot().write(|w| w.wren().set_bit());
    let base_addr = if app_sel == AppSel::A {
        APP_A_START_ADDR
    } else {
        APP_B_START_ADDR
    };
    unsafe {
        // First 4 bytes done with inline assembly, writing to the physical address 0x0 can not
        // be done without it. See https://users.rust-lang.org/t/reading-from-physical-address-0x0/117408/2.
        let first_four_bytes = core::ptr::read(base_addr as *const u32);
        core::arch::asm!(
            "str {0}, [{1}]",
            in(reg)  first_four_bytes,  // Input: App vector table.
            in(reg) BOOTLOADER_START_ADDR as *mut u32,  // Input: destination pointer
        );
        core::slice::from_raw_parts_mut(
            (BOOTLOADER_START_ADDR + 4) as *mut u8,
            (VECTOR_TABLE_LEN - 4) as usize,
        )
        .copy_from_slice(core::slice::from_raw_parts(
            (base_addr + 4) as *const u8,
            (VECTOR_TABLE_LEN - 4) as usize,
        ));
    }
    // Disable re-loading from FRAM/code ROM on soft reset
    syscfg
        .rst_cntl_rom()
        .modify(|_, w| w.sysrstreq().clear_bit());

    soft_reset(cp);
}

// Soft reset based on https://github.com/ARM-software/CMSIS_6/blob/5782d6f8057906d360f4b95ec08a2354afe5c9b9/CMSIS/Core/Include/core_cm0.h#L874.
fn soft_reset(cp: &cortex_m::Peripherals) -> ! {
    // Ensure all outstanding memory accesses included buffered write are completed before reset.
    cortex_m::asm::dsb();
    unsafe {
        cp.SCB
            .aircr
            .write((0x5FA << SCB_AIRCR_VECTKEY_POS) | SCB_AIRCR_SYSRESETREQ_MSK);
    }
    // Ensure completion of memory access.
    cortex_m::asm::dsb();

    // Loop until the reset occurs.
    loop {
        cortex_m::asm::nop();
    }
}
