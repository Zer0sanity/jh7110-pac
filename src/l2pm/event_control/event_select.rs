#[doc = "Register `event_select[%s]` reader"]
pub type R = crate::R<EventSelectSpec>;
#[doc = "Register `event_select[%s]` writer"]
pub type W = crate::W<EventSelectSpec>;
#[doc = "L2PM Event Class.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EventClass {
    #[doc = "1: L2PM transaction events."]
    Transaction = 1,
    #[doc = "2: L2PM L2 query result events."]
    L2QueryResult = 2,
    #[doc = "3: L2PM L2 request events."]
    L2Request = 3,
}
impl From<EventClass> for u8 {
    #[inline(always)]
    fn from(variant: EventClass) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EventClass {
    type Ux = u8;
}
#[doc = "Field `event_class` reader - L2PM Event Class."]
pub type EventClassR = crate::FieldReader<EventClass>;
impl EventClassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EventClass> {
        match self.bits {
            1 => Some(EventClass::Transaction),
            2 => Some(EventClass::L2QueryResult),
            3 => Some(EventClass::L2Request),
            _ => None,
        }
    }
    #[doc = "L2PM transaction events."]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        *self == EventClass::Transaction
    }
    #[doc = "L2PM L2 query result events."]
    #[inline(always)]
    pub fn is_l2_query_result(&self) -> bool {
        *self == EventClass::L2QueryResult
    }
    #[doc = "L2PM L2 request events."]
    #[inline(always)]
    pub fn is_l2_request(&self) -> bool {
        *self == EventClass::L2Request
    }
}
#[doc = "Field `event_class` writer - L2PM Event Class."]
pub type EventClassW<'a, REG> = crate::FieldWriter<'a, REG, 8, EventClass>;
impl<'a, REG> EventClassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "L2PM transaction events."]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut crate::W<REG> {
        self.variant(EventClass::Transaction)
    }
    #[doc = "L2PM L2 query result events."]
    #[inline(always)]
    pub fn l2_query_result(self) -> &'a mut crate::W<REG> {
        self.variant(EventClass::L2QueryResult)
    }
    #[doc = "L2PM L2 request events."]
    #[inline(always)]
    pub fn l2_request(self) -> &'a mut crate::W<REG> {
        self.variant(EventClass::L2Request)
    }
}
#[doc = "Field `event_mask` reader - L2PM Event Mask for specifying the event type according to its `event_class`."]
pub type EventMaskR = crate::FieldReader<u64>;
#[doc = "Field `event_mask` writer - L2PM Event Mask for specifying the event type according to its `event_class`."]
pub type EventMaskW<'a, REG> = crate::FieldWriter<'a, REG, 56, u64>;
impl R {
    #[doc = "Bits 0:7 - L2PM Event Class."]
    #[inline(always)]
    pub fn event_class(&self) -> EventClassR {
        EventClassR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:63 - L2PM Event Mask for specifying the event type according to its `event_class`."]
    #[inline(always)]
    pub fn event_mask(&self) -> EventMaskR {
        EventMaskR::new((self.bits >> 8) & 0x00ff_ffff_ffff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - L2PM Event Class."]
    #[inline(always)]
    #[must_use]
    pub fn event_class(&mut self) -> EventClassW<EventSelectSpec> {
        EventClassW::new(self, 0)
    }
    #[doc = "Bits 8:63 - L2PM Event Mask for specifying the event type according to its `event_class`."]
    #[inline(always)]
    #[must_use]
    pub fn event_mask(&mut self) -> EventMaskW<EventSelectSpec> {
        EventMaskW::new(self, 8)
    }
}
#[doc = "L2PM Event Control Event Select configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event_select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventSelectSpec;
impl crate::RegisterSpec for EventSelectSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`event_select::R`](R) reader structure"]
impl crate::Readable for EventSelectSpec {}
#[doc = "`write(|w| ..)` method takes [`event_select::W`](W) writer structure"]
impl crate::Writable for EventSelectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets event_select[%s]
to value 0x01"]
impl crate::Resettable for EventSelectSpec {
    const RESET_VALUE: u64 = 0x01;
}
