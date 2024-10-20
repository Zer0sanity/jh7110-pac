#[doc = "Register `tfl` reader"]
pub type R = crate::R<TflSpec>;
#[doc = "Field `tfl` reader - Transmit FIFO Level. This is indicates the number of data entries in the transmit FIFO."]
pub type TflR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit FIFO Level. This is indicates the number of data entries in the transmit FIFO."]
    #[inline(always)]
    pub fn tfl(&self) -> TflR {
        TflR::new(self.bits)
    }
}
#[doc = "Transmit FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TflSpec;
impl crate::RegisterSpec for TflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfl::R`](R) reader structure"]
impl crate::Readable for TflSpec {}
#[doc = "`reset()` method sets tfl to value 0"]
impl crate::Resettable for TflSpec {
    const RESET_VALUE: u32 = 0;
}
