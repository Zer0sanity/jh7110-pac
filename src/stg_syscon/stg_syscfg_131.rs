#[doc = "Register `stg_syscfg_131` reader"]
pub type R = crate::R<StgSyscfg131Spec>;
#[doc = "Field `u1_pcie_axi4_mst0_aratomap_191_160` reader - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
pub type U1PcieAxi4Mst0Aratomap191_160R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_aratomap_191_160(&self) -> U1PcieAxi4Mst0Aratomap191_160R {
        U1PcieAxi4Mst0Aratomap191_160R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 524\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_131::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg131Spec;
impl crate::RegisterSpec for StgSyscfg131Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_131::R`](R) reader structure"]
impl crate::Readable for StgSyscfg131Spec {}
#[doc = "`reset()` method sets stg_syscfg_131 to value 0"]
impl crate::Resettable for StgSyscfg131Spec {
    const RESET_VALUE: u32 = 0;
}
