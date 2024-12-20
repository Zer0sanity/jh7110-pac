#[doc = "Register `stg_syscfg_87` reader"]
pub type R = crate::R<StgSyscfg87Spec>;
#[doc = "Field `u0_pcie_pl_sideband_out_63_32` reader - PCIE PL Sideband OUT (little-endian)"]
pub type U0PciePlSidebandOut63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE PL Sideband OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_pl_sideband_out_63_32(&self) -> U0PciePlSidebandOut63_32R {
        U0PciePlSidebandOut63_32R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 348\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_87::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg87Spec;
impl crate::RegisterSpec for StgSyscfg87Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_87::R`](R) reader structure"]
impl crate::Readable for StgSyscfg87Spec {}
#[doc = "`reset()` method sets stg_syscfg_87 to value 0"]
impl crate::Resettable for StgSyscfg87Spec {
    const RESET_VALUE: u32 = 0;
}
