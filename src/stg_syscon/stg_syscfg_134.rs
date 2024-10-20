#[doc = "Register `stg_syscfg_134` reader"]
pub type R = crate::R<StgSyscfg134Spec>;
#[doc = "Field `u1_pcie_axi4_mst0_aratomop_257_256` reader - PCIE AXI4 ARATOMOP (little-endian)"]
pub type U1PcieAxi4Mst0Aratomop257_256R = crate::FieldReader;
#[doc = "Field `u1_pcie_axi4_mst0_arfunc` reader - PCIE AXI4 ARFUNC"]
pub type U1PcieAxi4Mst0ArfuncR = crate::FieldReader<u16>;
#[doc = "Field `u1_pcie_axi4_mst0_arregion` reader - PCIE AXI4 ARREGION"]
pub type U1PcieAxi4Mst0ArregionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - PCIE AXI4 ARATOMOP (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_aratomop_257_256(&self) -> U1PcieAxi4Mst0Aratomop257_256R {
        U1PcieAxi4Mst0Aratomop257_256R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:16 - PCIE AXI4 ARFUNC"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_arfunc(&self) -> U1PcieAxi4Mst0ArfuncR {
        U1PcieAxi4Mst0ArfuncR::new(((self.bits >> 2) & 0x7fff) as u16)
    }
    #[doc = "Bits 17:20 - PCIE AXI4 ARREGION"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_arregion(&self) -> U1PcieAxi4Mst0ArregionR {
        U1PcieAxi4Mst0ArregionR::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 536\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_134::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg134Spec;
impl crate::RegisterSpec for StgSyscfg134Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_134::R`](R) reader structure"]
impl crate::Readable for StgSyscfg134Spec {}
#[doc = "`reset()` method sets stg_syscfg_134 to value 0"]
impl crate::Resettable for StgSyscfg134Spec {
    const RESET_VALUE: u32 = 0;
}
