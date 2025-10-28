#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmasel0: Dmasel0,
    dmasel1: Dmasel1,
    dmasel2: Dmasel2,
    dmasel3: Dmasel3,
    dmattsel: Dmattsel,
    adcsel: Adcsel,
    dacsel0: Dacsel0,
    dacsel1: Dacsel1,
    irq_out0: IrqOut0,
    irq_out1: IrqOut1,
    irq_out2: IrqOut2,
    irq_out3: IrqOut3,
    irq_out4: IrqOut4,
    irq_out5: IrqOut5,
    _reserved14: [u8; 0x0fc4],
    perid: Perid,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt select for DMA channel 0"]
    #[inline(always)]
    pub const fn dmasel0(&self) -> &Dmasel0 {
        &self.dmasel0
    }
    #[doc = "0x04 - Interrupt select for DMA channel 1"]
    #[inline(always)]
    pub const fn dmasel1(&self) -> &Dmasel1 {
        &self.dmasel1
    }
    #[doc = "0x08 - Interrupt select for DMA channel 2"]
    #[inline(always)]
    pub const fn dmasel2(&self) -> &Dmasel2 {
        &self.dmasel2
    }
    #[doc = "0x0c - Interrupt select for DMA channel 3"]
    #[inline(always)]
    pub const fn dmasel3(&self) -> &Dmasel3 {
        &self.dmasel3
    }
    #[doc = "0x10 - Trigger select for the DMA channels"]
    #[inline(always)]
    pub const fn dmattsel(&self) -> &Dmattsel {
        &self.dmattsel
    }
    #[doc = "0x14 - Interrupt select for ADC"]
    #[inline(always)]
    pub const fn adcsel(&self) -> &Adcsel {
        &self.adcsel
    }
    #[doc = "0x18 - Interrupt select for DAC0"]
    #[inline(always)]
    pub const fn dacsel0(&self) -> &Dacsel0 {
        &self.dacsel0
    }
    #[doc = "0x1c - Interrupt select for DAC1"]
    #[inline(always)]
    pub const fn dacsel1(&self) -> &Dacsel1 {
        &self.dacsel1
    }
    #[doc = "0x20 - DEBUG IRQ_OUT\\[31:0\\]"]
    #[inline(always)]
    pub const fn irq_out0(&self) -> &IrqOut0 {
        &self.irq_out0
    }
    #[doc = "0x24 - DEBUG IRQ_OUT\\[63:32\\]"]
    #[inline(always)]
    pub const fn irq_out1(&self) -> &IrqOut1 {
        &self.irq_out1
    }
    #[doc = "0x28 - DEBUG IRQ_OUT\\[95:64\\]"]
    #[inline(always)]
    pub const fn irq_out2(&self) -> &IrqOut2 {
        &self.irq_out2
    }
    #[doc = "0x2c - DEBUG IRQ_OUT\\[127:96\\]"]
    #[inline(always)]
    pub const fn irq_out3(&self) -> &IrqOut3 {
        &self.irq_out3
    }
    #[doc = "0x30 - DEBUG IRQ_OUT\\[159:128\\]"]
    #[inline(always)]
    pub const fn irq_out4(&self) -> &IrqOut4 {
        &self.irq_out4
    }
    #[doc = "0x34 - DEBUG IRQ_OUT\\[179:160\\]"]
    #[inline(always)]
    pub const fn irq_out5(&self) -> &IrqOut5 {
        &self.irq_out5
    }
    #[doc = "0xffc - Peripheral ID Register"]
    #[inline(always)]
    pub const fn perid(&self) -> &Perid {
        &self.perid
    }
}
#[doc = "DMASEL0 (rw) register accessor: Interrupt select for DMA channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasel0`] module"]
#[doc(alias = "DMASEL0")]
pub type Dmasel0 = crate::Reg<dmasel0::Dmasel0Spec>;
#[doc = "Interrupt select for DMA channel 0"]
pub mod dmasel0;
#[doc = "DMASEL1 (rw) register accessor: Interrupt select for DMA channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasel1`] module"]
#[doc(alias = "DMASEL1")]
pub type Dmasel1 = crate::Reg<dmasel1::Dmasel1Spec>;
#[doc = "Interrupt select for DMA channel 1"]
pub mod dmasel1;
#[doc = "DMASEL2 (rw) register accessor: Interrupt select for DMA channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasel2`] module"]
#[doc(alias = "DMASEL2")]
pub type Dmasel2 = crate::Reg<dmasel2::Dmasel2Spec>;
#[doc = "Interrupt select for DMA channel 2"]
pub mod dmasel2;
#[doc = "DMASEL3 (rw) register accessor: Interrupt select for DMA channel 3\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasel3`] module"]
#[doc(alias = "DMASEL3")]
pub type Dmasel3 = crate::Reg<dmasel3::Dmasel3Spec>;
#[doc = "Interrupt select for DMA channel 3"]
pub mod dmasel3;
#[doc = "DMATTSEL (rw) register accessor: Trigger select for the DMA channels\n\nYou can [`read`](crate::Reg::read) this register and get [`dmattsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmattsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmattsel`] module"]
#[doc(alias = "DMATTSEL")]
pub type Dmattsel = crate::Reg<dmattsel::DmattselSpec>;
#[doc = "Trigger select for the DMA channels"]
pub mod dmattsel;
#[doc = "ADCSEL (rw) register accessor: Interrupt select for ADC\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsel`] module"]
#[doc(alias = "ADCSEL")]
pub type Adcsel = crate::Reg<adcsel::AdcselSpec>;
#[doc = "Interrupt select for ADC"]
pub mod adcsel;
#[doc = "DACSEL0 (rw) register accessor: Interrupt select for DAC0\n\nYou can [`read`](crate::Reg::read) this register and get [`dacsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacsel0`] module"]
#[doc(alias = "DACSEL0")]
pub type Dacsel0 = crate::Reg<dacsel0::Dacsel0Spec>;
#[doc = "Interrupt select for DAC0"]
pub mod dacsel0;
#[doc = "DACSEL1 (rw) register accessor: Interrupt select for DAC1\n\nYou can [`read`](crate::Reg::read) this register and get [`dacsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacsel1`] module"]
#[doc(alias = "DACSEL1")]
pub type Dacsel1 = crate::Reg<dacsel1::Dacsel1Spec>;
#[doc = "Interrupt select for DAC1"]
pub mod dacsel1;
#[doc = "IRQ_OUT0 (r) register accessor: DEBUG IRQ_OUT\\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_out0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_out0`] module"]
#[doc(alias = "IRQ_OUT0")]
pub type IrqOut0 = crate::Reg<irq_out0::IrqOut0Spec>;
#[doc = "DEBUG IRQ_OUT\\[31:0\\]"]
pub mod irq_out0;
#[doc = "IRQ_OUT1 (r) register accessor: DEBUG IRQ_OUT\\[63:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_out1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_out1`] module"]
#[doc(alias = "IRQ_OUT1")]
pub type IrqOut1 = crate::Reg<irq_out1::IrqOut1Spec>;
#[doc = "DEBUG IRQ_OUT\\[63:32\\]"]
pub mod irq_out1;
#[doc = "IRQ_OUT2 (r) register accessor: DEBUG IRQ_OUT\\[95:64\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_out2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_out2`] module"]
#[doc(alias = "IRQ_OUT2")]
pub type IrqOut2 = crate::Reg<irq_out2::IrqOut2Spec>;
#[doc = "DEBUG IRQ_OUT\\[95:64\\]"]
pub mod irq_out2;
#[doc = "IRQ_OUT3 (r) register accessor: DEBUG IRQ_OUT\\[127:96\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_out3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_out3`] module"]
#[doc(alias = "IRQ_OUT3")]
pub type IrqOut3 = crate::Reg<irq_out3::IrqOut3Spec>;
#[doc = "DEBUG IRQ_OUT\\[127:96\\]"]
pub mod irq_out3;
#[doc = "IRQ_OUT4 (r) register accessor: DEBUG IRQ_OUT\\[159:128\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_out4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_out4`] module"]
#[doc(alias = "IRQ_OUT4")]
pub type IrqOut4 = crate::Reg<irq_out4::IrqOut4Spec>;
#[doc = "DEBUG IRQ_OUT\\[159:128\\]"]
pub mod irq_out4;
#[doc = "IRQ_OUT5 (r) register accessor: DEBUG IRQ_OUT\\[179:160\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_out5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_out5`] module"]
#[doc(alias = "IRQ_OUT5")]
pub type IrqOut5 = crate::Reg<irq_out5::IrqOut5Spec>;
#[doc = "DEBUG IRQ_OUT\\[179:160\\]"]
pub mod irq_out5;
#[doc = "PERID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perid`] module"]
#[doc(alias = "PERID")]
pub type Perid = crate::Reg<perid::PeridSpec>;
#[doc = "Peripheral ID Register"]
pub mod perid;
