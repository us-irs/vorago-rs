#[doc = "Register `PERIPHERAL_CLK_ENABLE` reader"]
pub type R = crate::R<PeripheralClkEnableSpec>;
#[doc = "Register `PERIPHERAL_CLK_ENABLE` writer"]
pub type W = crate::W<PeripheralClkEnableSpec>;
#[doc = "Field `PORTA` reader - Enable PORTA clock"]
pub type PortaR = crate::BitReader;
#[doc = "Field `PORTA` writer - Enable PORTA clock"]
pub type PortaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTB` reader - Enable PORTB clock"]
pub type PortbR = crate::BitReader;
#[doc = "Field `PORTB` writer - Enable PORTB clock"]
pub type PortbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_0` reader - Enable SPI\\[0\\] clock"]
pub type Spi0R = crate::BitReader;
#[doc = "Field `SPI_0` writer - Enable SPI\\[0\\] clock"]
pub type Spi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_1` reader - Enable SPI\\[1\\] clock"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI_1` writer - Enable SPI\\[1\\] clock"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_2` reader - Enable SPI\\[2\\] clock"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI_2` writer - Enable SPI\\[2\\] clock"]
pub type Spi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_0` reader - Enable UART\\[0\\] clock"]
pub type Uart0R = crate::BitReader;
#[doc = "Field `UART_0` writer - Enable UART\\[0\\] clock"]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_1` reader - Enable UART\\[1\\] clock"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART_1` writer - Enable UART\\[1\\] clock"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_0` reader - Enable I2C\\[0\\] clock"]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C_0` writer - Enable I2C\\[0\\] clock"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_1` reader - Enable I2C\\[1\\] clock"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C_1` writer - Enable I2C\\[1\\] clock"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQSEL` reader - Enable IRQ selector clock"]
pub type IrqselR = crate::BitReader;
#[doc = "Field `IRQSEL` writer - Enable IRQ selector clock"]
pub type IrqselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCONFIG` reader - Enable IO Configuration block clock"]
pub type IoconfigR = crate::BitReader;
#[doc = "Field `IOCONFIG` writer - Enable IO Configuration block clock"]
pub type IoconfigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTILITY` reader - Enable utility clock"]
pub type UtilityR = crate::BitReader;
#[doc = "Field `UTILITY` writer - Enable utility clock"]
pub type UtilityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO` reader - Enable GPIO clock"]
pub type GpioR = crate::BitReader;
#[doc = "Field `GPIO` writer - Enable GPIO clock"]
pub type GpioW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable PORTA clock"]
    #[inline(always)]
    pub fn porta(&self) -> PortaR {
        PortaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable PORTB clock"]
    #[inline(always)]
    pub fn portb(&self) -> PortbR {
        PortbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable SPI\\[0\\] clock"]
    #[inline(always)]
    pub fn spi_0(&self) -> Spi0R {
        Spi0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable SPI\\[1\\] clock"]
    #[inline(always)]
    pub fn spi_1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable SPI\\[2\\] clock"]
    #[inline(always)]
    pub fn spi_2(&self) -> Spi2R {
        Spi2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable UART\\[0\\] clock"]
    #[inline(always)]
    pub fn uart_0(&self) -> Uart0R {
        Uart0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable UART\\[1\\] clock"]
    #[inline(always)]
    pub fn uart_1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable I2C\\[0\\] clock"]
    #[inline(always)]
    pub fn i2c_0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable I2C\\[1\\] clock"]
    #[inline(always)]
    pub fn i2c_1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable IRQ selector clock"]
    #[inline(always)]
    pub fn irqsel(&self) -> IrqselR {
        IrqselR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable IO Configuration block clock"]
    #[inline(always)]
    pub fn ioconfig(&self) -> IoconfigR {
        IoconfigR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable utility clock"]
    #[inline(always)]
    pub fn utility(&self) -> UtilityR {
        UtilityR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable GPIO clock"]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable PORTA clock"]
    #[inline(always)]
    pub fn porta(&mut self) -> PortaW<'_, PeripheralClkEnableSpec> {
        PortaW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable PORTB clock"]
    #[inline(always)]
    pub fn portb(&mut self) -> PortbW<'_, PeripheralClkEnableSpec> {
        PortbW::new(self, 1)
    }
    #[doc = "Bit 4 - Enable SPI\\[0\\] clock"]
    #[inline(always)]
    pub fn spi_0(&mut self) -> Spi0W<'_, PeripheralClkEnableSpec> {
        Spi0W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable SPI\\[1\\] clock"]
    #[inline(always)]
    pub fn spi_1(&mut self) -> Spi1W<'_, PeripheralClkEnableSpec> {
        Spi1W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable SPI\\[2\\] clock"]
    #[inline(always)]
    pub fn spi_2(&mut self) -> Spi2W<'_, PeripheralClkEnableSpec> {
        Spi2W::new(self, 6)
    }
    #[doc = "Bit 8 - Enable UART\\[0\\] clock"]
    #[inline(always)]
    pub fn uart_0(&mut self) -> Uart0W<'_, PeripheralClkEnableSpec> {
        Uart0W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable UART\\[1\\] clock"]
    #[inline(always)]
    pub fn uart_1(&mut self) -> Uart1W<'_, PeripheralClkEnableSpec> {
        Uart1W::new(self, 9)
    }
    #[doc = "Bit 16 - Enable I2C\\[0\\] clock"]
    #[inline(always)]
    pub fn i2c_0(&mut self) -> I2c0W<'_, PeripheralClkEnableSpec> {
        I2c0W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable I2C\\[1\\] clock"]
    #[inline(always)]
    pub fn i2c_1(&mut self) -> I2c1W<'_, PeripheralClkEnableSpec> {
        I2c1W::new(self, 17)
    }
    #[doc = "Bit 21 - Enable IRQ selector clock"]
    #[inline(always)]
    pub fn irqsel(&mut self) -> IrqselW<'_, PeripheralClkEnableSpec> {
        IrqselW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable IO Configuration block clock"]
    #[inline(always)]
    pub fn ioconfig(&mut self) -> IoconfigW<'_, PeripheralClkEnableSpec> {
        IoconfigW::new(self, 22)
    }
    #[doc = "Bit 23 - Enable utility clock"]
    #[inline(always)]
    pub fn utility(&mut self) -> UtilityW<'_, PeripheralClkEnableSpec> {
        UtilityW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable GPIO clock"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GpioW<'_, PeripheralClkEnableSpec> {
        GpioW::new(self, 24)
    }
}
#[doc = "Peripheral Enable Control\n\nYou can [`read`](crate::Reg::read) this register and get [`peripheral_clk_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peripheral_clk_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeripheralClkEnableSpec;
impl crate::RegisterSpec for PeripheralClkEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peripheral_clk_enable::R`](R) reader structure"]
impl crate::Readable for PeripheralClkEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`peripheral_clk_enable::W`](W) writer structure"]
impl crate::Writable for PeripheralClkEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIPHERAL_CLK_ENABLE to value 0"]
impl crate::Resettable for PeripheralClkEnableSpec {}
