#[doc = "Register `stg_syscfg_33` reader"]
pub type R = crate::R<StgSyscfg33Spec>;
#[doc = "Field `u0_pcie_axi4_mst0_wderr` reader - PCIE AXI4 WDERR"]
pub type U0PcieAxi4Mst0WderrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PCIE AXI4 WDERR"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_wderr(&self) -> U0PcieAxi4Mst0WderrR {
        U0PcieAxi4Mst0WderrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 132\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_33::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg33Spec;
impl crate::RegisterSpec for StgSyscfg33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_33::R`](R) reader structure"]
impl crate::Readable for StgSyscfg33Spec {}
#[doc = "`reset()` method sets stg_syscfg_33 to value 0"]
impl crate::Resettable for StgSyscfg33Spec {
    const RESET_VALUE: u32 = 0;
}
