#[doc = "Register `stg_syscfg_126` reader"]
pub type R = crate::R<StgSyscfg126Spec>;
#[doc = "Field `u1_pcie_axi4_mst0_aratomap_31_0` reader - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
pub type U1PcieAxi4Mst0Aratomap31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_aratomap_31_0(&self) -> U1PcieAxi4Mst0Aratomap31_0R {
        U1PcieAxi4Mst0Aratomap31_0R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 504\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_126::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg126Spec;
impl crate::RegisterSpec for StgSyscfg126Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_126::R`](R) reader structure"]
impl crate::Readable for StgSyscfg126Spec {}
#[doc = "`reset()` method sets stg_syscfg_126 to value 0"]
impl crate::Resettable for StgSyscfg126Spec {
    const RESET_VALUE: u32 = 0;
}
