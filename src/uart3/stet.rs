#[doc = "Register `stet` reader"]
pub type R = crate::R<StetSpec>;
#[doc = "Register `stet` writer"]
pub type W = crate::W<StetSpec>;
#[doc = "Field `stet` reader - Shadow TX Empty Trigger. This is a shadow register for the TX empty trigger bits (FCR\\[5:4\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the TX empty trigger bit gets updated. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO ¼ full 11 = FIFO ½ full"]
pub type StetR = crate::FieldReader;
#[doc = "Field `stet` writer - Shadow TX Empty Trigger. This is a shadow register for the TX empty trigger bits (FCR\\[5:4\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the TX empty trigger bit gets updated. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO ¼ full 11 = FIFO ½ full"]
pub type StetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Shadow TX Empty Trigger. This is a shadow register for the TX empty trigger bits (FCR\\[5:4\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the TX empty trigger bit gets updated. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO ¼ full 11 = FIFO ½ full"]
    #[inline(always)]
    pub fn stet(&self) -> StetR {
        StetR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Shadow TX Empty Trigger. This is a shadow register for the TX empty trigger bits (FCR\\[5:4\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the TX empty trigger bit gets updated. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO ¼ full 11 = FIFO ½ full"]
    #[inline(always)]
    #[must_use]
    pub fn stet(&mut self) -> StetW<StetSpec> {
        StetW::new(self, 0)
    }
}
#[doc = "Shadow TX Empty Trigger: This register is only valid when the DW_apb_uart is configured to have FIFOs implemented (FIFO_MODE != NONE) and THRE interrupt support implemented (THRE_MODE_USER == Enabled) and additional shadow registers implemented (SHADOW == YES). If FIFOs are not implemented or THRE interrupt support is not implemented or shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stet::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stet::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StetSpec;
impl crate::RegisterSpec for StetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stet::R`](R) reader structure"]
impl crate::Readable for StetSpec {}
#[doc = "`write(|w| ..)` method takes [`stet::W`](W) writer structure"]
impl crate::Writable for StetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stet to value 0"]
impl crate::Resettable for StetSpec {
    const RESET_VALUE: u32 = 0;
}
