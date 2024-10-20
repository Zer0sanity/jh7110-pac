#[doc = "Register `error_clear` writer"]
pub type W = crate::W<ErrorClearSpec>;
#[doc = "Field `error_clear` writer - Interrupt error clear."]
pub type ErrorClearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Interrupt error clear."]
    #[inline(always)]
    #[must_use]
    pub fn error_clear(&mut self) -> ErrorClearW<ErrorClearSpec> {
        ErrorClearW::new(self, 0)
    }
}
#[doc = "Interrupt Error Clear Register - clears the error interrupt requests. When writing to this register, each data bit that is HIGH causes the corresponding bit in the Status Register to be cleared. Data bits that are LOW have no effect on the corresponding bit in the register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorClearSpec;
impl crate::RegisterSpec for ErrorClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`error_clear::W`](W) writer structure"]
impl crate::Writable for ErrorClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets error_clear to value 0"]
impl crate::Resettable for ErrorClearSpec {
    const RESET_VALUE: u32 = 0;
}
