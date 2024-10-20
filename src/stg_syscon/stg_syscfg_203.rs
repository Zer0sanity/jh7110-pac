#[doc = "Register `stg_syscfg_203` reader"]
pub type R = crate::R<StgSyscfg203Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_159_128` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge159_128R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_159_128(&self) -> U1PcieTestOutBridge159_128R {
        U1PcieTestOutBridge159_128R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 812\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_203::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg203Spec;
impl crate::RegisterSpec for StgSyscfg203Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_203::R`](R) reader structure"]
impl crate::Readable for StgSyscfg203Spec {}
#[doc = "`reset()` method sets stg_syscfg_203 to value 0"]
impl crate::Resettable for StgSyscfg203Spec {
    const RESET_VALUE: u32 = 0;
}
