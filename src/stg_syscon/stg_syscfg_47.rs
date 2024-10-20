#[doc = "Register `stg_syscfg_47` reader"]
pub type R = crate::R<StgSyscfg47Spec>;
#[doc = "Field `u0_pcie_axi4_slv0_ruser` reader - PCIE AXI4 SLV0 RUSER"]
pub type U0PcieAxi4Slv0RuserR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE AXI4 SLV0 RUSER"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slv0_ruser(&self) -> U0PcieAxi4Slv0RuserR {
        U0PcieAxi4Slv0RuserR::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 188\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_47::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg47Spec;
impl crate::RegisterSpec for StgSyscfg47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_47::R`](R) reader structure"]
impl crate::Readable for StgSyscfg47Spec {}
#[doc = "`reset()` method sets stg_syscfg_47 to value 0"]
impl crate::Resettable for StgSyscfg47Spec {
    const RESET_VALUE: u32 = 0;
}
