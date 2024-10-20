#[doc = "Register `srr` writer"]
pub type W = crate::W<SrrSpec>;
#[doc = "Field `ur` writer - UART Reset. This asynchronously resets the DW_apb_uart and synchronously removes the reset assertion. For a two clock implementation both pclk and sclk domains are reset."]
pub type UrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfr` writer - RCVR FIFO Reset. This is a shadow register for the RCVR FIFO Reset bit (FCR\\[1\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the receive FIFO This resets the control portion of the receive FIFO and treats the FIFO as empty. This also de-asserts the DMA RX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub type RfrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xfr` writer - XMIT FIFO Reset. This is a shadow register for the XMIT FIFO Reset bit (FCR\\[2\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the transmit FIFO. This resets the control portion of the transmit FIFO and treats the FIFO as empty. This also de-asserts the DMA TX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub type XfrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - UART Reset. This asynchronously resets the DW_apb_uart and synchronously removes the reset assertion. For a two clock implementation both pclk and sclk domains are reset."]
    #[inline(always)]
    #[must_use]
    pub fn ur(&mut self) -> UrW<SrrSpec> {
        UrW::new(self, 0)
    }
    #[doc = "Bit 1 - RCVR FIFO Reset. This is a shadow register for the RCVR FIFO Reset bit (FCR\\[1\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the receive FIFO This resets the control portion of the receive FIFO and treats the FIFO as empty. This also de-asserts the DMA RX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn rfr(&mut self) -> RfrW<SrrSpec> {
        RfrW::new(self, 1)
    }
    #[doc = "Bit 2 - XMIT FIFO Reset. This is a shadow register for the XMIT FIFO Reset bit (FCR\\[2\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the transmit FIFO. This resets the control portion of the transmit FIFO and treats the FIFO as empty. This also de-asserts the DMA TX request and single signals when additional DMA handshaking signals are selected (DMA_EXTRA == YES). Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn xfr(&mut self) -> XfrW<SrrSpec> {
        XfrW::new(self, 2)
    }
}
#[doc = "Software Reset Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrrSpec;
impl crate::RegisterSpec for SrrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`srr::W`](W) writer structure"]
impl crate::Writable for SrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets srr to value 0"]
impl crate::Resettable for SrrSpec {
    const RESET_VALUE: u32 = 0;
}
