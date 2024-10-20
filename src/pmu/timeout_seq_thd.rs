#[doc = "Register `timeout_seq_thd` reader"]
pub type R = crate::R<TimeoutSeqThdSpec>;
#[doc = "Register `timeout_seq_thd` writer"]
pub type W = crate::W<TimeoutSeqThdSpec>;
#[doc = "Field `timeout_seq_thd` reader - "]
pub type TimeoutSeqThdR = crate::FieldReader<u16>;
#[doc = "Field `timeout_seq_thd` writer - "]
pub type TimeoutSeqThdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn timeout_seq_thd(&self) -> TimeoutSeqThdR {
        TimeoutSeqThdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_seq_thd(&mut self) -> TimeoutSeqThdW<TimeoutSeqThdSpec> {
        TimeoutSeqThdW::new(self, 0)
    }
}
#[doc = "Timeout Sequence Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout_seq_thd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout_seq_thd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutSeqThdSpec;
impl crate::RegisterSpec for TimeoutSeqThdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout_seq_thd::R`](R) reader structure"]
impl crate::Readable for TimeoutSeqThdSpec {}
#[doc = "`write(|w| ..)` method takes [`timeout_seq_thd::W`](W) writer structure"]
impl crate::Writable for TimeoutSeqThdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets timeout_seq_thd to value 0"]
impl crate::Resettable for TimeoutSeqThdSpec {
    const RESET_VALUE: u32 = 0;
}
