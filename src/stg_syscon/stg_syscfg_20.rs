#[doc = "Register `stg_syscfg_20` reader"]
pub type R = crate::R<StgSyscfg20Spec>;
#[doc = "Field `u0_pcie_axi4_mst0_aratomop_95_64` reader - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
pub type U0PcieAxi4Mst0Aratomop95_64R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_aratomop_95_64(&self) -> U0PcieAxi4Mst0Aratomop95_64R {
        U0PcieAxi4Mst0Aratomop95_64R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 80\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_20::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg20Spec;
impl crate::RegisterSpec for StgSyscfg20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_20::R`](R) reader structure"]
impl crate::Readable for StgSyscfg20Spec {}
#[doc = "`reset()` method sets stg_syscfg_20 to value 0"]
impl crate::Resettable for StgSyscfg20Spec {
    const RESET_VALUE: u32 = 0;
}
