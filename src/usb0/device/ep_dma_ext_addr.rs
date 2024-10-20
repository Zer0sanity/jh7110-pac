#[doc = "Register `ep_dma_ext_addr` reader"]
pub type R = crate::R<EpDmaExtAddrSpec>;
#[doc = "Register `ep_dma_ext_addr` writer"]
pub type W = crate::W<EpDmaExtAddrSpec>;
#[doc = "Field `ep_dma_ext_addr` reader - Custom packet."]
pub type EpDmaExtAddrR = crate::FieldReader<u32>;
#[doc = "Field `ep_dma_ext_addr` writer - Custom packet."]
pub type EpDmaExtAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Custom packet."]
    #[inline(always)]
    pub fn ep_dma_ext_addr(&self) -> EpDmaExtAddrR {
        EpDmaExtAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Custom packet."]
    #[inline(always)]
    #[must_use]
    pub fn ep_dma_ext_addr(&mut self) -> EpDmaExtAddrW<EpDmaExtAddrSpec> {
        EpDmaExtAddrW::new(self, 0)
    }
}
#[doc = "USB3 Endpoint upper address for DMA operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_dma_ext_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_dma_ext_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpDmaExtAddrSpec;
impl crate::RegisterSpec for EpDmaExtAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_dma_ext_addr::R`](R) reader structure"]
impl crate::Readable for EpDmaExtAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_dma_ext_addr::W`](W) writer structure"]
impl crate::Writable for EpDmaExtAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ep_dma_ext_addr to value 0"]
impl crate::Resettable for EpDmaExtAddrSpec {
    const RESET_VALUE: u32 = 0;
}
