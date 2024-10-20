#[doc = "Register `rfw` writer"]
pub type W = crate::W<RfwSpec>;
#[doc = "Field `rfwd` writer - Receive FIFO Write Data. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, the data that is written to the RFWD is pushed into the receive FIFO. Each consecutive write pushes the new data to the next write location in the receive FIFO. When FIFOs are not implemented or not enabled, the data that is written to the RFWD is pushed into the RBR."]
pub type RfwdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rfpe` writer - Receive FIFO Parity Error. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, this bit is used to write parity error detection information to the receive FIFO. When FIFOs are not implemented or not enabled, this bit is used to write parity error detection information to the RBR."]
pub type RfpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rffe` writer - Receive FIFO Framing Error. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, this bit is used to write framing error detection information to the receive FIFO. When FIFOs are not implemented or not enabled, this bit is used to write framing error detection information to the RBR."]
pub type RffeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:7 - Receive FIFO Write Data. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, the data that is written to the RFWD is pushed into the receive FIFO. Each consecutive write pushes the new data to the next write location in the receive FIFO. When FIFOs are not implemented or not enabled, the data that is written to the RFWD is pushed into the RBR."]
    #[inline(always)]
    #[must_use]
    pub fn rfwd(&mut self) -> RfwdW<RfwSpec> {
        RfwdW::new(self, 0)
    }
    #[doc = "Bit 8 - Receive FIFO Parity Error. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, this bit is used to write parity error detection information to the receive FIFO. When FIFOs are not implemented or not enabled, this bit is used to write parity error detection information to the RBR."]
    #[inline(always)]
    #[must_use]
    pub fn rfpe(&mut self) -> RfpeW<RfwSpec> {
        RfpeW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive FIFO Framing Error. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, this bit is used to write framing error detection information to the receive FIFO. When FIFOs are not implemented or not enabled, this bit is used to write framing error detection information to the RBR."]
    #[inline(always)]
    #[must_use]
    pub fn rffe(&mut self) -> RffeW<RfwSpec> {
        RffeW::new(self, 9)
    }
}
#[doc = "Receive FIFO Write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfw::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfwSpec;
impl crate::RegisterSpec for RfwSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rfw::W`](W) writer structure"]
impl crate::Writable for RfwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rfw to value 0"]
impl crate::Resettable for RfwSpec {
    const RESET_VALUE: u32 = 0;
}
