#[doc = "Register `PERIPHERAL_RESET` reader"]
pub type R = crate::R<PeripheralResetSpec>;
#[doc = "Register `PERIPHERAL_RESET` writer"]
pub type W = crate::W<PeripheralResetSpec>;
#[doc = "Field `PORTA` reader - Reset PORTA"]
pub type PortaR = crate::BitReader;
#[doc = "Field `PORTA` writer - Reset PORTA"]
pub type PortaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTB` reader - Reset PORTB"]
pub type PortbR = crate::BitReader;
#[doc = "Field `PORTB` writer - Reset PORTB"]
pub type PortbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_0` reader - Reset SPI\\[0\\]"]
pub type Spi0R = crate::BitReader;
#[doc = "Field `SPI_0` writer - Reset SPI\\[0\\]"]
pub type Spi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_1` reader - Reset SPI\\[1\\]"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI_1` writer - Reset SPI\\[1\\]"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_2` reader - Reset SPI\\[2\\]"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI_2` writer - Reset SPI\\[2\\]"]
pub type Spi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_0` reader - Reset UART\\[0\\]"]
pub type Uart0R = crate::BitReader;
#[doc = "Field `UART_0` writer - Reset UART\\[0\\]"]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_1` reader - Reset UART\\[1\\]"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART_1` writer - Reset UART\\[1\\]"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_0` reader - Reset I2C\\[0\\]"]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C_0` writer - Reset I2C\\[0\\]"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_1` reader - Reset I2C\\[1\\]"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C_1` writer - Reset I2C\\[1\\]"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQSEL` reader - Reset IRQ selector"]
pub type IrqselR = crate::BitReader;
#[doc = "Field `IRQSEL` writer - Reset IRQ selector"]
pub type IrqselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCONFIG` reader - Reset IO Configuration block"]
pub type IoconfigR = crate::BitReader;
#[doc = "Field `IOCONFIG` writer - Reset IO Configuration block"]
pub type IoconfigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTILITY` reader - Reset Utility Block"]
pub type UtilityR = crate::BitReader;
#[doc = "Field `UTILITY` writer - Reset Utility Block"]
pub type UtilityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO` reader - Reset GPIO"]
pub type GpioR = crate::BitReader;
#[doc = "Field `GPIO` writer - Reset GPIO"]
pub type GpioW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset PORTA"]
    #[inline(always)]
    pub fn porta(&self) -> PortaR {
        PortaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset PORTB"]
    #[inline(always)]
    pub fn portb(&self) -> PortbR {
        PortbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset SPI\\[0\\]"]
    #[inline(always)]
    pub fn spi_0(&self) -> Spi0R {
        Spi0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset SPI\\[1\\]"]
    #[inline(always)]
    pub fn spi_1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset SPI\\[2\\]"]
    #[inline(always)]
    pub fn spi_2(&self) -> Spi2R {
        Spi2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset UART\\[0\\]"]
    #[inline(always)]
    pub fn uart_0(&self) -> Uart0R {
        Uart0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset UART\\[1\\]"]
    #[inline(always)]
    pub fn uart_1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset I2C\\[0\\]"]
    #[inline(always)]
    pub fn i2c_0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset I2C\\[1\\]"]
    #[inline(always)]
    pub fn i2c_1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset IRQ selector"]
    #[inline(always)]
    pub fn irqsel(&self) -> IrqselR {
        IrqselR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reset IO Configuration block"]
    #[inline(always)]
    pub fn ioconfig(&self) -> IoconfigR {
        IoconfigR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset Utility Block"]
    #[inline(always)]
    pub fn utility(&self) -> UtilityR {
        UtilityR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset GPIO"]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset PORTA"]
    #[inline(always)]
    pub fn porta(&mut self) -> PortaW<'_, PeripheralResetSpec> {
        PortaW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset PORTB"]
    #[inline(always)]
    pub fn portb(&mut self) -> PortbW<'_, PeripheralResetSpec> {
        PortbW::new(self, 1)
    }
    #[doc = "Bit 4 - Reset SPI\\[0\\]"]
    #[inline(always)]
    pub fn spi_0(&mut self) -> Spi0W<'_, PeripheralResetSpec> {
        Spi0W::new(self, 4)
    }
    #[doc = "Bit 5 - Reset SPI\\[1\\]"]
    #[inline(always)]
    pub fn spi_1(&mut self) -> Spi1W<'_, PeripheralResetSpec> {
        Spi1W::new(self, 5)
    }
    #[doc = "Bit 6 - Reset SPI\\[2\\]"]
    #[inline(always)]
    pub fn spi_2(&mut self) -> Spi2W<'_, PeripheralResetSpec> {
        Spi2W::new(self, 6)
    }
    #[doc = "Bit 8 - Reset UART\\[0\\]"]
    #[inline(always)]
    pub fn uart_0(&mut self) -> Uart0W<'_, PeripheralResetSpec> {
        Uart0W::new(self, 8)
    }
    #[doc = "Bit 9 - Reset UART\\[1\\]"]
    #[inline(always)]
    pub fn uart_1(&mut self) -> Uart1W<'_, PeripheralResetSpec> {
        Uart1W::new(self, 9)
    }
    #[doc = "Bit 16 - Reset I2C\\[0\\]"]
    #[inline(always)]
    pub fn i2c_0(&mut self) -> I2c0W<'_, PeripheralResetSpec> {
        I2c0W::new(self, 16)
    }
    #[doc = "Bit 17 - Reset I2C\\[1\\]"]
    #[inline(always)]
    pub fn i2c_1(&mut self) -> I2c1W<'_, PeripheralResetSpec> {
        I2c1W::new(self, 17)
    }
    #[doc = "Bit 21 - Reset IRQ selector"]
    #[inline(always)]
    pub fn irqsel(&mut self) -> IrqselW<'_, PeripheralResetSpec> {
        IrqselW::new(self, 21)
    }
    #[doc = "Bit 22 - Reset IO Configuration block"]
    #[inline(always)]
    pub fn ioconfig(&mut self) -> IoconfigW<'_, PeripheralResetSpec> {
        IoconfigW::new(self, 22)
    }
    #[doc = "Bit 23 - Reset Utility Block"]
    #[inline(always)]
    pub fn utility(&mut self) -> UtilityW<'_, PeripheralResetSpec> {
        UtilityW::new(self, 23)
    }
    #[doc = "Bit 24 - Reset GPIO"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GpioW<'_, PeripheralResetSpec> {
        GpioW::new(self, 24)
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
#[doc = "`reset()` method sets PERIPHERAL_RESET to value 0xffff_ffff"]
impl crate::Resettable for PeripheralResetSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
