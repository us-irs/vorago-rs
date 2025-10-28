#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<Ctrl0Spec>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<Ctrl0Spec>;
#[doc = "Field `SIZE` reader - Data Size(0x3=>4, 0xf=>16)"]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - Data Size(0x3=>4, 0xf=>16)"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPO` reader - SPI Clock Polarity"]
pub type SpoR = crate::BitReader;
#[doc = "Field `SPO` writer - SPI Clock Polarity"]
pub type SpoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPH` reader - SPI Clock Phase"]
pub type SphR = crate::BitReader;
#[doc = "Field `SPH` writer - SPI Clock Phase"]
pub type SphW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRDV` reader - Serial Clock Rate divide+1 value"]
pub type ScrdvR = crate::FieldReader;
#[doc = "Field `SCRDV` writer - Serial Clock Rate divide+1 value"]
pub type ScrdvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Data Size(0x3=>4, 0xf=>16)"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn spo(&self) -> SpoR {
        SpoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Clock Phase"]
    #[inline(always)]
    pub fn sph(&self) -> SphR {
        SphR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Serial Clock Rate divide+1 value"]
    #[inline(always)]
    pub fn scrdv(&self) -> ScrdvR {
        ScrdvR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Size(0x3=>4, 0xf=>16)"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, Ctrl0Spec> {
        SizeW::new(self, 0)
    }
    #[doc = "Bit 6 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn spo(&mut self) -> SpoW<'_, Ctrl0Spec> {
        SpoW::new(self, 6)
    }
    #[doc = "Bit 7 - SPI Clock Phase"]
    #[inline(always)]
    pub fn sph(&mut self) -> SphW<'_, Ctrl0Spec> {
        SphW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Serial Clock Rate divide+1 value"]
    #[inline(always)]
    pub fn scrdv(&mut self) -> ScrdvW<'_, Ctrl0Spec> {
        ScrdvW::new(self, 8)
    }
}
#[doc = "Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl0Spec;
impl crate::RegisterSpec for Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for Ctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for Ctrl0Spec {}
