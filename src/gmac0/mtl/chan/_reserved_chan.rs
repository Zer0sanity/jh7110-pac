#[doc = "Register `_reserved_chan` reader"]
pub type R = crate::R<_ReservedChanSpec>;
#[doc = "Register `_reserved_chan` writer"]
pub type W = crate::W<_ReservedChanSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<_ReservedChanSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "MTL Channel Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_reserved_chan::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_reserved_chan::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _ReservedChanSpec;
impl crate::RegisterSpec for _ReservedChanSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_reserved_chan::R`](R) reader structure"]
impl crate::Readable for _ReservedChanSpec {}
#[doc = "`write(|w| ..)` method takes [`_reserved_chan::W`](W) writer structure"]
impl crate::Writable for _ReservedChanSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets _reserved_chan to value 0"]
impl crate::Resettable for _ReservedChanSpec {
    const RESET_VALUE: u32 = 0;
}
