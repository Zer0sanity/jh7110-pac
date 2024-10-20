#[doc = "Register `_ecc_reserved` reader"]
pub type R = crate::R<_EccReservedSpec>;
#[doc = "Register `_ecc_reserved` writer"]
pub type W = crate::W<_EccReservedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<_EccReservedSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "L2 Cache Control ECC Type (`directory`, `data`) reserved register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_ecc_reserved::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_ecc_reserved::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _EccReservedSpec;
impl crate::RegisterSpec for _EccReservedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_ecc_reserved::R`](R) reader structure"]
impl crate::Readable for _EccReservedSpec {}
#[doc = "`write(|w| ..)` method takes [`_ecc_reserved::W`](W) writer structure"]
impl crate::Writable for _EccReservedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets _ecc_reserved to value 0"]
impl crate::Resettable for _EccReservedSpec {
    const RESET_VALUE: u32 = 0;
}
