#[doc = "Register `stg_syscfg_137` reader"]
pub type R = crate::R<StgSyscfg137Spec>;
#[doc = "Field `u1_pcie_axi4_mst0_awfunc` reader - "]
pub type U1PcieAxi4Mst0AwfuncR = crate::FieldReader<u16>;
#[doc = "Field `u1_pcie_axi4_mst0_awregion` reader - "]
pub type U1PcieAxi4Mst0AwregionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_awfunc(&self) -> U1PcieAxi4Mst0AwfuncR {
        U1PcieAxi4Mst0AwfuncR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn u1_pcie_axi4_mst0_awregion(&self) -> U1PcieAxi4Mst0AwregionR {
        U1PcieAxi4Mst0AwregionR::new(((self.bits >> 15) & 0x0f) as u8)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 548\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_137::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg137Spec;
impl crate::RegisterSpec for StgSyscfg137Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_137::R`](R) reader structure"]
impl crate::Readable for StgSyscfg137Spec {}
#[doc = "`reset()` method sets stg_syscfg_137 to value 0"]
impl crate::Resettable for StgSyscfg137Spec {
    const RESET_VALUE: u32 = 0;
}
