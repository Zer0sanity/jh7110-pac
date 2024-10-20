#[doc = "Register `stg_syscfg_24` reader"]
pub type R = crate::R<StgSyscfg24Spec>;
#[doc = "Field `u0_pcie_axi4_mst0_aratomop_223_192` reader - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
pub type U0PcieAxi4Mst0Aratomop223_192R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 ARATOMOP MST0 (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_aratomop_223_192(&self) -> U0PcieAxi4Mst0Aratomop223_192R {
        U0PcieAxi4Mst0Aratomop223_192R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_24::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg24Spec;
impl crate::RegisterSpec for StgSyscfg24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_24::R`](R) reader structure"]
impl crate::Readable for StgSyscfg24Spec {}
#[doc = "`reset()` method sets stg_syscfg_24 to value 0"]
impl crate::Resettable for StgSyscfg24Spec {
    const RESET_VALUE: u32 = 0;
}
