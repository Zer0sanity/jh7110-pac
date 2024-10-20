#[doc = "Register `clr_rx_over` reader"]
pub type R = crate::R<ClrRxOverSpec>;
#[doc = "Register `clr_rx_over` writer"]
pub type W = crate::W<ClrRxOverSpec>;
#[doc = "Field `clr_rx_over` reader - "]
pub type ClrRxOverR = crate::FieldReader<u32>;
#[doc = "Field `clr_rx_over` writer - "]
pub type ClrRxOverW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clr_rx_over(&self) -> ClrRxOverR {
        ClrRxOverR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rx_over(&mut self) -> ClrRxOverW<ClrRxOverSpec> {
        ClrRxOverW::new(self, 0)
    }
}
#[doc = "DesignWare I2C Clear RX Overrun\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rx_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rx_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrRxOverSpec;
impl crate::RegisterSpec for ClrRxOverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_rx_over::R`](R) reader structure"]
impl crate::Readable for ClrRxOverSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_rx_over::W`](W) writer structure"]
impl crate::Writable for ClrRxOverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clr_rx_over to value 0"]
impl crate::Resettable for ClrRxOverSpec {
    const RESET_VALUE: u32 = 0;
}
