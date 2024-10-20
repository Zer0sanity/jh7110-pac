#[doc = "Register `stg_syscfg_95` reader"]
pub type R = crate::R<StgSyscfg95Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_159_128` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge159_128R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_159_128(&self) -> U0PcieTestOutBridge159_128R {
        U0PcieTestOutBridge159_128R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 380\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_95::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg95Spec;
impl crate::RegisterSpec for StgSyscfg95Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_95::R`](R) reader structure"]
impl crate::Readable for StgSyscfg95Spec {}
#[doc = "`reset()` method sets stg_syscfg_95 to value 0"]
impl crate::Resettable for StgSyscfg95Spec {
    const RESET_VALUE: u32 = 0;
}