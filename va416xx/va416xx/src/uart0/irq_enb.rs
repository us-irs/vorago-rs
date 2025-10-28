#[doc = "Register `IRQ_ENB` reader"]
pub type R = crate::R<IrqEnbSpec>;
#[doc = "Register `IRQ_ENB` writer"]
pub type W = crate::W<IrqEnbSpec>;
#[doc = "Field `IRQ_RX` reader - RX Interrupt"]
pub type IrqRxR = crate::BitReader;
#[doc = "Field `IRQ_RX` writer - RX Interrupt"]
pub type IrqRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_RX_STATUS` reader - RX Status Interrupt"]
pub type IrqRxStatusR = crate::BitReader;
#[doc = "Field `IRQ_RX_STATUS` writer - RX Status Interrupt"]
pub type IrqRxStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_RX_TO` reader - RX Timeout Interrupt"]
pub type IrqRxToR = crate::BitReader;
#[doc = "Field `IRQ_RX_TO` writer - RX Timeout Interrupt"]
pub type IrqRxToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_TX` reader - TX Interrupt"]
pub type IrqTxR = crate::BitReader;
#[doc = "Field `IRQ_TX` writer - TX Interrupt"]
pub type IrqTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_TX_STATUS` reader - TX Status Interrupt"]
pub type IrqTxStatusR = crate::BitReader;
#[doc = "Field `IRQ_TX_STATUS` writer - TX Status Interrupt"]
pub type IrqTxStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_TX_EMPTY` reader - TX Empty Interrupt"]
pub type IrqTxEmptyR = crate::BitReader;
#[doc = "Field `IRQ_TX_EMPTY` writer - TX Empty Interrupt"]
pub type IrqTxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_TX_CTS` reader - TX CTS Change Interrupt"]
pub type IrqTxCtsR = crate::BitReader;
#[doc = "Field `IRQ_TX_CTS` writer - TX CTS Change Interrupt"]
pub type IrqTxCtsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX Interrupt"]
    #[inline(always)]
    pub fn irq_rx(&self) -> IrqRxR {
        IrqRxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX Status Interrupt"]
    #[inline(always)]
    pub fn irq_rx_status(&self) -> IrqRxStatusR {
        IrqRxStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Timeout Interrupt"]
    #[inline(always)]
    pub fn irq_rx_to(&self) -> IrqRxToR {
        IrqRxToR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TX Interrupt"]
    #[inline(always)]
    pub fn irq_tx(&self) -> IrqTxR {
        IrqTxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX Status Interrupt"]
    #[inline(always)]
    pub fn irq_tx_status(&self) -> IrqTxStatusR {
        IrqTxStatusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Empty Interrupt"]
    #[inline(always)]
    pub fn irq_tx_empty(&self) -> IrqTxEmptyR {
        IrqTxEmptyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX CTS Change Interrupt"]
    #[inline(always)]
    pub fn irq_tx_cts(&self) -> IrqTxCtsR {
        IrqTxCtsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX Interrupt"]
    #[inline(always)]
    pub fn irq_rx(&mut self) -> IrqRxW<'_, IrqEnbSpec> {
        IrqRxW::new(self, 0)
    }
    #[doc = "Bit 1 - RX Status Interrupt"]
    #[inline(always)]
    pub fn irq_rx_status(&mut self) -> IrqRxStatusW<'_, IrqEnbSpec> {
        IrqRxStatusW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Timeout Interrupt"]
    #[inline(always)]
    pub fn irq_rx_to(&mut self) -> IrqRxToW<'_, IrqEnbSpec> {
        IrqRxToW::new(self, 2)
    }
    #[doc = "Bit 4 - TX Interrupt"]
    #[inline(always)]
    pub fn irq_tx(&mut self) -> IrqTxW<'_, IrqEnbSpec> {
        IrqTxW::new(self, 4)
    }
    #[doc = "Bit 5 - TX Status Interrupt"]
    #[inline(always)]
    pub fn irq_tx_status(&mut self) -> IrqTxStatusW<'_, IrqEnbSpec> {
        IrqTxStatusW::new(self, 5)
    }
    #[doc = "Bit 6 - TX Empty Interrupt"]
    #[inline(always)]
    pub fn irq_tx_empty(&mut self) -> IrqTxEmptyW<'_, IrqEnbSpec> {
        IrqTxEmptyW::new(self, 6)
    }
    #[doc = "Bit 7 - TX CTS Change Interrupt"]
    #[inline(always)]
    pub fn irq_tx_cts(&mut self) -> IrqTxCtsW<'_, IrqEnbSpec> {
        IrqTxCtsW::new(self, 7)
    }
}
#[doc = "IRQ Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEnbSpec;
impl crate::RegisterSpec for IrqEnbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_enb::R`](R) reader structure"]
impl crate::Readable for IrqEnbSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_enb::W`](W) writer structure"]
impl crate::Writable for IrqEnbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_ENB to value 0"]
impl crate::Resettable for IrqEnbSpec {}
