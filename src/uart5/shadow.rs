#[doc = "Register `shadow[%s]` reader"]
pub type R = crate::R<ShadowSpec>;
#[doc = "Register `shadow[%s]` writer"]
pub type W = crate::W<ShadowSpec>;
#[doc = "Field `srbr` reader - Shadow Receive Buffer Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If in non-FIFO mode (FIFO_MODE == NONE) or FIFOs are disabled (FCR\\[0\\]
set to zero), the data in the RBR must be read before the next data arrives, otherwise it is overwritten, resulting in an overrun error. If in FIFO mode (FIFO_MODE != NONE) and FIFOs are enabled (FCR\\[0\\]
set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO are preserved, but any incoming data is lost. An overrun error also occurs."]
pub type SrbrR = crate::FieldReader;
#[doc = "Field `sthr` writer - Shadow Threshold Register: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If in non-FIFO mode or FIFOs are disabled (FCR\\[0\\]
set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If in FIFO mode and FIFOs are enabled (FCR\\[0\\]
set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
pub type SthrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Shadow Receive Buffer Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If in non-FIFO mode (FIFO_MODE == NONE) or FIFOs are disabled (FCR\\[0\\]
set to zero), the data in the RBR must be read before the next data arrives, otherwise it is overwritten, resulting in an overrun error. If in FIFO mode (FIFO_MODE != NONE) and FIFOs are enabled (FCR\\[0\\]
set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO are preserved, but any incoming data is lost. An overrun error also occurs."]
    #[inline(always)]
    pub fn srbr(&self) -> SrbrR {
        SrbrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shadow Threshold Register: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If in non-FIFO mode or FIFOs are disabled (FCR\\[0\\]
set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If in FIFO mode and FIFOs are enabled (FCR\\[0\\]
set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    #[must_use]
    pub fn sthr(&mut self) -> SthrW<ShadowSpec> {
        SthrW::new(self, 0)
    }
}
#[doc = "This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShadowSpec;
impl crate::RegisterSpec for ShadowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shadow::R`](R) reader structure"]
impl crate::Readable for ShadowSpec {}
#[doc = "`write(|w| ..)` method takes [`shadow::W`](W) writer structure"]
impl crate::Writable for ShadowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets shadow[%s]
to value 0"]
impl crate::Resettable for ShadowSpec {
    const RESET_VALUE: u32 = 0;
}
