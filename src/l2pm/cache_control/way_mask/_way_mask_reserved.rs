#[doc = "Register `_way_mask_reserved` reader"]
pub type R = crate::R<_WayMaskReservedSpec>;
#[doc = "Register `_way_mask_reserved` writer"]
pub type W = crate::W<_WayMaskReservedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<_WayMaskReservedSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "L2 Cache Control Way Mask reserved register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_way_mask_reserved::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_way_mask_reserved::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _WayMaskReservedSpec;
impl crate::RegisterSpec for _WayMaskReservedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_way_mask_reserved::R`](R) reader structure"]
impl crate::Readable for _WayMaskReservedSpec {}
#[doc = "`write(|w| ..)` method takes [`_way_mask_reserved::W`](W) writer structure"]
impl crate::Writable for _WayMaskReservedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets _way_mask_reserved to value 0"]
impl crate::Resettable for _WayMaskReservedSpec {
    const RESET_VALUE: u32 = 0;
}
