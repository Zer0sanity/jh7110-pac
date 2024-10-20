#[doc = "Register `stg_syscfg_21` reader"]
pub type R = crate::R<StgSyscfg21Spec>;
#[doc = "Field `u0_pcie_axi4_mst0_aratomop_127_96` reader - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
pub type U0PcieAxi4Mst0Aratomop127_96R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_aratomop_127_96(&self) -> U0PcieAxi4Mst0Aratomop127_96R {
        U0PcieAxi4Mst0Aratomop127_96R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_21::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg21Spec;
impl crate::RegisterSpec for StgSyscfg21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_21::R`](R) reader structure"]
impl crate::Readable for StgSyscfg21Spec {}
#[doc = "`reset()` method sets stg_syscfg_21 to value 0"]
impl crate::Resettable for StgSyscfg21Spec {
    const RESET_VALUE: u32 = 0;
}
