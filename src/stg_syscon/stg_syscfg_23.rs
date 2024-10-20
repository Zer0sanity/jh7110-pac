#[doc = "Register `stg_syscfg_23` reader"]
pub type R = crate::R<StgSyscfg23Spec>;
#[doc = "Field `u0_pcie_axi4_mst0_aratomop_191_160` reader - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
pub type U0PcieAxi4Mst0Aratomop191_160R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_aratomop_191_160(&self) -> U0PcieAxi4Mst0Aratomop191_160R {
        U0PcieAxi4Mst0Aratomop191_160R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_23::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg23Spec;
impl crate::RegisterSpec for StgSyscfg23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_23::R`](R) reader structure"]
impl crate::Readable for StgSyscfg23Spec {}
#[doc = "`reset()` method sets stg_syscfg_23 to value 0"]
impl crate::Resettable for StgSyscfg23Spec {
    const RESET_VALUE: u32 = 0;
}
