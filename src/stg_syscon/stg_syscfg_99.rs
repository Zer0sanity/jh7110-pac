#[doc = "Register `stg_syscfg_99` reader"]
pub type R = crate::R<StgSyscfg99Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_287_256` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge287_256R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_287_256(&self) -> U0PcieTestOutBridge287_256R {
        U0PcieTestOutBridge287_256R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 396\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_99::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg99Spec;
impl crate::RegisterSpec for StgSyscfg99Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_99::R`](R) reader structure"]
impl crate::Readable for StgSyscfg99Spec {}
#[doc = "`reset()` method sets stg_syscfg_99 to value 0"]
impl crate::Resettable for StgSyscfg99Spec {
    const RESET_VALUE: u32 = 0;
}
