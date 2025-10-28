#[doc = "Register `PERIPHERAL_RESET` reader"]
pub type R = crate::R<PeripheralResetSpec>;
#[doc = "Register `PERIPHERAL_RESET` writer"]
pub type W = crate::W<PeripheralResetSpec>;
#[doc = "Field `SPI0` reader - Resetn of SPI0"]
pub type Spi0R = crate::BitReader;
#[doc = "Field `SPI0` writer - Resetn of SPI0"]
pub type Spi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - Resetn of SPI1"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - Resetn of SPI1"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` reader - Resetn of SPI2"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI2` writer - Resetn of SPI2"]
pub type Spi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3` reader - Resetn of SPI3"]
pub type Spi3R = crate::BitReader;
#[doc = "Field `SPI3` writer - Resetn of SPI3"]
pub type Spi3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0` reader - Resetn of UART0"]
pub type Uart0R = crate::BitReader;
#[doc = "Field `UART0` writer - Resetn of UART0"]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - Resetn of UART1"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - Resetn of UART1"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2` reader - Resetn of UART2"]
pub type Uart2R = crate::BitReader;
#[doc = "Field `UART2` writer - Resetn of UART2"]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0` reader - Resetn of I2C0"]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C0` writer - Resetn of I2C0"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - Resetn of I2C1"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - Resetn of I2C1"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - Resetn of I2C2"]
pub type I2c2R = crate::BitReader;
#[doc = "Field `I2C2` writer - Resetn of I2C2"]
pub type I2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN0` reader - Resetn of CAN0"]
pub type Can0R = crate::BitReader;
#[doc = "Field `CAN0` writer - Resetn of CAN0"]
pub type Can0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1` reader - Resetn of CAN1"]
pub type Can1R = crate::BitReader;
#[doc = "Field `CAN1` writer - Resetn of CAN1"]
pub type Can1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRNG` reader - Resetn of TRNG"]
pub type TrngR = crate::BitReader;
#[doc = "Field `TRNG` writer - Resetn of TRNG"]
pub type TrngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC` reader - Resetn of ADC"]
pub type AdcR = crate::BitReader;
#[doc = "Field `ADC` writer - Resetn of ADC"]
pub type AdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC` reader - Resetn of DAC"]
pub type DacR = crate::BitReader;
#[doc = "Field `DAC` writer - Resetn of DAC"]
pub type DacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - Resetn of DMA"]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - Resetn of DMA"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBI` reader - Resetn of EBI"]
pub type EbiR = crate::BitReader;
#[doc = "Field `EBI` writer - Resetn of EBI"]
pub type EbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH` reader - Resetn of Ethernet"]
pub type EthR = crate::BitReader;
#[doc = "Field `ETH` writer - Resetn of Ethernet"]
pub type EthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPW` reader - Resetn of SpaceWire"]
pub type SpwR = crate::BitReader;
#[doc = "Field `SPW` writer - Resetn of SpaceWire"]
pub type SpwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGEN` reader - RESETn of PLL in Clock Generation Module"]
pub type ClkgenR = crate::BitReader;
#[doc = "Field `CLKGEN` writer - RESETn of PLL in Clock Generation Module"]
pub type ClkgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ` reader - Resetn of IRQ Router"]
pub type IrqR = crate::BitReader;
#[doc = "Field `IRQ` writer - Resetn of IRQ Router"]
pub type IrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCONFIG` reader - Resetn of IO CONFIG"]
pub type IoconfigR = crate::BitReader;
#[doc = "Field `IOCONFIG` writer - Resetn of IO CONFIG"]
pub type IoconfigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTILITY` reader - Resetn of UTILITY peripheral"]
pub type UtilityR = crate::BitReader;
#[doc = "Field `UTILITY` writer - Resetn of UTILITY peripheral"]
pub type UtilityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG` reader - Resetn of WDOG"]
pub type WdogR = crate::BitReader;
#[doc = "Field `WDOG` writer - Resetn of WDOG"]
pub type WdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTA` reader - Resetn of PORTA"]
pub type PortaR = crate::BitReader;
#[doc = "Field `PORTA` writer - Resetn of PORTA"]
pub type PortaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTB` reader - Resetn of PORTB"]
pub type PortbR = crate::BitReader;
#[doc = "Field `PORTB` writer - Resetn of PORTB"]
pub type PortbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTC` reader - Resetn of PORTC"]
pub type PortcR = crate::BitReader;
#[doc = "Field `PORTC` writer - Resetn of PORTC"]
pub type PortcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTD` reader - Resetn of PORTD"]
pub type PortdR = crate::BitReader;
#[doc = "Field `PORTD` writer - Resetn of PORTD"]
pub type PortdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTE` reader - Resetn of PORTE"]
pub type PorteR = crate::BitReader;
#[doc = "Field `PORTE` writer - Resetn of PORTE"]
pub type PorteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTF` reader - Resetn of PORTF"]
pub type PortfR = crate::BitReader;
#[doc = "Field `PORTF` writer - Resetn of PORTF"]
pub type PortfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTG` reader - Resetn of PORTG"]
pub type PortgR = crate::BitReader;
#[doc = "Field `PORTG` writer - Resetn of PORTG"]
pub type PortgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Resetn of SPI0"]
    #[inline(always)]
    pub fn spi0(&self) -> Spi0R {
        Spi0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resetn of SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Resetn of SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> Spi2R {
        Spi2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Resetn of SPI3"]
    #[inline(always)]
    pub fn spi3(&self) -> Spi3R {
        Spi3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Resetn of UART0"]
    #[inline(always)]
    pub fn uart0(&self) -> Uart0R {
        Uart0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Resetn of UART1"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Resetn of UART2"]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Resetn of I2C0"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Resetn of I2C1"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Resetn of I2C2"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2c2R {
        I2c2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Resetn of CAN0"]
    #[inline(always)]
    pub fn can0(&self) -> Can0R {
        Can0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Resetn of CAN1"]
    #[inline(always)]
    pub fn can1(&self) -> Can1R {
        Can1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Resetn of TRNG"]
    #[inline(always)]
    pub fn trng(&self) -> TrngR {
        TrngR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Resetn of ADC"]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Resetn of DAC"]
    #[inline(always)]
    pub fn dac(&self) -> DacR {
        DacR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Resetn of DMA"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Resetn of EBI"]
    #[inline(always)]
    pub fn ebi(&self) -> EbiR {
        EbiR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Resetn of Ethernet"]
    #[inline(always)]
    pub fn eth(&self) -> EthR {
        EthR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Resetn of SpaceWire"]
    #[inline(always)]
    pub fn spw(&self) -> SpwR {
        SpwR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RESETn of PLL in Clock Generation Module"]
    #[inline(always)]
    pub fn clkgen(&self) -> ClkgenR {
        ClkgenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Resetn of IRQ Router"]
    #[inline(always)]
    pub fn irq(&self) -> IrqR {
        IrqR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Resetn of IO CONFIG"]
    #[inline(always)]
    pub fn ioconfig(&self) -> IoconfigR {
        IoconfigR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Resetn of UTILITY peripheral"]
    #[inline(always)]
    pub fn utility(&self) -> UtilityR {
        UtilityR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Resetn of WDOG"]
    #[inline(always)]
    pub fn wdog(&self) -> WdogR {
        WdogR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Resetn of PORTA"]
    #[inline(always)]
    pub fn porta(&self) -> PortaR {
        PortaR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Resetn of PORTB"]
    #[inline(always)]
    pub fn portb(&self) -> PortbR {
        PortbR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Resetn of PORTC"]
    #[inline(always)]
    pub fn portc(&self) -> PortcR {
        PortcR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Resetn of PORTD"]
    #[inline(always)]
    pub fn portd(&self) -> PortdR {
        PortdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Resetn of PORTE"]
    #[inline(always)]
    pub fn porte(&self) -> PorteR {
        PorteR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Resetn of PORTF"]
    #[inline(always)]
    pub fn portf(&self) -> PortfR {
        PortfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Resetn of PORTG"]
    #[inline(always)]
    pub fn portg(&self) -> PortgR {
        PortgR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Resetn of SPI0"]
    #[inline(always)]
    pub fn spi0(&mut self) -> Spi0W<'_, PeripheralResetSpec> {
        Spi0W::new(self, 0)
    }
    #[doc = "Bit 1 - Resetn of SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<'_, PeripheralResetSpec> {
        Spi1W::new(self, 1)
    }
    #[doc = "Bit 2 - Resetn of SPI2"]
    #[inline(always)]
    pub fn spi2(&mut self) -> Spi2W<'_, PeripheralResetSpec> {
        Spi2W::new(self, 2)
    }
    #[doc = "Bit 3 - Resetn of SPI3"]
    #[inline(always)]
    pub fn spi3(&mut self) -> Spi3W<'_, PeripheralResetSpec> {
        Spi3W::new(self, 3)
    }
    #[doc = "Bit 4 - Resetn of UART0"]
    #[inline(always)]
    pub fn uart0(&mut self) -> Uart0W<'_, PeripheralResetSpec> {
        Uart0W::new(self, 4)
    }
    #[doc = "Bit 5 - Resetn of UART1"]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<'_, PeripheralResetSpec> {
        Uart1W::new(self, 5)
    }
    #[doc = "Bit 6 - Resetn of UART2"]
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<'_, PeripheralResetSpec> {
        Uart2W::new(self, 6)
    }
    #[doc = "Bit 7 - Resetn of I2C0"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2c0W<'_, PeripheralResetSpec> {
        I2c0W::new(self, 7)
    }
    #[doc = "Bit 8 - Resetn of I2C1"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<'_, PeripheralResetSpec> {
        I2c1W::new(self, 8)
    }
    #[doc = "Bit 9 - Resetn of I2C2"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2c2W<'_, PeripheralResetSpec> {
        I2c2W::new(self, 9)
    }
    #[doc = "Bit 10 - Resetn of CAN0"]
    #[inline(always)]
    pub fn can0(&mut self) -> Can0W<'_, PeripheralResetSpec> {
        Can0W::new(self, 10)
    }
    #[doc = "Bit 11 - Resetn of CAN1"]
    #[inline(always)]
    pub fn can1(&mut self) -> Can1W<'_, PeripheralResetSpec> {
        Can1W::new(self, 11)
    }
    #[doc = "Bit 12 - Resetn of TRNG"]
    #[inline(always)]
    pub fn trng(&mut self) -> TrngW<'_, PeripheralResetSpec> {
        TrngW::new(self, 12)
    }
    #[doc = "Bit 13 - Resetn of ADC"]
    #[inline(always)]
    pub fn adc(&mut self) -> AdcW<'_, PeripheralResetSpec> {
        AdcW::new(self, 13)
    }
    #[doc = "Bit 14 - Resetn of DAC"]
    #[inline(always)]
    pub fn dac(&mut self) -> DacW<'_, PeripheralResetSpec> {
        DacW::new(self, 14)
    }
    #[doc = "Bit 15 - Resetn of DMA"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, PeripheralResetSpec> {
        DmaW::new(self, 15)
    }
    #[doc = "Bit 16 - Resetn of EBI"]
    #[inline(always)]
    pub fn ebi(&mut self) -> EbiW<'_, PeripheralResetSpec> {
        EbiW::new(self, 16)
    }
    #[doc = "Bit 17 - Resetn of Ethernet"]
    #[inline(always)]
    pub fn eth(&mut self) -> EthW<'_, PeripheralResetSpec> {
        EthW::new(self, 17)
    }
    #[doc = "Bit 18 - Resetn of SpaceWire"]
    #[inline(always)]
    pub fn spw(&mut self) -> SpwW<'_, PeripheralResetSpec> {
        SpwW::new(self, 18)
    }
    #[doc = "Bit 19 - RESETn of PLL in Clock Generation Module"]
    #[inline(always)]
    pub fn clkgen(&mut self) -> ClkgenW<'_, PeripheralResetSpec> {
        ClkgenW::new(self, 19)
    }
    #[doc = "Bit 20 - Resetn of IRQ Router"]
    #[inline(always)]
    pub fn irq(&mut self) -> IrqW<'_, PeripheralResetSpec> {
        IrqW::new(self, 20)
    }
    #[doc = "Bit 21 - Resetn of IO CONFIG"]
    #[inline(always)]
    pub fn ioconfig(&mut self) -> IoconfigW<'_, PeripheralResetSpec> {
        IoconfigW::new(self, 21)
    }
    #[doc = "Bit 22 - Resetn of UTILITY peripheral"]
    #[inline(always)]
    pub fn utility(&mut self) -> UtilityW<'_, PeripheralResetSpec> {
        UtilityW::new(self, 22)
    }
    #[doc = "Bit 23 - Resetn of WDOG"]
    #[inline(always)]
    pub fn wdog(&mut self) -> WdogW<'_, PeripheralResetSpec> {
        WdogW::new(self, 23)
    }
    #[doc = "Bit 24 - Resetn of PORTA"]
    #[inline(always)]
    pub fn porta(&mut self) -> PortaW<'_, PeripheralResetSpec> {
        PortaW::new(self, 24)
    }
    #[doc = "Bit 25 - Resetn of PORTB"]
    #[inline(always)]
    pub fn portb(&mut self) -> PortbW<'_, PeripheralResetSpec> {
        PortbW::new(self, 25)
    }
    #[doc = "Bit 26 - Resetn of PORTC"]
    #[inline(always)]
    pub fn portc(&mut self) -> PortcW<'_, PeripheralResetSpec> {
        PortcW::new(self, 26)
    }
    #[doc = "Bit 27 - Resetn of PORTD"]
    #[inline(always)]
    pub fn portd(&mut self) -> PortdW<'_, PeripheralResetSpec> {
        PortdW::new(self, 27)
    }
    #[doc = "Bit 28 - Resetn of PORTE"]
    #[inline(always)]
    pub fn porte(&mut self) -> PorteW<'_, PeripheralResetSpec> {
        PorteW::new(self, 28)
    }
    #[doc = "Bit 29 - Resetn of PORTF"]
    #[inline(always)]
    pub fn portf(&mut self) -> PortfW<'_, PeripheralResetSpec> {
        PortfW::new(self, 29)
    }
    #[doc = "Bit 30 - Resetn of PORTG"]
    #[inline(always)]
    pub fn portg(&mut self) -> PortgW<'_, PeripheralResetSpec> {
        PortgW::new(self, 30)
    }
}
#[doc = "Peripheral Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheral_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheral_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeripheralResetSpec;
impl crate::RegisterSpec for PeripheralResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peripheral_reset::R`](R) reader structure"]
impl crate::Readable for PeripheralResetSpec {}
#[doc = "`write(|w| ..)` method takes [`peripheral_reset::W`](W) writer structure"]
impl crate::Writable for PeripheralResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIPHERAL_RESET to value 0x7f7b_efff"]
impl crate::Resettable for PeripheralResetSpec {
    const RESET_VALUE: u32 = 0x7f7b_efff;
}
