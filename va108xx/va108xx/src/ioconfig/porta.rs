#[doc = "Register `PORTA[%s]` reader"]
pub type R = crate::R<PortaSpec>;
#[doc = "Register `PORTA[%s]` writer"]
pub type W = crate::W<PortaSpec>;
#[doc = "Input Filter Selectoin\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flttype {
    #[doc = "0: Synchronize to system clock"]
    Sync = 0,
    #[doc = "1: Direct input, no synchronization"]
    Direct = 1,
    #[doc = "2: Require 2 samples to have the same value"]
    Filter1 = 2,
    #[doc = "3: Require 3 samples to have the same value"]
    Filter2 = 3,
    #[doc = "4: Require 4 samples to have the same value"]
    Filter3 = 4,
    #[doc = "5: Require 5 samples to have the same value"]
    Filter4 = 5,
}
impl From<Flttype> for u8 {
    #[inline(always)]
    fn from(variant: Flttype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flttype {
    type Ux = u8;
}
impl crate::IsEnum for Flttype {}
#[doc = "Field `FLTTYPE` reader - Input Filter Selectoin"]
pub type FlttypeR = crate::FieldReader<Flttype>;
impl FlttypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flttype> {
        match self.bits {
            0 => Some(Flttype::Sync),
            1 => Some(Flttype::Direct),
            2 => Some(Flttype::Filter1),
            3 => Some(Flttype::Filter2),
            4 => Some(Flttype::Filter3),
            5 => Some(Flttype::Filter4),
            _ => None,
        }
    }
    #[doc = "Synchronize to system clock"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Flttype::Sync
    }
    #[doc = "Direct input, no synchronization"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == Flttype::Direct
    }
    #[doc = "Require 2 samples to have the same value"]
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        *self == Flttype::Filter1
    }
    #[doc = "Require 3 samples to have the same value"]
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == Flttype::Filter2
    }
    #[doc = "Require 4 samples to have the same value"]
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        *self == Flttype::Filter3
    }
    #[doc = "Require 5 samples to have the same value"]
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == Flttype::Filter4
    }
}
#[doc = "Field `FLTTYPE` writer - Input Filter Selectoin"]
pub type FlttypeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Flttype>;
impl<'a, REG> FlttypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Synchronize to system clock"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Flttype::Sync)
    }
    #[doc = "Direct input, no synchronization"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(Flttype::Direct)
    }
    #[doc = "Require 2 samples to have the same value"]
    #[inline(always)]
    pub fn filter1(self) -> &'a mut crate::W<REG> {
        self.variant(Flttype::Filter1)
    }
    #[doc = "Require 3 samples to have the same value"]
    #[inline(always)]
    pub fn filter2(self) -> &'a mut crate::W<REG> {
        self.variant(Flttype::Filter2)
    }
    #[doc = "Require 4 samples to have the same value"]
    #[inline(always)]
    pub fn filter3(self) -> &'a mut crate::W<REG> {
        self.variant(Flttype::Filter3)
    }
    #[doc = "Require 5 samples to have the same value"]
    #[inline(always)]
    pub fn filter4(self) -> &'a mut crate::W<REG> {
        self.variant(Flttype::Filter4)
    }
}
#[doc = "Field `FLTCLK` reader - Input Filter Clock Selection"]
pub type FltclkR = crate::FieldReader;
#[doc = "Field `FLTCLK` writer - Input Filter Clock Selection"]
pub type FltclkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INVINP` reader - Input Invert Selection"]
pub type InvinpR = crate::BitReader;
#[doc = "Field `INVINP` writer - Input Invert Selection"]
pub type InvinpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEWO` reader - Input Enable While Output enabled"]
pub type IewoR = crate::BitReader;
#[doc = "Field `IEWO` writer - Input Enable While Output enabled"]
pub type IewoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPENDRN` reader - Output Open Drain Mode"]
pub type OpendrnR = crate::BitReader;
#[doc = "Field `OPENDRN` writer - Output Open Drain Mode"]
pub type OpendrnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVOUT` reader - Output Invert Selection"]
pub type InvoutR = crate::BitReader;
#[doc = "Field `INVOUT` writer - Output Invert Selection"]
pub type InvoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLEVEL` reader - Internal Pull up/down level"]
pub type PlevelR = crate::BitReader;
#[doc = "Field `PLEVEL` writer - Internal Pull up/down level"]
pub type PlevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Enable Internal Pull up/down"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Enable Internal Pull up/down"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWOA` reader - Enable Pull when output active"]
pub type PwoaR = crate::BitReader;
#[doc = "Field `PWOA` writer - Enable Pull when output active"]
pub type PwoaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUNSEL` reader - Pin Function Selection"]
pub type FunselR = crate::FieldReader;
#[doc = "Field `FUNSEL` writer - Pin Function Selection"]
pub type FunselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IODIS` reader - IO Pin Disable"]
pub type IodisR = crate::BitReader;
#[doc = "Field `IODIS` writer - IO Pin Disable"]
pub type IodisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Input Filter Selectoin"]
    #[inline(always)]
    pub fn flttype(&self) -> FlttypeR {
        FlttypeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Input Filter Clock Selection"]
    #[inline(always)]
    pub fn fltclk(&self) -> FltclkR {
        FltclkR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Input Invert Selection"]
    #[inline(always)]
    pub fn invinp(&self) -> InvinpR {
        InvinpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input Enable While Output enabled"]
    #[inline(always)]
    pub fn iewo(&self) -> IewoR {
        IewoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Open Drain Mode"]
    #[inline(always)]
    pub fn opendrn(&self) -> OpendrnR {
        OpendrnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Invert Selection"]
    #[inline(always)]
    pub fn invout(&self) -> InvoutR {
        InvoutR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Internal Pull up/down level"]
    #[inline(always)]
    pub fn plevel(&self) -> PlevelR {
        PlevelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Internal Pull up/down"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Pull when output active"]
    #[inline(always)]
    pub fn pwoa(&self) -> PwoaR {
        PwoaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Pin Function Selection"]
    #[inline(always)]
    pub fn funsel(&self) -> FunselR {
        FunselR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - IO Pin Disable"]
    #[inline(always)]
    pub fn iodis(&self) -> IodisR {
        IodisR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input Filter Selectoin"]
    #[inline(always)]
    pub fn flttype(&mut self) -> FlttypeW<'_, PortaSpec> {
        FlttypeW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Input Filter Clock Selection"]
    #[inline(always)]
    pub fn fltclk(&mut self) -> FltclkW<'_, PortaSpec> {
        FltclkW::new(self, 3)
    }
    #[doc = "Bit 6 - Input Invert Selection"]
    #[inline(always)]
    pub fn invinp(&mut self) -> InvinpW<'_, PortaSpec> {
        InvinpW::new(self, 6)
    }
    #[doc = "Bit 7 - Input Enable While Output enabled"]
    #[inline(always)]
    pub fn iewo(&mut self) -> IewoW<'_, PortaSpec> {
        IewoW::new(self, 7)
    }
    #[doc = "Bit 8 - Output Open Drain Mode"]
    #[inline(always)]
    pub fn opendrn(&mut self) -> OpendrnW<'_, PortaSpec> {
        OpendrnW::new(self, 8)
    }
    #[doc = "Bit 9 - Output Invert Selection"]
    #[inline(always)]
    pub fn invout(&mut self) -> InvoutW<'_, PortaSpec> {
        InvoutW::new(self, 9)
    }
    #[doc = "Bit 10 - Internal Pull up/down level"]
    #[inline(always)]
    pub fn plevel(&mut self) -> PlevelW<'_, PortaSpec> {
        PlevelW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Internal Pull up/down"]
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<'_, PortaSpec> {
        PenW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Pull when output active"]
    #[inline(always)]
    pub fn pwoa(&mut self) -> PwoaW<'_, PortaSpec> {
        PwoaW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Pin Function Selection"]
    #[inline(always)]
    pub fn funsel(&mut self) -> FunselW<'_, PortaSpec> {
        FunselW::new(self, 13)
    }
    #[doc = "Bit 16 - IO Pin Disable"]
    #[inline(always)]
    pub fn iodis(&mut self) -> IodisW<'_, PortaSpec> {
        IodisW::new(self, 16)
    }
}
#[doc = "PORTA Pin Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`porta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortaSpec;
impl crate::RegisterSpec for PortaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`porta::R`](R) reader structure"]
impl crate::Readable for PortaSpec {}
#[doc = "`write(|w| ..)` method takes [`porta::W`](W) writer structure"]
impl crate::Writable for PortaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORTA[%s] to value 0"]
impl crate::Resettable for PortaSpec {}
