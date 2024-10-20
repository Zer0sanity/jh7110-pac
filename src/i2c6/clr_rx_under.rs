#[doc = "Register `clr_rx_under` reader"]
pub type R = crate::R<ClrRxUnderSpec>;
#[doc = "Register `clr_rx_under` writer"]
pub type W = crate::W<ClrRxUnderSpec>;
#[doc = "Field `clr_rx_under` reader - "]
pub type ClrRxUnderR = crate::FieldReader<u32>;
#[doc = "Field `clr_rx_under` writer - "]
pub type ClrRxUnderW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clr_rx_under(&self) -> ClrRxUnderR {
        ClrRxUnderR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rx_under(&mut self) -> ClrRxUnderW<ClrRxUnderSpec> {
        ClrRxUnderW::new(self, 0)
    }
}
#[doc = "DesignWare I2C Clear RX Underrun\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rx_under::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rx_under::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrRxUnderSpec;
impl crate::RegisterSpec for ClrRxUnderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_rx_under::R`](R) reader structure"]
impl crate::Readable for ClrRxUnderSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_rx_under::W`](W) writer structure"]
impl crate::Writable for ClrRxUnderSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clr_rx_under to value 0"]
impl crate::Resettable for ClrRxUnderSpec {
    const RESET_VALUE: u32 = 0;
}
