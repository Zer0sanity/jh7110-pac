#[doc = "Register `stg_syscfg_128` reader"]
pub type R = crate::R<StgSyscfg128Spec>;
#[doc = "Field `u1_pcie_axi4_mst0_aratomap_95_64` reader - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
pub type U1PcieAxi4Mst0Aratomap95_64R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_aratomap_95_64(&self) -> U1PcieAxi4Mst0Aratomap95_64R {
        U1PcieAxi4Mst0Aratomap95_64R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 512\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_128::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg128Spec;
impl crate::RegisterSpec for StgSyscfg128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_128::R`](R) reader structure"]
impl crate::Readable for StgSyscfg128Spec {}
#[doc = "`reset()` method sets stg_syscfg_128 to value 0"]
impl crate::Resettable for StgSyscfg128Spec {
    const RESET_VALUE: u32 = 0;
}
