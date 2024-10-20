#[doc = "Register `dmasa` writer"]
pub type W = crate::W<DmasaSpec>;
#[doc = "Field `dmasa` writer - This register is use to perform a DMA software acknowledge if a transfer needs to be terminated due to an error condition. For example, if the DMA disables the channel, then the DW_apb_uart should clear its request. This causes the TX request, TX single, RX request and RX single signals to de-assert. Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub type DmasaW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - This register is use to perform a DMA software acknowledge if a transfer needs to be terminated due to an error condition. For example, if the DMA disables the channel, then the DW_apb_uart should clear its request. This causes the TX request, TX single, RX request and RX single signals to de-assert. Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn dmasa(&mut self) -> DmasaW<DmasaSpec> {
        DmasaW::new(self, 0)
    }
}
#[doc = "DMA Software Acknowledge\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasa::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmasaSpec;
impl crate::RegisterSpec for DmasaSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmasa::W`](W) writer structure"]
impl crate::Writable for DmasaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dmasa to value 0"]
impl crate::Resettable for DmasaSpec {
    const RESET_VALUE: u32 = 0;
}
