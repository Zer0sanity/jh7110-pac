#[doc = "Register `rx_flow_ctrl` reader"]
pub type R = crate::R<RxFlowCtrlSpec>;
#[doc = "Register `rx_flow_ctrl` writer"]
pub type W = crate::W<RxFlowCtrlSpec>;
#[doc = "Field `rfe` reader - Receive Flow Enable"]
pub type RfeR = crate::BitReader;
#[doc = "Field `rfe` writer - Receive Flow Enable"]
pub type RfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive Flow Enable"]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Flow Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RfeW<RxFlowCtrlSpec> {
        RfeW::new(self, 0)
    }
}
#[doc = "MAC RX Flow Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_flow_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_flow_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFlowCtrlSpec;
impl crate::RegisterSpec for RxFlowCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_flow_ctrl::R`](R) reader structure"]
impl crate::Readable for RxFlowCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_flow_ctrl::W`](W) writer structure"]
impl crate::Writable for RxFlowCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rx_flow_ctrl to value 0"]
impl crate::Resettable for RxFlowCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
