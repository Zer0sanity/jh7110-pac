#[doc = "Register `tx_queue_flow_ctrl[%s]` reader"]
pub type R = crate::R<TxQueueFlowCtrlSpec>;
#[doc = "Register `tx_queue_flow_ctrl[%s]` writer"]
pub type W = crate::W<TxQueueFlowCtrlSpec>;
#[doc = "Field `tfe` reader - Tranmission Flow Enable"]
pub type TfeR = crate::BitReader;
#[doc = "Field `tfe` writer - Tranmission Flow Enable"]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt` reader - Pause Time"]
pub type PtR = crate::FieldReader<u16>;
#[doc = "Field `pt` writer - Pause Time"]
pub type PtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 1 - Tranmission Flow Enable"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - Tranmission Flow Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TfeW<TxQueueFlowCtrlSpec> {
        TfeW::new(self, 1)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PtW<TxQueueFlowCtrlSpec> {
        PtW::new(self, 16)
    }
}
#[doc = "MAC TX Queue Flow Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_queue_flow_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_queue_flow_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxQueueFlowCtrlSpec;
impl crate::RegisterSpec for TxQueueFlowCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_queue_flow_ctrl::R`](R) reader structure"]
impl crate::Readable for TxQueueFlowCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_queue_flow_ctrl::W`](W) writer structure"]
impl crate::Writable for TxQueueFlowCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tx_queue_flow_ctrl[%s]
to value 0"]
impl crate::Resettable for TxQueueFlowCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
