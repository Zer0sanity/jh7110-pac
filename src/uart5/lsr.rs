#[doc = "Register `lsr` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Field `dr` reader - Data Ready bit. This is used to indicate that the receiver contains at least one character in the RBR or the receiver FIFO. 0 = no data ready 1 = data ready This bit is cleared when the RBR is read in non-FIFO mode, or when the receiver FIFO is empty, in FIFO mode."]
pub type DrR = crate::BitReader;
#[doc = "Field `oe` reader - Overrun error bit. This is used to indicate the occurrence of an overrun error. This occurs if a new data character was received before the previous data was read. In the non-FIFO mode, the OE bit is set when a new character arrives in the receiver before the previous character was read from the RBR. When this happens, the data in the RBR is overwritten. In the FIFO mode, an overrun error occurs when the FIFO is full and a new character arrives at the receiver. The data in the FIFO is retained and the data in the receive shift register is lost. 0 = no overrun error 1 = overrun error Reading the LSR clears the OE bit."]
pub type OeR = crate::BitReader;
#[doc = "Field `pe` reader - Parity Error bit. This is used to indicate the occurrence of a parity error in the receiver if the Parity Enable (PEN) bit (LCR\\[3\\]) is set. In the FIFO mode, since the parity error is associated with a character received, it is revealed when the character with the parity error arrives at the top of the FIFO. It should be noted that the Parity Error (PE) bit (LSR\\[2\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]). 0 = no parity error 1 = parity error Reading the LSR clears the PE bit."]
pub type PeR = crate::BitReader;
#[doc = "Field `fe` reader - Framing Error bit. This is used to indicate the occurrence of a framing error in the receiver. A framing error occurs when the receiver does not detect a valid STOP bit in the received data. In the FIFO mode, since the framing error is associated with a character received, it is revealed when the character with the framing error is at the top of the FIFO. When a framing error occurs, the UART tries to resynchronize. It does this by assuming that the error was due to the start bit of the next character and then continues receiving the other bit i.e. data, and/or parity and stop. It should be noted that the Framing Error (FE) bit (LSR\\[3\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]). 0 = no framing error 1 = framing error Reading the LSR clears the FE bit."]
pub type FeR = crate::BitReader;
#[doc = "Field `bi` reader - Break Interrupt bit. This is used to indicate the detection of a break sequence on the serial input data. If in UART mode (SIR_MODE == Disabled), it is set whenever the serial input, sin, is held in a logic '0' state for longer than the sum of start time + data bits + parity + stop bits. If in infrared mode (SIR_MODE == Enabled), it is set whenever the serial input, sir_in, is continuously pulsed to logic '0' for longer than the sum of start time + data bits + parity + stop bits. A break condition on serial input causes one and only one character, consisting of all zeros, to be received by the UART. In the FIFO mode, the character associated with the break condition is carried through the FIFO and is revealed when the character is at the top of the FIFO. Reading the LSR clears the BI bit. In the non-FIFO mode, the BI indication occurs immediately and persists until the LSR is read."]
pub type BiR = crate::BitReader;
#[doc = "Field `thre` reader - Transmit Holding Register Empty bit. If THRE_MODE_USER == Disabled or THRE mode is disabled (IER\\[7\\]
set to zero) and regardless of FIFO's being implemented/enabled or not, this bit indicates that the THR or TX FIFO is empty. This bit is set whenever data is transferred from the THR or TX FIFO to the transmitter shift register and no new data has been written to the THR or TX FIFO. This also causes a THRE Interrupt to occur, if the THRE Interrupt is enabled. If THRE_MODE_USER == Enabled AND FIFO_MODE != NONE and both modes are active (IER\\[7\\]
set to one and FCR\\[0\\]
set to one respectively), the functionality is switched to indicate the transmitter FIFO is full, and no longer controls THRE interrupts, which are then controlled by the FCR\\[5:4\\]
threshold setting. For more details, see “Programmable THRE Interrupt” on page 54."]
pub type ThreR = crate::BitReader;
#[doc = "Field `temt` reader - Transmitter Empty bit. If in FIFO mode (FIFO_MODE != NONE) and FIFOs enabled (FCR\\[0\\]
set to one), this bit is set whenever the Transmitter Shift Register and the FIFO are both empty. If in non-FIFO mode or FIFOs are disabled, this bit is set whenever the Transmitter Holding Register and the Transmitter Shift Register are both empty."]
pub type TemtR = crate::BitReader;
#[doc = "Field `rfe` reader - Receiver FIFO Error bit. This bit is only relevant when FIFO_MODE != NONE AND FIFOs are enabled (FCR\\[0\\]
set to one). This is used to indicate if there is at least one parity error, framing error, or break indication in the FIFO. 0 = no error in RX FIFO 1 = error in RX FIFO This bit is cleared when the LSR is read and the character with the error is at the top of the receiver FIFO and there are no subsequent errors in the FIFO."]
pub type RfeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Ready bit. This is used to indicate that the receiver contains at least one character in the RBR or the receiver FIFO. 0 = no data ready 1 = data ready This bit is cleared when the RBR is read in non-FIFO mode, or when the receiver FIFO is empty, in FIFO mode."]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun error bit. This is used to indicate the occurrence of an overrun error. This occurs if a new data character was received before the previous data was read. In the non-FIFO mode, the OE bit is set when a new character arrives in the receiver before the previous character was read from the RBR. When this happens, the data in the RBR is overwritten. In the FIFO mode, an overrun error occurs when the FIFO is full and a new character arrives at the receiver. The data in the FIFO is retained and the data in the receive shift register is lost. 0 = no overrun error 1 = overrun error Reading the LSR clears the OE bit."]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error bit. This is used to indicate the occurrence of a parity error in the receiver if the Parity Enable (PEN) bit (LCR\\[3\\]) is set. In the FIFO mode, since the parity error is associated with a character received, it is revealed when the character with the parity error arrives at the top of the FIFO. It should be noted that the Parity Error (PE) bit (LSR\\[2\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]). 0 = no parity error 1 = parity error Reading the LSR clears the PE bit."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error bit. This is used to indicate the occurrence of a framing error in the receiver. A framing error occurs when the receiver does not detect a valid STOP bit in the received data. In the FIFO mode, since the framing error is associated with a character received, it is revealed when the character with the framing error is at the top of the FIFO. When a framing error occurs, the UART tries to resynchronize. It does this by assuming that the error was due to the start bit of the next character and then continues receiving the other bit i.e. data, and/or parity and stop. It should be noted that the Framing Error (FE) bit (LSR\\[3\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]). 0 = no framing error 1 = framing error Reading the LSR clears the FE bit."]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt bit. This is used to indicate the detection of a break sequence on the serial input data. If in UART mode (SIR_MODE == Disabled), it is set whenever the serial input, sin, is held in a logic '0' state for longer than the sum of start time + data bits + parity + stop bits. If in infrared mode (SIR_MODE == Enabled), it is set whenever the serial input, sir_in, is continuously pulsed to logic '0' for longer than the sum of start time + data bits + parity + stop bits. A break condition on serial input causes one and only one character, consisting of all zeros, to be received by the UART. In the FIFO mode, the character associated with the break condition is carried through the FIFO and is revealed when the character is at the top of the FIFO. Reading the LSR clears the BI bit. In the non-FIFO mode, the BI indication occurs immediately and persists until the LSR is read."]
    #[inline(always)]
    pub fn bi(&self) -> BiR {
        BiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Holding Register Empty bit. If THRE_MODE_USER == Disabled or THRE mode is disabled (IER\\[7\\]
set to zero) and regardless of FIFO's being implemented/enabled or not, this bit indicates that the THR or TX FIFO is empty. This bit is set whenever data is transferred from the THR or TX FIFO to the transmitter shift register and no new data has been written to the THR or TX FIFO. This also causes a THRE Interrupt to occur, if the THRE Interrupt is enabled. If THRE_MODE_USER == Enabled AND FIFO_MODE != NONE and both modes are active (IER\\[7\\]
set to one and FCR\\[0\\]
set to one respectively), the functionality is switched to indicate the transmitter FIFO is full, and no longer controls THRE interrupts, which are then controlled by the FCR\\[5:4\\]
threshold setting. For more details, see “Programmable THRE Interrupt” on page 54."]
    #[inline(always)]
    pub fn thre(&self) -> ThreR {
        ThreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty bit. If in FIFO mode (FIFO_MODE != NONE) and FIFOs enabled (FCR\\[0\\]
set to one), this bit is set whenever the Transmitter Shift Register and the FIFO are both empty. If in non-FIFO mode or FIFOs are disabled, this bit is set whenever the Transmitter Holding Register and the Transmitter Shift Register are both empty."]
    #[inline(always)]
    pub fn temt(&self) -> TemtR {
        TemtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receiver FIFO Error bit. This bit is only relevant when FIFO_MODE != NONE AND FIFOs are enabled (FCR\\[0\\]
set to one). This is used to indicate if there is at least one parity error, framing error, or break indication in the FIFO. 0 = no error in RX FIFO 1 = error in RX FIFO This bit is cleared when the LSR is read and the character with the error is at the top of the receiver FIFO and there are no subsequent errors in the FIFO."]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Line Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`reset()` method sets lsr to value 0"]
impl crate::Resettable for LsrSpec {
    const RESET_VALUE: u32 = 0;
}
