#[doc = "Register `event_counter[%s]` reader"]
pub type R = crate::R<EventCounterSpec>;
#[doc = "Field `counter` reader - L2PM Event Counter."]
pub type CounterR = crate::FieldReader<u64>;
impl R {
    #[doc = "Bits 0:63 - L2PM Event Counter."]
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new(self.bits)
    }
}
#[doc = "L2PM Event Control Event Select configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_counter::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventCounterSpec;
impl crate::RegisterSpec for EventCounterSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`event_counter::R`](R) reader structure"]
impl crate::Readable for EventCounterSpec {}
#[doc = "`reset()` method sets event_counter[%s]
to value 0"]
impl crate::Resettable for EventCounterSpec {
    const RESET_VALUE: u64 = 0;
}
