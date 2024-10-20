#[doc = "Register `stg_syscfg_136` reader"]
pub type R = crate::R<StgSyscfg136Spec>;
#[doc = "Field `u1_pcie_axi4_mst0_aruser_52_32` reader - PCIE AXI4 ARUSER (little-endian)"]
pub type U1PcieAxi4Mst0Aruser52_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:20 - PCIE AXI4 ARUSER (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_aruser_52_32(&self) -> U1PcieAxi4Mst0Aruser52_32R {
        U1PcieAxi4Mst0Aruser52_32R::new(self.bits & 0x001f_ffff)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 544\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_136::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg136Spec;
impl crate::RegisterSpec for StgSyscfg136Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_136::R`](R) reader structure"]
impl crate::Readable for StgSyscfg136Spec {}
#[doc = "`reset()` method sets stg_syscfg_136 to value 0"]
impl crate::Resettable for StgSyscfg136Spec {
    const RESET_VALUE: u32 = 0;
}
