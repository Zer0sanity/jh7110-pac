#[doc = "Register `clr_rx_done` reader"]
pub type R = crate::R<ClrRxDoneSpec>;
#[doc = "Register `clr_rx_done` writer"]
pub type W = crate::W<ClrRxDoneSpec>;
#[doc = "Field `clr_rx_done` reader - "]
pub type ClrRxDoneR = crate::FieldReader<u32>;
#[doc = "Field `clr_rx_done` writer - "]
pub type ClrRxDoneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clr_rx_done(&self) -> ClrRxDoneR {
        ClrRxDoneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rx_done(&mut self) -> ClrRxDoneW<ClrRxDoneSpec> {
        ClrRxDoneW::new(self, 0)
    }
}
#[doc = "DesignWare I2C Clear RX Done\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rx_done::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rx_done::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrRxDoneSpec;
impl crate::RegisterSpec for ClrRxDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_rx_done::R`](R) reader structure"]
impl crate::Readable for ClrRxDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_rx_done::W`](W) writer structure"]
impl crate::Writable for ClrRxDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clr_rx_done to value 0"]
impl crate::Resettable for ClrRxDoneSpec {
    const RESET_VALUE: u32 = 0;
}
