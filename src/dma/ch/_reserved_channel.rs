#[doc = "Register `_reserved_channel` reader"]
pub type R = crate::R<_ReservedChannelSpec>;
#[doc = "Register `_reserved_channel` writer"]
pub type W = crate::W<_ReservedChannelSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<_ReservedChannelSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "DMAC Reserved register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_reserved_channel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_reserved_channel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _ReservedChannelSpec;
impl crate::RegisterSpec for _ReservedChannelSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`_reserved_channel::R`](R) reader structure"]
impl crate::Readable for _ReservedChannelSpec {}
#[doc = "`write(|w| ..)` method takes [`_reserved_channel::W`](W) writer structure"]
impl crate::Writable for _ReservedChannelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets _reserved_channel to value 0"]
impl crate::Resettable for _ReservedChannelSpec {
    const RESET_VALUE: u64 = 0;
}
