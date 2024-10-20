#[doc = "Register `dst_addr` reader"]
pub type R = crate::R<DstAddrSpec>;
#[doc = "Register `dst_addr` writer"]
pub type W = crate::W<DstAddrSpec>;
#[doc = "Field `dst_addr` reader - DMA destination address."]
pub type DstAddrR = crate::FieldReader<u32>;
#[doc = "Field `dst_addr` writer - DMA destination address."]
pub type DstAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA destination address."]
    #[inline(always)]
    pub fn dst_addr(&self) -> DstAddrR {
        DstAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA destination address."]
    #[inline(always)]
    #[must_use]
    pub fn dst_addr(&mut self) -> DstAddrW<DstAddrSpec> {
        DstAddrW::new(self, 0)
    }
}
#[doc = "DMA Destination Address register - contain the current destination address, byte-aligned, of the data to be transferred. Software programs each register directly before the channel is enabled. When the DMA channel is enabled, the register is updated as the destination address is incremented and by following the linked list when a complete packet of data has been transferred. Reading the register when the channel is active does not provide useful information. This is because by the time the software has processed the value read, the channel might have progressed. It is intended to be read-only when a channel has stopped. In this case, it shows the destination address of the last item read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstAddrSpec;
impl crate::RegisterSpec for DstAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_addr::R`](R) reader structure"]
impl crate::Readable for DstAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_addr::W`](W) writer structure"]
impl crate::Writable for DstAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dst_addr to value 0"]
impl crate::Resettable for DstAddrSpec {
    const RESET_VALUE: u32 = 0;
}
