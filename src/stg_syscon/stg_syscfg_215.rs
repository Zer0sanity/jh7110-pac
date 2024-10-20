#[doc = "Register `stg_syscfg_215` reader"]
pub type R = crate::R<StgSyscfg215Spec>;
#[doc = "Field `u1_pcie_test_out_31_0` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_31_0(&self) -> U1PcieTestOut31_0R {
        U1PcieTestOut31_0R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 860\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_215::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg215Spec;
impl crate::RegisterSpec for StgSyscfg215Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_215::R`](R) reader structure"]
impl crate::Readable for StgSyscfg215Spec {}
#[doc = "`reset()` method sets stg_syscfg_215 to value 0"]
impl crate::Resettable for StgSyscfg215Spec {
    const RESET_VALUE: u32 = 0;
}
