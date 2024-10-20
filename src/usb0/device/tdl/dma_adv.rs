#[doc = "Register `dma_adv` reader"]
pub type R = crate::R<DmaAdvSpec>;
#[doc = "Register `dma_adv` writer"]
pub type W = crate::W<DmaAdvSpec>;
#[doc = "Field `dma_adv` reader - TDL DMA advance configuration."]
pub type DmaAdvR = crate::FieldReader<u32>;
#[doc = "Field `dma_adv` writer - TDL DMA advance configuration."]
pub type DmaAdvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TDL DMA advance configuration."]
    #[inline(always)]
    pub fn dma_adv(&self) -> DmaAdvR {
        DmaAdvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TDL DMA advance configuration."]
    #[inline(always)]
    #[must_use]
    pub fn dma_adv(&mut self) -> DmaAdvW<DmaAdvSpec> {
        DmaAdvW::new(self, 0)
    }
}
#[doc = "TDL DMA Advance configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_adv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_adv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaAdvSpec;
impl crate::RegisterSpec for DmaAdvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_adv::R`](R) reader structure"]
impl crate::Readable for DmaAdvSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_adv::W`](W) writer structure"]
impl crate::Writable for DmaAdvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dma_adv to value 0"]
impl crate::Resettable for DmaAdvSpec {
    const RESET_VALUE: u32 = 0;
}
