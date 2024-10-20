#[doc = "Register `stg_syscfg_141` reader"]
pub type R = crate::R<StgSyscfg141Spec>;
#[doc = "Field `u1_pcie_axi4_mst0_wderr` reader - PCIE AXI4 WDERR"]
pub type U1PcieAxi4Mst0WderrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PCIE AXI4 WDERR"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_wderr(&self) -> U1PcieAxi4Mst0WderrR {
        U1PcieAxi4Mst0WderrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 564\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_141::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg141Spec;
impl crate::RegisterSpec for StgSyscfg141Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_141::R`](R) reader structure"]
impl crate::Readable for StgSyscfg141Spec {}
#[doc = "`reset()` method sets stg_syscfg_141 to value 0"]
impl crate::Resettable for StgSyscfg141Spec {
    const RESET_VALUE: u32 = 0;
}
