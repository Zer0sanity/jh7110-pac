#[doc = "Register `stg_syscfg_130` reader"]
pub type R = crate::R<StgSyscfg130Spec>;
#[doc = "Field `u1_pcie_axi4_mst0_aratomap_159_128` reader - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
pub type U1PcieAxi4Mst0Aratomap159_128R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_aratomap_159_128(&self) -> U1PcieAxi4Mst0Aratomap159_128R {
        U1PcieAxi4Mst0Aratomap159_128R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 520\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_130::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg130Spec;
impl crate::RegisterSpec for StgSyscfg130Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_130::R`](R) reader structure"]
impl crate::Readable for StgSyscfg130Spec {}
#[doc = "`reset()` method sets stg_syscfg_130 to value 0"]
impl crate::Resettable for StgSyscfg130Spec {
    const RESET_VALUE: u32 = 0;
}
