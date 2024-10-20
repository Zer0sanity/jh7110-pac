#[doc = "Register `dma[%s]` reader"]
pub type R = crate::R<DmaSpec>;
#[doc = "Register `dma[%s]` writer"]
pub type W = crate::W<DmaSpec>;
#[doc = "Field `dma_len` reader - DMA transfer length"]
pub type DmaLenR = crate::FieldReader<u32>;
#[doc = "Field `dma_len` writer - DMA transfer length"]
pub type DmaLenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA transfer length"]
    #[inline(always)]
    pub fn dma_len(&self) -> DmaLenR {
        DmaLenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA transfer length"]
    #[inline(always)]
    #[must_use]
    pub fn dma_len(&mut self) -> DmaLenW<DmaSpec> {
        DmaLenW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto DMA registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSpec;
impl crate::RegisterSpec for DmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DmaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dma[%s]
to value 0"]
impl crate::Resettable for DmaSpec {
    const RESET_VALUE: u32 = 0;
}
