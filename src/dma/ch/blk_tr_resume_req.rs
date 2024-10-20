#[doc = "Register `blk_tr_resume_req` writer"]
pub type W = crate::W<BlkTrResumeReqSpec>;
#[doc = "Field `blk_tr_resume_req` writer - Block Transfer Resume Request during Linked-List or Shadow-Register-based multi-block transfer - 0: no request, 1: request"]
pub type BlkTrResumeReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Block Transfer Resume Request during Linked-List or Shadow-Register-based multi-block transfer - 0: no request, 1: request"]
    #[inline(always)]
    #[must_use]
    pub fn blk_tr_resume_req(&mut self) -> BlkTrResumeReqW<BlkTrResumeReqSpec> {
        BlkTrResumeReqW::new(self, 0)
    }
}
#[doc = "Block Transfer Resume Request register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk_tr_resume_req::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkTrResumeReqSpec;
impl crate::RegisterSpec for BlkTrResumeReqSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`blk_tr_resume_req::W`](W) writer structure"]
impl crate::Writable for BlkTrResumeReqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets blk_tr_resume_req to value 0"]
impl crate::Resettable for BlkTrResumeReqSpec {
    const RESET_VALUE: u64 = 0;
}
