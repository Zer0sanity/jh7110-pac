#[doc = "Register `srts` reader"]
pub type R = crate::R<SrtsSpec>;
#[doc = "Register `srts` writer"]
pub type W = crate::W<SrtsSpec>;
#[doc = "Field `srts` reader - Shadow Request to Send. This is a shadow register for the RTS bit (MCR\\[1\\]), this can be used to remove the burden of having to performing a read-modify-write on the MCR. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the DW_apb_uart is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
= 0), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high. In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
= 1) and FIFOs enable (FCR\\[0\\]
= 1), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). Note that in Loopback mode (MCR\\[4\\]
= 1), the rts_n output is held inactive-high while the value of this location is internally looped back to an input."]
pub type SrtsR = crate::BitReader;
#[doc = "Field `srts` writer - Shadow Request to Send. This is a shadow register for the RTS bit (MCR\\[1\\]), this can be used to remove the burden of having to performing a read-modify-write on the MCR. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the DW_apb_uart is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
= 0), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high. In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
= 1) and FIFOs enable (FCR\\[0\\]
= 1), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). Note that in Loopback mode (MCR\\[4\\]
= 1), the rts_n output is held inactive-high while the value of this location is internally looped back to an input."]
pub type SrtsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shadow Request to Send. This is a shadow register for the RTS bit (MCR\\[1\\]), this can be used to remove the burden of having to performing a read-modify-write on the MCR. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the DW_apb_uart is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
= 0), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high. In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
= 1) and FIFOs enable (FCR\\[0\\]
= 1), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). Note that in Loopback mode (MCR\\[4\\]
= 1), the rts_n output is held inactive-high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn srts(&self) -> SrtsR {
        SrtsR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow Request to Send. This is a shadow register for the RTS bit (MCR\\[1\\]), this can be used to remove the burden of having to performing a read-modify-write on the MCR. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the DW_apb_uart is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
= 0), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high. In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
= 1) and FIFOs enable (FCR\\[0\\]
= 1), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). Note that in Loopback mode (MCR\\[4\\]
= 1), the rts_n output is held inactive-high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    #[must_use]
    pub fn srts(&mut self) -> SrtsW<SrtsSpec> {
        SrtsW::new(self, 0)
    }
}
#[doc = "Shadow Request to Send: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrtsSpec;
impl crate::RegisterSpec for SrtsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srts::R`](R) reader structure"]
impl crate::Readable for SrtsSpec {}
#[doc = "`write(|w| ..)` method takes [`srts::W`](W) writer structure"]
impl crate::Writable for SrtsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets srts to value 0"]
impl crate::Resettable for SrtsSpec {
    const RESET_VALUE: u32 = 0;
}
