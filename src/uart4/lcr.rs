#[doc = "Register `lcr` reader"]
pub type R = crate::R<LcrSpec>;
#[doc = "Register `lcr` writer"]
pub type W = crate::W<LcrSpec>;
#[doc = "Field `dls` reader - Data Length Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows: 00 = 5 bits 01 = 6 bits 10 = 7 bits 11 = 8 bits"]
pub type DlsR = crate::FieldReader;
#[doc = "Field `dls` writer - Data Length Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows: 00 = 5 bits 01 = 6 bits 10 = 7 bits 11 = 8 bits"]
pub type DlsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `stop` reader - Number of stop bits. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data. If set to one and the data bits are set to 5 (LCR\\[1:0\\]
set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit. 0 = 1 stop bit 1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit"]
pub type StopR = crate::BitReader;
#[doc = "Field `stop` writer - Number of stop bits. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data. If set to one and the data bits are set to 5 (LCR\\[1:0\\]
set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit. 0 = 1 stop bit 1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pen` reader - Parity Enable. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively. 0 = parity disabled 1 = parity enabled"]
pub type PenR = crate::BitReader;
#[doc = "Field `pen` writer - Parity Enable. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively. 0 = parity disabled 1 = parity enabled"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `eps` reader - Even Parity Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
pub type EpsR = crate::BitReader;
#[doc = "Field `eps` writer - Even Parity Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bc` reader - Break Control Bit.This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If SIR_MODE == Enabled and active (MCR\\[6\\]
set to one) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
pub type BcR = crate::BitReader;
#[doc = "Field `bc` writer - Break Control Bit.This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If SIR_MODE == Enabled and active (MCR\\[6\\]
set to one) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
pub type BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dlab` reader - Divisor Latch Access Bit. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART. This bit must be cleared after initial baud rate setup in order to access other registers."]
pub type DlabR = crate::BitReader;
#[doc = "Field `dlab` writer - Divisor Latch Access Bit. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART. This bit must be cleared after initial baud rate setup in order to access other registers."]
pub type DlabW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Data Length Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows: 00 = 5 bits 01 = 6 bits 10 = 7 bits 11 = 8 bits"]
    #[inline(always)]
    pub fn dls(&self) -> DlsR {
        DlsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Number of stop bits. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data. If set to one and the data bits are set to 5 (LCR\\[1:0\\]
set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit. 0 = 1 stop bit 1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively. 0 = parity disabled 1 = parity enabled"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Even Parity Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Break Control Bit.This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If SIR_MODE == Enabled and active (MCR\\[6\\]
set to one) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART. This bit must be cleared after initial baud rate setup in order to access other registers."]
    #[inline(always)]
    pub fn dlab(&self) -> DlabR {
        DlabR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data Length Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows: 00 = 5 bits 01 = 6 bits 10 = 7 bits 11 = 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn dls(&mut self) -> DlsW<LcrSpec> {
        DlsW::new(self, 0)
    }
    #[doc = "Bit 2 - Number of stop bits. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data. If set to one and the data bits are set to 5 (LCR\\[1:0\\]
set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit. 0 = 1 stop bit 1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<LcrSpec> {
        StopW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity Enable. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively. 0 = parity disabled 1 = parity enabled"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<LcrSpec> {
        PenW::new(self, 3)
    }
    #[doc = "Bit 4 - Even Parity Select. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
    #[inline(always)]
    #[must_use]
    pub fn eps(&mut self) -> EpsW<LcrSpec> {
        EpsW::new(self, 4)
    }
    #[doc = "Bit 6 - Break Control Bit.This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If SIR_MODE == Enabled and active (MCR\\[6\\]
set to one) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
    #[inline(always)]
    #[must_use]
    pub fn bc(&mut self) -> BcW<LcrSpec> {
        BcW::new(self, 6)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit. Writeable only when UART is not busy (USR\\[0\\]
is zero), always readable. This bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART. This bit must be cleared after initial baud rate setup in order to access other registers."]
    #[inline(always)]
    #[must_use]
    pub fn dlab(&mut self) -> DlabW<LcrSpec> {
        DlabW::new(self, 7)
    }
}
#[doc = "Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrSpec;
impl crate::RegisterSpec for LcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lcr to value 0"]
impl crate::Resettable for LcrSpec {
    const RESET_VALUE: u32 = 0;
}
