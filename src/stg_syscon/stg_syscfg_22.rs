#[doc = "Register `stg_syscfg_22` reader"]
pub type R = crate::R<StgSyscfg22Spec>;
#[doc = "Field `u0_pcie_axi4_mst0_aratomop_159_128` reader - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
pub type U0PcieAxi4Mst0Aratomop159_128R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_aratomop_159_128(&self) -> U0PcieAxi4Mst0Aratomop159_128R {
        U0PcieAxi4Mst0Aratomop159_128R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 88\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_22::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg22Spec;
impl crate::RegisterSpec for StgSyscfg22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_22::R`](R) reader structure"]
impl crate::Readable for StgSyscfg22Spec {}
#[doc = "`reset()` method sets stg_syscfg_22 to value 0"]
impl crate::Resettable for StgSyscfg22Spec {
    const RESET_VALUE: u32 = 0;
}
