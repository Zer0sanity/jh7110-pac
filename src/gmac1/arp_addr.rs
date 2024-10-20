#[doc = "Register `arp_addr` reader"]
pub type R = crate::R<ArpAddrSpec>;
#[doc = "Register `arp_addr` writer"]
pub type W = crate::W<ArpAddrSpec>;
#[doc = "Field `arp_addr` reader - MAC ARP Address"]
pub type ArpAddrR = crate::FieldReader<u32>;
#[doc = "Field `arp_addr` writer - MAC ARP Address"]
pub type ArpAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC ARP Address"]
    #[inline(always)]
    pub fn arp_addr(&self) -> ArpAddrR {
        ArpAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC ARP Address"]
    #[inline(always)]
    #[must_use]
    pub fn arp_addr(&mut self) -> ArpAddrW<ArpAddrSpec> {
        ArpAddrW::new(self, 0)
    }
}
#[doc = "MAC ARP Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arp_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arp_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArpAddrSpec;
impl crate::RegisterSpec for ArpAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arp_addr::R`](R) reader structure"]
impl crate::Readable for ArpAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`arp_addr::W`](W) writer structure"]
impl crate::Writable for ArpAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets arp_addr to value 0"]
impl crate::Resettable for ArpAddrSpec {
    const RESET_VALUE: u32 = 0;
}
