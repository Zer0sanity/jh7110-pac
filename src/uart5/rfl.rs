#[doc = "Register `rfl` reader"]
pub type R = crate::R<RflSpec>;
#[doc = "Field `rfl` reader - Receive FIFO Level. This is indicates the number of data entries in the receive FIFO."]
pub type RflR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive FIFO Level. This is indicates the number of data entries in the receive FIFO."]
    #[inline(always)]
    pub fn rfl(&self) -> RflR {
        RflR::new(self.bits)
    }
}
#[doc = "Receive FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RflSpec;
impl crate::RegisterSpec for RflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfl::R`](R) reader structure"]
impl crate::Readable for RflSpec {}
#[doc = "`reset()` method sets rfl to value 0"]
impl crate::Resettable for RflSpec {
    const RESET_VALUE: u32 = 0;
}
