#[doc = "Register `stg_syscfg_10` reader"]
pub type R = crate::R<StgSyscfg10Spec>;
#[doc = "Field `u0_e2_wfi_from_tile_0` reader - "]
pub type U0E2WfiFromTile0R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn u0_e2_wfi_from_tile_0(&self) -> U0E2WfiFromTile0R {
        U0E2WfiFromTile0R::new((self.bits & 1) != 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg10Spec;
impl crate::RegisterSpec for StgSyscfg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_10::R`](R) reader structure"]
impl crate::Readable for StgSyscfg10Spec {}
#[doc = "`reset()` method sets stg_syscfg_10 to value 0"]
impl crate::Resettable for StgSyscfg10Spec {
    const RESET_VALUE: u32 = 0;
}
