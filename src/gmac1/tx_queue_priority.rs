#[doc = "Register `tx_queue_priority[%s]` reader"]
pub type R = crate::R<TxQueuePrioritySpec>;
#[doc = "Register `tx_queue_priority[%s]` writer"]
pub type W = crate::W<TxQueuePrioritySpec>;
#[doc = "Field `priority(0-3)` reader - Tranmission Queue Priority"]
pub type PriorityR = crate::FieldReader;
#[doc = "Field `priority(0-3)` writer - Tranmission Queue Priority"]
pub type PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Tranmission Queue Priority"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `priority0` field"]
    #[inline(always)]
    pub fn priority(&self, n: u8) -> PriorityR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        PriorityR::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Tranmission Queue Priority"]
    #[inline(always)]
    pub fn priority_iter(&self) -> impl Iterator<Item = PriorityR> + '_ {
        (0..4).map(move |n| PriorityR::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - Tranmission Queue Priority"]
    #[inline(always)]
    pub fn priority0(&self) -> PriorityR {
        PriorityR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Tranmission Queue Priority"]
    #[inline(always)]
    pub fn priority1(&self) -> PriorityR {
        PriorityR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Tranmission Queue Priority"]
    #[inline(always)]
    pub fn priority2(&self) -> PriorityR {
        PriorityR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Tranmission Queue Priority"]
    #[inline(always)]
    pub fn priority3(&self) -> PriorityR {
        PriorityR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Tranmission Queue Priority"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `priority0` field"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self, n: u8) -> PriorityW<TxQueuePrioritySpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        PriorityW::new(self, n * 8)
    }
    #[doc = "Bits 0:7 - Tranmission Queue Priority"]
    #[inline(always)]
    #[must_use]
    pub fn priority0(&mut self) -> PriorityW<TxQueuePrioritySpec> {
        PriorityW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Tranmission Queue Priority"]
    #[inline(always)]
    #[must_use]
    pub fn priority1(&mut self) -> PriorityW<TxQueuePrioritySpec> {
        PriorityW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Tranmission Queue Priority"]
    #[inline(always)]
    #[must_use]
    pub fn priority2(&mut self) -> PriorityW<TxQueuePrioritySpec> {
        PriorityW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Tranmission Queue Priority"]
    #[inline(always)]
    #[must_use]
    pub fn priority3(&mut self) -> PriorityW<TxQueuePrioritySpec> {
        PriorityW::new(self, 24)
    }
}
#[doc = "MAC TX Queue Priority - tx_queue_priority0: queue 0-3, tx_queue_priority1: queue 4-7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_queue_priority::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_queue_priority::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxQueuePrioritySpec;
impl crate::RegisterSpec for TxQueuePrioritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_queue_priority::R`](R) reader structure"]
impl crate::Readable for TxQueuePrioritySpec {}
#[doc = "`write(|w| ..)` method takes [`tx_queue_priority::W`](W) writer structure"]
impl crate::Writable for TxQueuePrioritySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tx_queue_priority[%s]
to value 0"]
impl crate::Resettable for TxQueuePrioritySpec {
    const RESET_VALUE: u32 = 0;
}
