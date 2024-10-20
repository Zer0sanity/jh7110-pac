#[doc = "Register `status[%s]` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `seq_done_event` reader - Sequence complete."]
pub type SeqDoneEventR = crate::BitReader;
#[doc = "Field `hw_req_event` reader - Hardware encouragement request."]
pub type HwReqEventR = crate::BitReader;
#[doc = "Field `sw_fail_event` reader - Software encouragement failure."]
pub type SwFailEventR = crate::FieldReader;
#[doc = "Field `hw_fail_event` reader - Hardware encouragement failure."]
pub type HwFailEventR = crate::FieldReader;
#[doc = "Field `pch_fail_event` reader - P-channel failure."]
pub type PchFailEventR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Sequence complete."]
    #[inline(always)]
    pub fn seq_done_event(&self) -> SeqDoneEventR {
        SeqDoneEventR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hardware encouragement request."]
    #[inline(always)]
    pub fn hw_req_event(&self) -> HwReqEventR {
        HwReqEventR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Software encouragement failure."]
    #[inline(always)]
    pub fn sw_fail_event(&self) -> SwFailEventR {
        SwFailEventR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Hardware encouragement failure."]
    #[inline(always)]
    pub fn hw_fail_event(&self) -> HwFailEventR {
        HwFailEventR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:8 - P-channel failure."]
    #[inline(always)]
    pub fn pch_fail_event(&self) -> PchFailEventR {
        PchFailEventR::new(((self.bits >> 6) & 7) as u8)
    }
}
#[doc = "Event and Interrupt Status registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets status[%s]
to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
