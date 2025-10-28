#[doc = "Register `MAC_ADDR_L` reader"]
pub type R = crate::R<MacAddrLSpec>;
#[doc = "Register `MAC_ADDR_L` writer"]
pub type W = crate::W<MacAddrLSpec>;
#[doc = "Field `ADDRLO` reader - MAC Address0\\[31:0\\]"]
pub type AddrloR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MAC Address0\\[31:0\\]"]
    #[inline(always)]
    pub fn addrlo(&self) -> AddrloR {
        AddrloR::new(self.bits)
    }
}
impl W {}
#[doc = "Contains the Low 32-bits of the first MAC Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_addr_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_addr_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacAddrLSpec;
impl crate::RegisterSpec for MacAddrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_addr_l::R`](R) reader structure"]
impl crate::Readable for MacAddrLSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_addr_l::W`](W) writer structure"]
impl crate::Writable for MacAddrLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_ADDR_L to value 0xffff_ffff"]
impl crate::Resettable for MacAddrLSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
