#[doc = "Register `stg_syscfg_27` reader"]
pub type R = crate::R<StgSyscfg27Spec>;
#[doc = "Field `u0_pcie_axi4_mst0_aruser_31_0` reader - PCIE AXI4 ARUSER (little-endian)"]
pub type U0PcieAxi4Mst0Aruser31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 ARUSER (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_aruser_31_0(&self) -> U0PcieAxi4Mst0Aruser31_0R {
        U0PcieAxi4Mst0Aruser31_0R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_27::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg27Spec;
impl crate::RegisterSpec for StgSyscfg27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_27::R`](R) reader structure"]
impl crate::Readable for StgSyscfg27Spec {}
#[doc = "`reset()` method sets stg_syscfg_27 to value 0"]
impl crate::Resettable for StgSyscfg27Spec {
    const RESET_VALUE: u32 = 0;
}
