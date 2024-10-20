#[doc = "Register `int_clear` writer"]
pub type W = crate::W<IntClearSpec>;
#[doc = "Field `int_clear` writer - Clear interrupt, and reload the counter - 0: no-op, 1: clear/reload."]
pub type IntClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear interrupt, and reload the counter - 0: no-op, 1: clear/reload."]
    #[inline(always)]
    #[must_use]
    pub fn int_clear(&mut self) -> IntClearW<IntClearSpec> {
        IntClearW::new(self, 0)
    }
}
#[doc = "StarFive JH7110 Watchdog Interrupt Clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClearSpec;
impl crate::RegisterSpec for IntClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clear::W`](W) writer structure"]
impl crate::Writable for IntClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets int_clear to value 0"]
impl crate::Resettable for IntClearSpec {
    const RESET_VALUE: u32 = 0;
}
