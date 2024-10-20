#[doc = "Register `stg_syscfg_19` reader"]
pub type R = crate::R<StgSyscfg19Spec>;
#[doc = "Field `u0_pcie_axi4_mst0_aratomop_63_32` reader - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
pub type U0PcieAxi4Mst0Aratomop63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_aratomop_63_32(&self) -> U0PcieAxi4Mst0Aratomop63_32R {
        U0PcieAxi4Mst0Aratomop63_32R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_19::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg19Spec;
impl crate::RegisterSpec for StgSyscfg19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_19::R`](R) reader structure"]
impl crate::Readable for StgSyscfg19Spec {}
#[doc = "`reset()` method sets stg_syscfg_19 to value 0"]
impl crate::Resettable for StgSyscfg19Spec {
    const RESET_VALUE: u32 = 0;
}
