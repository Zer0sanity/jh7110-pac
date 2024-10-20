#[doc = "Register `rbr` reader"]
pub type R = crate::R<RbrSpec>;
#[doc = "Field `rbr` reader - Data byte received on the serial input port (sin) in UART mode, or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line Status Register (LCR) is set. If in non-FIFO mode (FIFO_MODE == NONE) or FIFOs are disabled (FCR\\[0\\]
set to zero), the data in the RBR must be read before the next data arrives, otherwise it is overwritten, resulting in an over-run error. If in FIFO mode (FIFO_MODE != NONE) and FIFOs are enabled (FCR\\[0\\]
set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO is preserved, but any incoming data are lost and an over-run error occurs."]
pub type RbrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Data byte received on the serial input port (sin) in UART mode, or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line Status Register (LCR) is set. If in non-FIFO mode (FIFO_MODE == NONE) or FIFOs are disabled (FCR\\[0\\]
set to zero), the data in the RBR must be read before the next data arrives, otherwise it is overwritten, resulting in an over-run error. If in FIFO mode (FIFO_MODE != NONE) and FIFOs are enabled (FCR\\[0\\]
set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO is preserved, but any incoming data are lost and an over-run error occurs."]
    #[inline(always)]
    pub fn rbr(&self) -> RbrR {
        RbrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receive Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbrSpec;
impl crate::RegisterSpec for RbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbr::R`](R) reader structure"]
impl crate::Readable for RbrSpec {}
#[doc = "`reset()` method sets rbr to value 0"]
impl crate::Resettable for RbrSpec {
    const RESET_VALUE: u32 = 0;
}
