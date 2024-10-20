#[doc = "Register `stg_syscfg_28` reader"]
pub type R = crate::R<StgSyscfg28Spec>;
#[doc = "Field `u0_pcie_axi4_mst0_aruser_52_32` reader - PCIE AXI4 ARUSER (little-endian)"]
pub type U0PcieAxi4Mst0Aruser52_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:20 - PCIE AXI4 ARUSER (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_axi4_mst0_aruser_52_32(&self) -> U0PcieAxi4Mst0Aruser52_32R {
        U0PcieAxi4Mst0Aruser52_32R::new(self.bits & 0x001f_ffff)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 112\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_28::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg28Spec;
impl crate::RegisterSpec for StgSyscfg28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_28::R`](R) reader structure"]
impl crate::Readable for StgSyscfg28Spec {}
#[doc = "`reset()` method sets stg_syscfg_28 to value 0"]
impl crate::Resettable for StgSyscfg28Spec {
    const RESET_VALUE: u32 = 0;
}
