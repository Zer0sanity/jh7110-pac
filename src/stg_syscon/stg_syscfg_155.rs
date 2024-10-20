#[doc = "Register `stg_syscfg_155` reader"]
pub type R = crate::R<StgSyscfg155Spec>;
#[doc = "Field `u1_pcie_axi4_slv0_ruser` reader - PCIE AXI4 SLV0 RUSER"]
pub type U1PcieAxi4Slv0RuserR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 SLV0 RUSER"]
    #[inline(always)]
    pub fn u1_pcie_axi4_slv0_ruser(&self) -> U1PcieAxi4Slv0RuserR {
        U1PcieAxi4Slv0RuserR::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 620\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_155::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg155Spec;
impl crate::RegisterSpec for StgSyscfg155Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_155::R`](R) reader structure"]
impl crate::Readable for StgSyscfg155Spec {}
#[doc = "`reset()` method sets stg_syscfg_155 to value 0"]
impl crate::Resettable for StgSyscfg155Spec {
    const RESET_VALUE: u32 = 0;
}
