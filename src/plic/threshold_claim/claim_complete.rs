#[doc = "Register `claim_complete` reader"]
pub type R = crate::R<ClaimCompleteSpec>;
#[doc = "Register `claim_complete` writer"]
pub type W = crate::W<ClaimCompleteSpec>;
#[doc = "Field `claim` reader - "]
pub type ClaimR = crate::FieldReader<u32>;
#[doc = "Field `complete` writer - "]
pub type CompleteW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn claim(&self) -> ClaimR {
        ClaimR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn complete(&mut self) -> CompleteW<ClaimCompleteSpec> {
        CompleteW::new(self, 0)
    }
}
#[doc = "Interrupt source `claim` (read) and complete (write) register. The PLIC will write pending interrupt source information to the `claim` register. When the interrupt handler is finished, the interrupt source idendification should be written to the corresponding `complete` register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim_complete::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim_complete::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClaimCompleteSpec;
impl crate::RegisterSpec for ClaimCompleteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claim_complete::R`](R) reader structure"]
impl crate::Readable for ClaimCompleteSpec {}
#[doc = "`write(|w| ..)` method takes [`claim_complete::W`](W) writer structure"]
impl crate::Writable for ClaimCompleteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets claim_complete to value 0"]
impl crate::Resettable for ClaimCompleteSpec {
    const RESET_VALUE: u32 = 0;
}
