#[doc = "Register `EF_CONFIG` reader"]
pub type R = crate::R<EfConfigSpec>;
#[doc = "Field `ROM_SPEED` reader - Specifies the speed of ROM_SCK"]
pub type RomSpeedR = crate::FieldReader;
#[doc = "Field `ROM_SIZE` reader - Specifies how much of the full 128K byte address space is loaded from the external SPI memory after a reset"]
pub type RomSizeR = crate::FieldReader;
#[doc = "Field `ROM_NOCHECK` reader - When set to 1, the ROM check is skipped"]
pub type RomNocheckR = crate::BitReader;
#[doc = "Field `BOOT_DELAY` reader - Specifies the boot delay from the end of the Power-On-Sequence to the release of Reset"]
pub type BootDelayR = crate::FieldReader;
#[doc = "Field `ROM_READ` reader - SPI ROM read instruction code"]
pub type RomReadR = crate::FieldReader;
#[doc = "Field `ROM_LATENCY` reader - Number of bits of latency from Address until data from the SPI ROM"]
pub type RomLatencyR = crate::FieldReader;
#[doc = "Field `ROM_ADDRESS` reader - ROM Address Mode"]
pub type RomAddressR = crate::FieldReader;
#[doc = "Field `ROM_DLYCAP` reader - ROM SPI Delayed capture"]
pub type RomDlycapR = crate::BitReader;
#[doc = "Field `ROM_STATUS` reader - The first data byte from the SPI ROM following an address is taken as a status byte"]
pub type RomStatusR = crate::BitReader;
#[doc = "Field `RM` reader - This bit controls the internal RAM read timing and must be maintained at this value"]
pub type RmR = crate::BitReader;
#[doc = "Field `WM` reader - This bit controls the internal RAM write timing and must be maintained at this value"]
pub type WmR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Specifies the speed of ROM_SCK"]
    #[inline(always)]
    pub fn rom_speed(&self) -> RomSpeedR {
        RomSpeedR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Specifies how much of the full 128K byte address space is loaded from the external SPI memory after a reset"]
    #[inline(always)]
    pub fn rom_size(&self) -> RomSizeR {
        RomSizeR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - When set to 1, the ROM check is skipped"]
    #[inline(always)]
    pub fn rom_nocheck(&self) -> RomNocheckR {
        RomNocheckR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - Specifies the boot delay from the end of the Power-On-Sequence to the release of Reset"]
    #[inline(always)]
    pub fn boot_delay(&self) -> BootDelayR {
        BootDelayR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:17 - SPI ROM read instruction code"]
    #[inline(always)]
    pub fn rom_read(&self) -> RomReadR {
        RomReadR::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:22 - Number of bits of latency from Address until data from the SPI ROM"]
    #[inline(always)]
    pub fn rom_latency(&self) -> RomLatencyR {
        RomLatencyR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:24 - ROM Address Mode"]
    #[inline(always)]
    pub fn rom_address(&self) -> RomAddressR {
        RomAddressR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - ROM SPI Delayed capture"]
    #[inline(always)]
    pub fn rom_dlycap(&self) -> RomDlycapR {
        RomDlycapR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The first data byte from the SPI ROM following an address is taken as a status byte"]
    #[inline(always)]
    pub fn rom_status(&self) -> RomStatusR {
        RomStatusR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit controls the internal RAM read timing and must be maintained at this value"]
    #[inline(always)]
    pub fn rm(&self) -> RmR {
        RmR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit controls the internal RAM write timing and must be maintained at this value"]
    #[inline(always)]
    pub fn wm(&self) -> WmR {
        WmR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "EFuse Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_config::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfConfigSpec;
impl crate::RegisterSpec for EfConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_config::R`](R) reader structure"]
impl crate::Readable for EfConfigSpec {}
#[doc = "`reset()` method sets EF_CONFIG to value 0x0a80_0c40"]
impl crate::Resettable for EfConfigSpec {
    const RESET_VALUE: u32 = 0x0a80_0c40;
}
