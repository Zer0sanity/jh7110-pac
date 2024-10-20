#[doc = "Register `stg_syscfg_111` reader"]
pub type R = crate::R<StgSyscfg111Spec>;
#[doc = "Field `u0_pcie_test_out_159_128` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut159_128R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_159_128(&self) -> U0PcieTestOut159_128R {
        U0PcieTestOut159_128R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 444\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_111::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg111Spec;
impl crate::RegisterSpec for StgSyscfg111Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_111::R`](R) reader structure"]
impl crate::Readable for StgSyscfg111Spec {}
#[doc = "`reset()` method sets stg_syscfg_111 to value 0"]
impl crate::Resettable for StgSyscfg111Spec {
    const RESET_VALUE: u32 = 0;
}
