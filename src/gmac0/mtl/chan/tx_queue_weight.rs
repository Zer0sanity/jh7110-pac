#[doc = "Register `tx_queue_weight` reader"]
pub type R = crate::R<TxQueueWeightSpec>;
#[doc = "Register `tx_queue_weight` writer"]
pub type W = crate::W<TxQueueWeightSpec>;
#[doc = "Field `iscqw` reader - MTL Channel ISC Queue Weight"]
pub type IscqwR = crate::FieldReader<u32>;
#[doc = "Field `iscqw` writer - MTL Channel ISC Queue Weight"]
pub type IscqwW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - MTL Channel ISC Queue Weight"]
    #[inline(always)]
    pub fn iscqw(&self) -> IscqwR {
        IscqwR::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - MTL Channel ISC Queue Weight"]
    #[inline(always)]
    #[must_use]
    pub fn iscqw(&mut self) -> IscqwW<TxQueueWeightSpec> {
        IscqwW::new(self, 0)
    }
}
#[doc = "MTL Channel TX Queue Weight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_queue_weight::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_queue_weight::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxQueueWeightSpec;
impl crate::RegisterSpec for TxQueueWeightSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_queue_weight::R`](R) reader structure"]
impl crate::Readable for TxQueueWeightSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_queue_weight::W`](W) writer structure"]
impl crate::Writable for TxQueueWeightSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tx_queue_weight to value 0"]
impl crate::Resettable for TxQueueWeightSpec {
    const RESET_VALUE: u32 = 0;
}
