#[doc = "Register `tc_clear` writer"]
pub type W = crate::W<TcClearSpec>;
#[doc = "Field `tc_clear` writer - Terminal count request clear."]
pub type TcClearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Terminal count request clear."]
    #[inline(always)]
    #[must_use]
    pub fn tc_clear(&mut self) -> TcClearW<TcClearSpec> {
        TcClearW::new(self, 0)
    }
}
#[doc = "Interrupt Terminal Count Clear Register - clears a terminal count interrupt request. When writing to this register, each data bit that is set HIGH causes the corresponding bit in the Status Register to be cleared. Data bits that are LOW have no effect on the corresponding bit in the register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tc_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcClearSpec;
impl crate::RegisterSpec for TcClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tc_clear::W`](W) writer structure"]
impl crate::Writable for TcClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tc_clear to value 0"]
impl crate::Resettable for TcClearSpec {
    const RESET_VALUE: u32 = 0;
}
