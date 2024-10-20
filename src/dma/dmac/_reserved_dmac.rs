#[doc = "Register `_reserved_dmac` reader"]
pub type R = crate::R<_ReservedDmacSpec>;
#[doc = "Register `_reserved_dmac` writer"]
pub type W = crate::W<_ReservedDmacSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<_ReservedDmacSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "DMAC Reserved register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_reserved_dmac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_reserved_dmac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _ReservedDmacSpec;
impl crate::RegisterSpec for _ReservedDmacSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`_reserved_dmac::R`](R) reader structure"]
impl crate::Readable for _ReservedDmacSpec {}
#[doc = "`write(|w| ..)` method takes [`_reserved_dmac::W`](W) writer structure"]
impl crate::Writable for _ReservedDmacSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets _reserved_dmac to value 0"]
impl crate::Resettable for _ReservedDmacSpec {
    const RESET_VALUE: u64 = 0;
}
