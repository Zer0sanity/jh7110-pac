#[doc = "Register `stg_syscfg_86` reader"]
pub type R = crate::R<StgSyscfg86Spec>;
#[doc = "Field `u0_pcie_pl_sideband_out_31_0` reader - PCIE PL Sideband OUT (little-endian)"]
pub type U0PciePlSidebandOut31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE PL Sideband OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_pl_sideband_out_31_0(&self) -> U0PciePlSidebandOut31_0R {
        U0PciePlSidebandOut31_0R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 344\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_86::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg86Spec;
impl crate::RegisterSpec for StgSyscfg86Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_86::R`](R) reader structure"]
impl crate::Readable for StgSyscfg86Spec {}
#[doc = "`reset()` method sets stg_syscfg_86 to value 0"]
impl crate::Resettable for StgSyscfg86Spec {
    const RESET_VALUE: u32 = 0;
}
