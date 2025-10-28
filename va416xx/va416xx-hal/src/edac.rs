use crate::{enable_nvic_interrupt, pac};

#[inline(always)]
pub fn enable_rom_scrub(syscfg: &mut pac::Sysconfig, counter_reset: u16) {
    syscfg
        .rom_scrub()
        .write(|w| unsafe { w.bits(counter_reset as u32) });
}

#[inline(always)]
pub fn enable_ram0_scrub(syscfg: &mut pac::Sysconfig, counter_reset: u16) {
    syscfg
        .ram0_scrub()
        .write(|w| unsafe { w.bits(counter_reset as u32) });
}

#[inline(always)]
pub fn enable_ram1_scrub(syscfg: &mut pac::Sysconfig, counter_reset: u16) {
    syscfg
        .ram1_scrub()
        .write(|w| unsafe { w.bits(counter_reset as u32) });
}

/// This function enables the SBE related interrupts. The user should also provide a
/// `EDAC_SBE` ISR and use [clear_sbe_irq] inside that ISR at the very least.
#[inline(always)]
pub fn enable_sbe_irq() {
    unsafe {
        enable_nvic_interrupt(pac::Interrupt::EDAC_SBE);
    }
}

/// This function enables the SBE related interrupts. The user should also provide a
/// `EDAC_MBE` ISR and use [clear_mbe_irq] inside that ISR at the very least.
#[inline(always)]
pub fn enable_mbe_irq() {
    unsafe {
        enable_nvic_interrupt(pac::Interrupt::EDAC_MBE);
    }
}

/// This function should be called in the user provided `EDAC_SBE` interrupt-service routine
/// to clear the SBE related interrupts.
#[inline(always)]
pub fn clear_sbe_irq() {
    // Safety: This function only clears SBE related IRQs
    let syscfg = unsafe { pac::Sysconfig::steal() };
    syscfg.irq_clr().write(|w| {
        w.romsbe().set_bit();
        w.ram0sbe().set_bit();
        w.ram1sbe().set_bit()
    });
}

/// This function should be called in the user provided `EDAC_MBE` interrupt-service routine
/// to clear the MBE related interrupts.
#[inline(always)]
pub fn clear_mbe_irq() {
    // Safety: This function only clears SBE related IRQs
    let syscfg = unsafe { pac::Sysconfig::steal() };
    syscfg.irq_clr().write(|w| {
        w.rommbe().set_bit();
        w.ram0mbe().set_bit();
        w.ram1mbe().set_bit()
    });
}
