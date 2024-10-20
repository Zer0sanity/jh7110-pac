#[doc = "Register `sdmam` reader"]
pub type R = crate::R<SdmamSpec>;
#[doc = "Register `sdmam` writer"]
pub type W = crate::W<SdmamSpec>;
#[doc = "Field `sdmam` reader - Shadow DMA Mode. This is a shadow register for the DMA mode bit (FCR\\[3\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the DMA Mode bit gets updated. This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals when additional DMA handshaking signals are not selected (DMA_EXTRA == NO). 0 = mode 0 1 = mode 1"]
pub type SdmamR = crate::BitReader;
#[doc = "Field `sdmam` writer - Shadow DMA Mode. This is a shadow register for the DMA mode bit (FCR\\[3\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the DMA Mode bit gets updated. This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals when additional DMA handshaking signals are not selected (DMA_EXTRA == NO). 0 = mode 0 1 = mode 1"]
pub type SdmamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shadow DMA Mode. This is a shadow register for the DMA mode bit (FCR\\[3\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the DMA Mode bit gets updated. This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals when additional DMA handshaking signals are not selected (DMA_EXTRA == NO). 0 = mode 0 1 = mode 1"]
    #[inline(always)]
    pub fn sdmam(&self) -> SdmamR {
        SdmamR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow DMA Mode. This is a shadow register for the DMA mode bit (FCR\\[3\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the DMA Mode bit gets updated. This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals when additional DMA handshaking signals are not selected (DMA_EXTRA == NO). 0 = mode 0 1 = mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn sdmam(&mut self) -> SdmamW<SdmamSpec> {
        SdmamW::new(self, 0)
    }
}
#[doc = "Shadow DMA Mode: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmam::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmam::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmamSpec;
impl crate::RegisterSpec for SdmamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmam::R`](R) reader structure"]
impl crate::Readable for SdmamSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmam::W`](W) writer structure"]
impl crate::Writable for SdmamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sdmam to value 0"]
impl crate::Resettable for SdmamSpec {
    const RESET_VALUE: u32 = 0;
}
