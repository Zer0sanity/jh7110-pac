#[doc = "Register `reset` reader"]
pub type R = crate::R<ResetSpec>;
#[doc = "Register `reset` writer"]
pub type W = crate::W<ResetSpec>;
#[doc = "Field `rst` reader - DMAC Reset - 0: no-op, 1: request reset. **NOTE** Software is not allowed to write 0 to this bit."]
pub type RstR = crate::BitReader;
#[doc = "Field `rst` writer - DMAC Reset - 0: no-op, 1: request reset. **NOTE** Software is not allowed to write 0 to this bit."]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMAC Reset - 0: no-op, 1: request reset. **NOTE** Software is not allowed to write 0 to this bit."]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMAC Reset - 0: no-op, 1: request reset. **NOTE** Software is not allowed to write 0 to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<ResetSpec> {
        RstW::new(self, 0)
    }
}
#[doc = "DMAC Channel Interrupt Status register contains the DMAC channel interrupt status. Only exists when DMAX_NUM_CHANNELS > 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetSpec;
impl crate::RegisterSpec for ResetSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`reset::R`](R) reader structure"]
impl crate::Readable for ResetSpec {}
#[doc = "`write(|w| ..)` method takes [`reset::W`](W) writer structure"]
impl crate::Writable for ResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets reset to value 0"]
impl crate::Resettable for ResetSpec {
    const RESET_VALUE: u64 = 0;
}
