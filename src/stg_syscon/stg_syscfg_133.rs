#[doc = "Register `stg_syscfg_133` reader"]
pub type R = crate::R<StgSyscfg133Spec>;
#[doc = "Field `u1_pcie_axi4_mst0_aratomap_255_224` reader - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
pub type U1PcieAxi4Mst0Aratomap255_224R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 MST0 ARATOMAP (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_aratomap_255_224(&self) -> U1PcieAxi4Mst0Aratomap255_224R {
        U1PcieAxi4Mst0Aratomap255_224R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 532\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_133::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg133Spec;
impl crate::RegisterSpec for StgSyscfg133Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_133::R`](R) reader structure"]
impl crate::Readable for StgSyscfg133Spec {}
#[doc = "`reset()` method sets stg_syscfg_133 to value 0"]
impl crate::Resettable for StgSyscfg133Spec {
    const RESET_VALUE: u32 = 0;
}
