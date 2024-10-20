#[doc = "Register `stg_syscfg_132` reader"]
pub type R = crate::R<StgSyscfg132Spec>;
#[doc = "Field `u1_pcie_axi4_mst0_aratomap_223_192` reader - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
pub type U1PcieAxi4Mst0Aratomap223_192R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_aratomap_223_192(&self) -> U1PcieAxi4Mst0Aratomap223_192R {
        U1PcieAxi4Mst0Aratomap223_192R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 528\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_132::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg132Spec;
impl crate::RegisterSpec for StgSyscfg132Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_132::R`](R) reader structure"]
impl crate::Readable for StgSyscfg132Spec {}
#[doc = "`reset()` method sets stg_syscfg_132 to value 0"]
impl crate::Resettable for StgSyscfg132Spec {
    const RESET_VALUE: u32 = 0;
}
