#[doc = "Register `sfe` reader"]
pub type R = crate::R<SfeSpec>;
#[doc = "Register `sfe` writer"]
pub type W = crate::W<SfeSpec>;
#[doc = "Field `sfe` reader - Shadow FIFO Enable. This is a shadow register for the FIFO enable bit (FCR\\[0\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the FIFO enable bit gets updated.This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. If this bit is set to zero (disabled) after being enabled then both the XMIT and RCVR controller portion of FIFOs are reset."]
pub type SfeR = crate::BitReader;
#[doc = "Field `sfe` writer - Shadow FIFO Enable. This is a shadow register for the FIFO enable bit (FCR\\[0\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the FIFO enable bit gets updated.This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. If this bit is set to zero (disabled) after being enabled then both the XMIT and RCVR controller portion of FIFOs are reset."]
pub type SfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shadow FIFO Enable. This is a shadow register for the FIFO enable bit (FCR\\[0\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the FIFO enable bit gets updated.This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. If this bit is set to zero (disabled) after being enabled then both the XMIT and RCVR controller portion of FIFOs are reset."]
    #[inline(always)]
    pub fn sfe(&self) -> SfeR {
        SfeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow FIFO Enable. This is a shadow register for the FIFO enable bit (FCR\\[0\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the FIFO enable bit gets updated.This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. If this bit is set to zero (disabled) after being enabled then both the XMIT and RCVR controller portion of FIFOs are reset."]
    #[inline(always)]
    #[must_use]
    pub fn sfe(&mut self) -> SfeW<SfeSpec> {
        SfeW::new(self, 0)
    }
}
#[doc = "Shadow FIFO Enable: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfeSpec;
impl crate::RegisterSpec for SfeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfe::R`](R) reader structure"]
impl crate::Readable for SfeSpec {}
#[doc = "`write(|w| ..)` method takes [`sfe::W`](W) writer structure"]
impl crate::Writable for SfeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sfe to value 0"]
impl crate::Resettable for SfeSpec {
    const RESET_VALUE: u32 = 0;
}
