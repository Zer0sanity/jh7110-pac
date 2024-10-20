#[doc = "Register `sbcr` reader"]
pub type R = crate::R<SbcrSpec>;
#[doc = "Register `sbcr` writer"]
pub type W = crate::W<SbcrSpec>;
#[doc = "Field `sbcr` reader - Shadow Break Control Bit. This is a shadow register for the Break bit (LCR\\[6\\]), this can be used to remove the burden of having to performing a read modify write on the LCR. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If SIR_MODE == Enabled and active (MCR\\[6\\]
= 1) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver."]
pub type SbcrR = crate::BitReader;
#[doc = "Field `sbcr` writer - Shadow Break Control Bit. This is a shadow register for the Break bit (LCR\\[6\\]), this can be used to remove the burden of having to performing a read modify write on the LCR. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If SIR_MODE == Enabled and active (MCR\\[6\\]
= 1) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver."]
pub type SbcrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shadow Break Control Bit. This is a shadow register for the Break bit (LCR\\[6\\]), this can be used to remove the burden of having to performing a read modify write on the LCR. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If SIR_MODE == Enabled and active (MCR\\[6\\]
= 1) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver."]
    #[inline(always)]
    pub fn sbcr(&self) -> SbcrR {
        SbcrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow Break Control Bit. This is a shadow register for the Break bit (LCR\\[6\\]), this can be used to remove the burden of having to performing a read modify write on the LCR. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If SIR_MODE == Enabled and active (MCR\\[6\\]
= 1) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver."]
    #[inline(always)]
    #[must_use]
    pub fn sbcr(&mut self) -> SbcrW<SbcrSpec> {
        SbcrW::new(self, 0)
    }
}
#[doc = "Shadow Break Control Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sbcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sbcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SbcrSpec;
impl crate::RegisterSpec for SbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbcr::R`](R) reader structure"]
impl crate::Readable for SbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sbcr::W`](W) writer structure"]
impl crate::Writable for SbcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sbcr to value 0"]
impl crate::Resettable for SbcrSpec {
    const RESET_VALUE: u32 = 0;
}
