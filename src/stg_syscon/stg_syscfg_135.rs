#[doc = "Register `stg_syscfg_135` reader"]
pub type R = crate::R<StgSyscfg135Spec>;
#[doc = "Field `u1_pcie_axi4_mst0_aruser_31_0` reader - PCIE AXI4 ARUSER (little-endian)"]
pub type U1PcieAxi4Mst0Aruser31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 ARUSER (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_aruser_31_0(&self) -> U1PcieAxi4Mst0Aruser31_0R {
        U1PcieAxi4Mst0Aruser31_0R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 540\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_135::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg135Spec;
impl crate::RegisterSpec for StgSyscfg135Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_135::R`](R) reader structure"]
impl crate::Readable for StgSyscfg135Spec {}
#[doc = "`reset()` method sets stg_syscfg_135 to value 0"]
impl crate::Resettable for StgSyscfg135Spec {
    const RESET_VALUE: u32 = 0;
}
