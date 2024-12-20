#[doc = "Register `stg_syscfg_194` reader"]
pub type R = crate::R<StgSyscfg194Spec>;
#[doc = "Field `u1_pcie_pl_sideband_out_31_0` reader - PCIE PL Sideband OUT (little-endian)"]
pub type U1PciePlSidebandOut31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE PL Sideband OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_pl_sideband_out_31_0(&self) -> U1PciePlSidebandOut31_0R {
        U1PciePlSidebandOut31_0R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 776\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_194::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg194Spec;
impl crate::RegisterSpec for StgSyscfg194Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_194::R`](R) reader structure"]
impl crate::Readable for StgSyscfg194Spec {}
#[doc = "`reset()` method sets stg_syscfg_194 to value 0"]
impl crate::Resettable for StgSyscfg194Spec {
    const RESET_VALUE: u32 = 0;
}
