#[doc = "Register `ssp_dmacr` reader"]
pub type R = crate::R<SspDmacrSpec>;
#[doc = "Register `ssp_dmacr` writer"]
pub type W = crate::W<SspDmacrSpec>;
#[doc = "Field `rxdmae` reader - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RxdmaeR = crate::BitReader;
#[doc = "Field `rxdmae` writer - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txdmae` reader - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TxdmaeR = crate::BitReader;
#[doc = "Field `txdmae` writer - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub fn rxdmae(&self) -> RxdmaeR {
        RxdmaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    pub fn txdmae(&self) -> TxdmaeR {
        TxdmaeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RxdmaeW<SspDmacrSpec> {
        RxdmaeW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TxdmaeW<SspDmacrSpec> {
        TxdmaeW::new(self, 1)
    }
}
#[doc = "The SSPDMACR register is the DMA control register. It is a RW register. All the bits are cleared to 0 on reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_dmacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_dmacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspDmacrSpec;
impl crate::RegisterSpec for SspDmacrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_dmacr::R`](R) reader structure"]
impl crate::Readable for SspDmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssp_dmacr::W`](W) writer structure"]
impl crate::Writable for SspDmacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ssp_dmacr to value 0"]
impl crate::Resettable for SspDmacrSpec {
    const RESET_VALUE: u16 = 0;
}
