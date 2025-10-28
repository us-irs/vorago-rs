#[cfg(feature = "vor1x")]
use va108xx as pac;
#[cfg(feature = "vor4x")]
use va416xx as pac;

#[inline]
pub fn enable_peripheral_clock(clock: crate::PeripheralSelect) {
    let syscfg = unsafe { pac::Sysconfig::steal() };
    syscfg
        .peripheral_clk_enable()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << clock as u8)) });
}

#[inline]
pub fn disable_peripheral_clock(clock: crate::PeripheralSelect) {
    let syscfg = unsafe { pac::Sysconfig::steal() };
    syscfg
        .peripheral_clk_enable()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << clock as u8)) });
}

#[inline]
pub fn assert_peripheral_reset(periph_sel: crate::PeripheralSelect) {
    let syscfg = unsafe { pac::Sysconfig::steal() };
    syscfg
        .peripheral_reset()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << periph_sel as u8)) });
}

#[inline]
pub fn deassert_peripheral_reset(periph_sel: crate::PeripheralSelect) {
    let syscfg = unsafe { pac::Sysconfig::steal() };
    syscfg
        .peripheral_reset()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << periph_sel as u8)) });
}

#[inline]
pub fn reset_peripheral_for_cycles(periph_sel: crate::PeripheralSelect, cycles: usize) {
    assert_peripheral_reset(periph_sel);
    cortex_m::asm::delay(cycles as u32);
    deassert_peripheral_reset(periph_sel);
}
