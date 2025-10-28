#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "Field `RXENABLE` reader - Rx Enable"]
pub type RxenableR = crate::BitReader;
#[doc = "Field `RXENABLE` writer - Rx Enable"]
pub type RxenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXENABLE` reader - Tx Enable"]
pub type TxenableR = crate::BitReader;
#[doc = "Field `TXENABLE` writer - Tx Enable"]
pub type TxenableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx Enable"]
    #[inline(always)]
    pub fn rxenable(&self) -> RxenableR {
        RxenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx Enable"]
    #[inline(always)]
    pub fn txenable(&self) -> TxenableR {
        TxenableR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx Enable"]
    #[inline(always)]
    pub fn rxenable(&mut self) -> RxenableW<'_, EnableSpec> {
        RxenableW::new(self, 0)
    }
    #[doc = "Bit 1 - Tx Enable"]
    #[inline(always)]
    pub fn txenable(&mut self) -> TxenableW<'_, EnableSpec> {
        TxenableW::new(self, 1)
    }
}
#[doc = "Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSpec;
impl crate::RegisterSpec for EnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for EnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for EnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for EnableSpec {}
