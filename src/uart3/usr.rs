#[doc = "Register `usr` reader"]
pub type R = crate::R<UsrSpec>;
#[doc = "Field `busy` reader - UART Busy. This is indicates that a serial transfer is in progress, when cleared indicates that the DW_apb_uart is idle or inactive. 0 = DW_apb_uart is idle or inactive 1 = DW_apb_uart is busy (actively transferring data) NOTE: It is possible for the UART Busy bit to be cleared even though a new character may have been sent from another device. That is, if the DW_apb_uart has no data in THR and RBR and there is no transmission in progress and a start bit of a new character has just reached the DW_apb_uart. This is due to the fact that a valid start is not seen until the middle of the bit period and this duration is dependent on the baud divisor that has been programmed. If a second system clock has been implemented (CLOCK_MODE == Enabled), the assertion of this bit is also delayed by several cycles of the slower clock."]
pub type BusyR = crate::BitReader;
#[doc = "Field `tfnf` reader - Transmit FIFO Not Full. This bit is only valid when FIFO_STAT == YES. This is used to indicate that the transmit FIFO in not full. 0 = Transmit FIFO is full 1 = Transmit FIFO is not full This bit is cleared when the TX FIFO is full."]
pub type TfnfR = crate::BitReader;
#[doc = "Field `tfe` reader - Transmit FIFO Empty. This bit is only valid when FIFO_STAT == YES. This is used to indicate that the transmit FIFO is completely empty. 0 = Transmit FIFO is not empty 1 = Transmit FIFO is empty This bit is cleared when the TX FIFO is no longer empty."]
pub type TfeR = crate::BitReader;
#[doc = "Field `rfne` reader - Receive FIFO Not Empty. This bit is only valid when FIFO_STAT == YES. This is used to indicate that the receive FIFO contains one or more entries. 0 = Receive FIFO is empty 1 = Receive FIFO is not empty This bit is cleared when the RX FIFO is empty."]
pub type RfneR = crate::BitReader;
#[doc = "Field `rff` reader - Receive FIFO Full. This bit is only valid when FIFO_STAT == YES. This is used to indicate that the receive FIFO is completely full. 0 = Receive FIFO not full 1 = Receive FIFO Full This bit is cleared when the RX FIFO is no longer full."]
pub type RffR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - UART Busy. This is indicates that a serial transfer is in progress, when cleared indicates that the DW_apb_uart is idle or inactive. 0 = DW_apb_uart is idle or inactive 1 = DW_apb_uart is busy (actively transferring data) NOTE: It is possible for the UART Busy bit to be cleared even though a new character may have been sent from another device. That is, if the DW_apb_uart has no data in THR and RBR and there is no transmission in progress and a start bit of a new character has just reached the DW_apb_uart. This is due to the fact that a valid start is not seen until the middle of the bit period and this duration is dependent on the baud divisor that has been programmed. If a second system clock has been implemented (CLOCK_MODE == Enabled), the assertion of this bit is also delayed by several cycles of the slower clock."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full. This bit is only valid when FIFO_STAT == YES. This is used to indicate that the transmit FIFO in not full. 0 = Transmit FIFO is full 1 = Transmit FIFO is not full This bit is cleared when the TX FIFO is full."]
    #[inline(always)]
    pub fn tfnf(&self) -> TfnfR {
        TfnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Empty. This bit is only valid when FIFO_STAT == YES. This is used to indicate that the transmit FIFO is completely empty. 0 = Transmit FIFO is not empty 1 = Transmit FIFO is empty This bit is cleared when the TX FIFO is no longer empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Not Empty. This bit is only valid when FIFO_STAT == YES. This is used to indicate that the receive FIFO contains one or more entries. 0 = Receive FIFO is empty 1 = Receive FIFO is not empty This bit is cleared when the RX FIFO is empty."]
    #[inline(always)]
    pub fn rfne(&self) -> RfneR {
        RfneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full. This bit is only valid when FIFO_STAT == YES. This is used to indicate that the receive FIFO is completely full. 0 = Receive FIFO not full 1 = Receive FIFO Full This bit is cleared when the RX FIFO is no longer full."]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "UART Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsrSpec;
impl crate::RegisterSpec for UsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usr::R`](R) reader structure"]
impl crate::Readable for UsrSpec {}
#[doc = "`reset()` method sets usr to value 0"]
impl crate::Resettable for UsrSpec {
    const RESET_VALUE: u32 = 0;
}
