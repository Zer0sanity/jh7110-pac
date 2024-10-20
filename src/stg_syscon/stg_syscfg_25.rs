#[doc = "Register `stg_syscfg_25` reader"]
pub type R = crate::R<StgSyscfg25Spec>;
#[doc = "Field `u0_pcie_axi4_mst0_aratomop_255_224` reader - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
pub type U0PcieAxi4Mst0Aratomop255_224R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_aratomop_255_224(&self) -> U0PcieAxi4Mst0Aratomop255_224R {
        U0PcieAxi4Mst0Aratomop255_224R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_25::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg25Spec;
impl crate::RegisterSpec for StgSyscfg25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_25::R`](R) reader structure"]
impl crate::Readable for StgSyscfg25Spec {}
#[doc = "`reset()` method sets stg_syscfg_25 to value 0"]
impl crate::Resettable for StgSyscfg25Spec {
    const RESET_VALUE: u32 = 0;
}
