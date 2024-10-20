#[doc = "Register `fcr` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "Field `fifoe` writer - FIFO Enable. This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. Whenever the value of this bit is changed both the XMIT and RCVR controller portion of FIFOs is reset."]
pub type FifoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfifor` writer - RCVR FIFO Reset. This resets the control portion of the receive FIFO and treats the FIFO as empty. This also de-asserts the DMA RX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub type RfiforW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xfifor` writer - XMIT FIFO Reset. This resets the control portion of the transmit FIFO and treats the FIFO as empty. This also de-asserts the DMA TX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub type XfiforW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmam` writer - DMA Mode. This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals when additional DMA handshaking signals are not selected (DMA_EXTRA == No). For details on DMA support, refer to “DMA Support” on page 58. 0 = mode 0 1 = mode 1"]
pub type DmamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tet` writer - TX Empty Trigger. Writes have no effect when THRE_MODE_USER == Disabled. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. It also determines when the dma_tx_req_n signal is asserted when in certain modes of operation. For details on DMA support, refer to “DMA Support” on page 58. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO ¼ full 11 = FIFO ½ full"]
pub type TetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rt` writer - RCVR Trigger. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt is generated. In auto flow control mode it is used to determine when the rts_n signal is de-asserted. It also determines when the dma_rx_req_n signal is asserted in certain modes of operation. For details on DMA support, refer to “DMA Support” on page 58. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO ¼ full 10 = FIFO ½ full 11 = FIFO 2 less than full"]
pub type RtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bit 0 - FIFO Enable. This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. Whenever the value of this bit is changed both the XMIT and RCVR controller portion of FIFOs is reset."]
    #[inline(always)]
    #[must_use]
    pub fn fifoe(&mut self) -> FifoeW<FcrSpec> {
        FifoeW::new(self, 0)
    }
    #[doc = "Bit 1 - RCVR FIFO Reset. This resets the control portion of the receive FIFO and treats the FIFO as empty. This also de-asserts the DMA RX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn rfifor(&mut self) -> RfiforW<FcrSpec> {
        RfiforW::new(self, 1)
    }
    #[doc = "Bit 2 - XMIT FIFO Reset. This resets the control portion of the transmit FIFO and treats the FIFO as empty. This also de-asserts the DMA TX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn xfifor(&mut self) -> XfiforW<FcrSpec> {
        XfiforW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Mode. This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals when additional DMA handshaking signals are not selected (DMA_EXTRA == No). For details on DMA support, refer to “DMA Support” on page 58. 0 = mode 0 1 = mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn dmam(&mut self) -> DmamW<FcrSpec> {
        DmamW::new(self, 3)
    }
    #[doc = "Bits 4:5 - TX Empty Trigger. Writes have no effect when THRE_MODE_USER == Disabled. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. It also determines when the dma_tx_req_n signal is asserted when in certain modes of operation. For details on DMA support, refer to “DMA Support” on page 58. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO ¼ full 11 = FIFO ½ full"]
    #[inline(always)]
    #[must_use]
    pub fn tet(&mut self) -> TetW<FcrSpec> {
        TetW::new(self, 4)
    }
    #[doc = "Bits 6:7 - RCVR Trigger. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt is generated. In auto flow control mode it is used to determine when the rts_n signal is de-asserted. It also determines when the dma_rx_req_n signal is asserted in certain modes of operation. For details on DMA support, refer to “DMA Support” on page 58. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO ¼ full 10 = FIFO ½ full 11 = FIFO 2 less than full"]
    #[inline(always)]
    #[must_use]
    pub fn rt(&mut self) -> RtW<FcrSpec> {
        RtW::new(self, 6)
    }
}
#[doc = "FIFO Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fcr to value 0"]
impl crate::Resettable for FcrSpec {
    const RESET_VALUE: u32 = 0;
}
