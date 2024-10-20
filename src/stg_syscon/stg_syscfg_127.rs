#[doc = "Register `stg_syscfg_127` reader"]
pub type R = crate::R<StgSyscfg127Spec>;
#[doc = "Field `u1_pcie_axi4_mst0_aratomap_63_32` reader - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
pub type U1PcieAxi4Mst0Aratomap63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_aratomap_63_32(&self) -> U1PcieAxi4Mst0Aratomap63_32R {
        U1PcieAxi4Mst0Aratomap63_32R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 508\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_127::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg127Spec;
impl crate::RegisterSpec for StgSyscfg127Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_127::R`](R) reader structure"]
impl crate::Readable for StgSyscfg127Spec {}
#[doc = "`reset()` method sets stg_syscfg_127 to value 0"]
impl crate::Resettable for StgSyscfg127Spec {
    const RESET_VALUE: u32 = 0;
}
