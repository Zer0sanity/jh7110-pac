#[doc = "Register `lpdlh` reader"]
pub type R = crate::R<LpdlhSpec>;
#[doc = "Register `lpdlh` writer"]
pub type W = crate::W<LpdlhSpec>;
#[doc = "Field `lpdlh` reader - This register makes up the upper 8-bits of a 16-bit, read/write, Low Power Divisor Latch register that contains the baud rate divisor for the UART, which must give a baud rate of 115.2K. This is required for SIR Low Power (minimum pulse width) detection at the receiver. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set and the UART is not busy (USR\\[0\\]) is 0). The output low-power baud rate is equal to the serial clock (sclk) frequency divided by sixteen times the value of the baud rate divisor, as follows: Low power baud rate = (serial clock frequency)/(16* divisor) Therefore, a divisor must be selected to give a baud rate of 115.2K. NOTE: When the Low Power Divisor Latch registers (LPDLL and LPDLH) are set to 0, the low-power baud clock is disabled and no low-power pulse detection (or any pulse detection) occurs at the receiver. Also, once the LPDLH is set, at least eight clock cycles of the slowest DW_apb_uart clock should be allowed to pass before transmitting or receiving data"]
pub type LpdlhR = crate::FieldReader;
#[doc = "Field `lpdlh` writer - This register makes up the upper 8-bits of a 16-bit, read/write, Low Power Divisor Latch register that contains the baud rate divisor for the UART, which must give a baud rate of 115.2K. This is required for SIR Low Power (minimum pulse width) detection at the receiver. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set and the UART is not busy (USR\\[0\\]) is 0). The output low-power baud rate is equal to the serial clock (sclk) frequency divided by sixteen times the value of the baud rate divisor, as follows: Low power baud rate = (serial clock frequency)/(16* divisor) Therefore, a divisor must be selected to give a baud rate of 115.2K. NOTE: When the Low Power Divisor Latch registers (LPDLL and LPDLH) are set to 0, the low-power baud clock is disabled and no low-power pulse detection (or any pulse detection) occurs at the receiver. Also, once the LPDLH is set, at least eight clock cycles of the slowest DW_apb_uart clock should be allowed to pass before transmitting or receiving data"]
pub type LpdlhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register makes up the upper 8-bits of a 16-bit, read/write, Low Power Divisor Latch register that contains the baud rate divisor for the UART, which must give a baud rate of 115.2K. This is required for SIR Low Power (minimum pulse width) detection at the receiver. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set and the UART is not busy (USR\\[0\\]) is 0). The output low-power baud rate is equal to the serial clock (sclk) frequency divided by sixteen times the value of the baud rate divisor, as follows: Low power baud rate = (serial clock frequency)/(16* divisor) Therefore, a divisor must be selected to give a baud rate of 115.2K. NOTE: When the Low Power Divisor Latch registers (LPDLL and LPDLH) are set to 0, the low-power baud clock is disabled and no low-power pulse detection (or any pulse detection) occurs at the receiver. Also, once the LPDLH is set, at least eight clock cycles of the slowest DW_apb_uart clock should be allowed to pass before transmitting or receiving data"]
    #[inline(always)]
    pub fn lpdlh(&self) -> LpdlhR {
        LpdlhR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register makes up the upper 8-bits of a 16-bit, read/write, Low Power Divisor Latch register that contains the baud rate divisor for the UART, which must give a baud rate of 115.2K. This is required for SIR Low Power (minimum pulse width) detection at the receiver. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set and the UART is not busy (USR\\[0\\]) is 0). The output low-power baud rate is equal to the serial clock (sclk) frequency divided by sixteen times the value of the baud rate divisor, as follows: Low power baud rate = (serial clock frequency)/(16* divisor) Therefore, a divisor must be selected to give a baud rate of 115.2K. NOTE: When the Low Power Divisor Latch registers (LPDLL and LPDLH) are set to 0, the low-power baud clock is disabled and no low-power pulse detection (or any pulse detection) occurs at the receiver. Also, once the LPDLH is set, at least eight clock cycles of the slowest DW_apb_uart clock should be allowed to pass before transmitting or receiving data"]
    #[inline(always)]
    #[must_use]
    pub fn lpdlh(&mut self) -> LpdlhW<LpdlhSpec> {
        LpdlhW::new(self, 0)
    }
}
#[doc = "Low Power Divisor Latch High Register: This register is only valid when the DW_apb_uart is configured to have SIR low-power reception capabilities implemented (SIR_LP_RX = Yes). If SIR low-power reception capabilities are not implemented, this register does not exist and reading from thsi register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdlh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdlh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpdlhSpec;
impl crate::RegisterSpec for LpdlhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpdlh::R`](R) reader structure"]
impl crate::Readable for LpdlhSpec {}
#[doc = "`write(|w| ..)` method takes [`lpdlh::W`](W) writer structure"]
impl crate::Writable for LpdlhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lpdlh to value 0"]
impl crate::Resettable for LpdlhSpec {
    const RESET_VALUE: u32 = 0;
}
