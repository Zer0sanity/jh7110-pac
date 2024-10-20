#[doc = "Register `stg_syscfg_129` reader"]
pub type R = crate::R<StgSyscfg129Spec>;
#[doc = "Field `u1_pcie_axi4_mst0_aratomap_127_96` reader - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
pub type U1PcieAxi4Mst0Aratomap127_96R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_aratomap_127_96(&self) -> U1PcieAxi4Mst0Aratomap127_96R {
        U1PcieAxi4Mst0Aratomap127_96R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 516\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_129::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg129Spec;
impl crate::RegisterSpec for StgSyscfg129Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_129::R`](R) reader structure"]
impl crate::Readable for StgSyscfg129Spec {}
#[doc = "`reset()` method sets stg_syscfg_129 to value 0"]
impl crate::Resettable for StgSyscfg129Spec {
    const RESET_VALUE: u32 = 0;
}
