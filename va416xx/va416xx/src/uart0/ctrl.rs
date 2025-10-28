#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `PAREN` reader - Parity Enable"]
pub type ParenR = crate::BitReader;
#[doc = "Field `PAREN` writer - Parity Enable"]
pub type ParenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAREVEN` reader - Parity Even/Odd(1/0)"]
pub type ParevenR = crate::BitReader;
#[doc = "Field `PAREVEN` writer - Parity Even/Odd(1/0)"]
pub type ParevenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARSTK` reader - Parity Sticky"]
pub type ParstkR = crate::BitReader;
#[doc = "Field `PARSTK` writer - Parity Sticky"]
pub type ParstkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPBITS` reader - Stop Bits 1/2(0/1)"]
pub type StopbitsR = crate::BitReader;
#[doc = "Field `STOPBITS` writer - Stop Bits 1/2(0/1)"]
pub type StopbitsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORDSIZE` reader - Word Size in Bits 5/6/7/8(00/01/10/11)"]
pub type WordsizeR = crate::FieldReader;
#[doc = "Field `WORDSIZE` writer - Word Size in Bits 5/6/7/8(00/01/10/11)"]
pub type WordsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LOOPBACK` reader - Loopback Enable"]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Loopback Enable"]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACKBLK` reader - Loopback Block"]
pub type LoopbackblkR = crate::BitReader;
#[doc = "Field `LOOPBACKBLK` writer - Loopback Block"]
pub type LoopbackblkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCTS` reader - Enable Auto CTS mode"]
pub type AutoctsR = crate::BitReader;
#[doc = "Field `AUTOCTS` writer - Enable Auto CTS mode"]
pub type AutoctsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFRTS` reader - Default RTSn value"]
pub type DefrtsR = crate::BitReader;
#[doc = "Field `DEFRTS` writer - Default RTSn value"]
pub type DefrtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTORTS` reader - Enable Auto RTS mode"]
pub type AutortsR = crate::BitReader;
#[doc = "Field `AUTORTS` writer - Enable Auto RTS mode"]
pub type AutortsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BAUD8` reader - Enable BAUD8 mode"]
pub type Baud8R = crate::BitReader;
#[doc = "Field `BAUD8` writer - Enable BAUD8 mode"]
pub type Baud8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parity Enable"]
    #[inline(always)]
    pub fn paren(&self) -> ParenR {
        ParenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Even/Odd(1/0)"]
    #[inline(always)]
    pub fn pareven(&self) -> ParevenR {
        ParevenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Sticky"]
    #[inline(always)]
    pub fn parstk(&self) -> ParstkR {
        ParstkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop Bits 1/2(0/1)"]
    #[inline(always)]
    pub fn stopbits(&self) -> StopbitsR {
        StopbitsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Word Size in Bits 5/6/7/8(00/01/10/11)"]
    #[inline(always)]
    pub fn wordsize(&self) -> WordsizeR {
        WordsizeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Loopback Enable"]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Loopback Block"]
    #[inline(always)]
    pub fn loopbackblk(&self) -> LoopbackblkR {
        LoopbackblkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Auto CTS mode"]
    #[inline(always)]
    pub fn autocts(&self) -> AutoctsR {
        AutoctsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Default RTSn value"]
    #[inline(always)]
    pub fn defrts(&self) -> DefrtsR {
        DefrtsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Auto RTS mode"]
    #[inline(always)]
    pub fn autorts(&self) -> AutortsR {
        AutortsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable BAUD8 mode"]
    #[inline(always)]
    pub fn baud8(&self) -> Baud8R {
        Baud8R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Enable"]
    #[inline(always)]
    pub fn paren(&mut self) -> ParenW<'_, CtrlSpec> {
        ParenW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity Even/Odd(1/0)"]
    #[inline(always)]
    pub fn pareven(&mut self) -> ParevenW<'_, CtrlSpec> {
        ParevenW::new(self, 1)
    }
    #[doc = "Bit 2 - Parity Sticky"]
    #[inline(always)]
    pub fn parstk(&mut self) -> ParstkW<'_, CtrlSpec> {
        ParstkW::new(self, 2)
    }
    #[doc = "Bit 3 - Stop Bits 1/2(0/1)"]
    #[inline(always)]
    pub fn stopbits(&mut self) -> StopbitsW<'_, CtrlSpec> {
        StopbitsW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Word Size in Bits 5/6/7/8(00/01/10/11)"]
    #[inline(always)]
    pub fn wordsize(&mut self) -> WordsizeW<'_, CtrlSpec> {
        WordsizeW::new(self, 4)
    }
    #[doc = "Bit 6 - Loopback Enable"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LoopbackW<'_, CtrlSpec> {
        LoopbackW::new(self, 6)
    }
    #[doc = "Bit 7 - Loopback Block"]
    #[inline(always)]
    pub fn loopbackblk(&mut self) -> LoopbackblkW<'_, CtrlSpec> {
        LoopbackblkW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Auto CTS mode"]
    #[inline(always)]
    pub fn autocts(&mut self) -> AutoctsW<'_, CtrlSpec> {
        AutoctsW::new(self, 8)
    }
    #[doc = "Bit 9 - Default RTSn value"]
    #[inline(always)]
    pub fn defrts(&mut self) -> DefrtsW<'_, CtrlSpec> {
        DefrtsW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Auto RTS mode"]
    #[inline(always)]
    pub fn autorts(&mut self) -> AutortsW<'_, CtrlSpec> {
        AutortsW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable BAUD8 mode"]
    #[inline(always)]
    pub fn baud8(&mut self) -> Baud8W<'_, CtrlSpec> {
        Baud8W::new(self, 11)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
