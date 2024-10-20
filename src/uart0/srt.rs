#[doc = "Register `srt` reader"]
pub type R = crate::R<SrtSpec>;
#[doc = "Register `srt` writer"]
pub type W = crate::W<SrtSpec>;
#[doc = "Field `srt` reader - Shadow RCVR Trigger. This is a shadow register for the RCVR trigger bits (FCR\\[7:6\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the RCVR trigger bit gets updated. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt is generated. It also determines when the dma_rx_req_n signal is asserted when DMA Mode (FCR\\[3\\]) = 1. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO ¼ full 10 = FIFO ½ full 11 = FIFO 2 less than full"]
pub type SrtR = crate::FieldReader;
#[doc = "Field `srt` writer - Shadow RCVR Trigger. This is a shadow register for the RCVR trigger bits (FCR\\[7:6\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the RCVR trigger bit gets updated. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt is generated. It also determines when the dma_rx_req_n signal is asserted when DMA Mode (FCR\\[3\\]) = 1. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO ¼ full 10 = FIFO ½ full 11 = FIFO 2 less than full"]
pub type SrtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Shadow RCVR Trigger. This is a shadow register for the RCVR trigger bits (FCR\\[7:6\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the RCVR trigger bit gets updated. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt is generated. It also determines when the dma_rx_req_n signal is asserted when DMA Mode (FCR\\[3\\]) = 1. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO ¼ full 10 = FIFO ½ full 11 = FIFO 2 less than full"]
    #[inline(always)]
    pub fn srt(&self) -> SrtR {
        SrtR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Shadow RCVR Trigger. This is a shadow register for the RCVR trigger bits (FCR\\[7:6\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the RCVR trigger bit gets updated. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt is generated. It also determines when the dma_rx_req_n signal is asserted when DMA Mode (FCR\\[3\\]) = 1. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO ¼ full 10 = FIFO ½ full 11 = FIFO 2 less than full"]
    #[inline(always)]
    #[must_use]
    pub fn srt(&mut self) -> SrtW<SrtSpec> {
        SrtW::new(self, 0)
    }
}
#[doc = "Shadow RCVR Trigger: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrtSpec;
impl crate::RegisterSpec for SrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srt::R`](R) reader structure"]
impl crate::Readable for SrtSpec {}
#[doc = "`write(|w| ..)` method takes [`srt::W`](W) writer structure"]
impl crate::Writable for SrtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets srt to value 0"]
impl crate::Resettable for SrtSpec {
    const RESET_VALUE: u32 = 0;
}
