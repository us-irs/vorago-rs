#[doc = "Register `CEDIAG` reader"]
pub type R = crate::R<CediagSpec>;
#[doc = "Register `CEDIAG` writer"]
pub type W = crate::W<CediagSpec>;
#[doc = "Field `EFID` reader - Error Field Identifier"]
pub type EfidR = crate::FieldReader;
#[doc = "Field `EFID` writer - Error Field Identifier"]
pub type EfidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EBID` reader - Error Bit Identifier"]
pub type EbidR = crate::FieldReader;
#[doc = "Field `EBID` writer - Error Bit Identifier"]
pub type EbidW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TXE` reader - Transmit Error"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - Transmit Error"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUFF` reader - Stuff Error"]
pub type StuffR = crate::BitReader;
#[doc = "Field `STUFF` writer - Stuff Error"]
pub type StuffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - CRC"]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - CRC"]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON` reader - Monitor"]
pub type MonR = crate::BitReader;
#[doc = "Field `MON` writer - Monitor"]
pub type MonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVE` reader - Drive"]
pub type DriveR = crate::BitReader;
#[doc = "Field `DRIVE` writer - Drive"]
pub type DriveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Error Field Identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EfidR {
        EfidR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - Error Bit Identifier"]
    #[inline(always)]
    pub fn ebid(&self) -> EbidR {
        EbidR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 10 - Transmit Error"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stuff Error"]
    #[inline(always)]
    pub fn stuff(&self) -> StuffR {
        StuffR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Monitor"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Drive"]
    #[inline(always)]
    pub fn drive(&self) -> DriveR {
        DriveR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Error Field Identifier"]
    #[inline(always)]
    pub fn efid(&mut self) -> EfidW<'_, CediagSpec> {
        EfidW::new(self, 0)
    }
    #[doc = "Bits 4:9 - Error Bit Identifier"]
    #[inline(always)]
    pub fn ebid(&mut self) -> EbidW<'_, CediagSpec> {
        EbidW::new(self, 4)
    }
    #[doc = "Bit 10 - Transmit Error"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, CediagSpec> {
        TxeW::new(self, 10)
    }
    #[doc = "Bit 11 - Stuff Error"]
    #[inline(always)]
    pub fn stuff(&mut self) -> StuffW<'_, CediagSpec> {
        StuffW::new(self, 11)
    }
    #[doc = "Bit 12 - CRC"]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<'_, CediagSpec> {
        CrcW::new(self, 12)
    }
    #[doc = "Bit 13 - Monitor"]
    #[inline(always)]
    pub fn mon(&mut self) -> MonW<'_, CediagSpec> {
        MonW::new(self, 13)
    }
    #[doc = "Bit 14 - Drive"]
    #[inline(always)]
    pub fn drive(&mut self) -> DriveW<'_, CediagSpec> {
        DriveW::new(self, 14)
    }
}
#[doc = "CAN Error Diagnostic Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cediag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cediag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CediagSpec;
impl crate::RegisterSpec for CediagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cediag::R`](R) reader structure"]
impl crate::Readable for CediagSpec {}
#[doc = "`write(|w| ..)` method takes [`cediag::W`](W) writer structure"]
impl crate::Writable for CediagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CEDIAG to value 0"]
impl crate::Resettable for CediagSpec {}
