#[doc = "Register `stg_syscfg_96` reader"]
pub type R = crate::R<StgSyscfg96Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_191_160` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge191_160R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_191_160(&self) -> U0PcieTestOutBridge191_160R {
        U0PcieTestOutBridge191_160R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 384\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_96::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg96Spec;
impl crate::RegisterSpec for StgSyscfg96Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_96::R`](R) reader structure"]
impl crate::Readable for StgSyscfg96Spec {}
#[doc = "`reset()` method sets stg_syscfg_96 to value 0"]
impl crate::Resettable for StgSyscfg96Spec {
    const RESET_VALUE: u32 = 0;
}
