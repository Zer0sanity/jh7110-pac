#[doc = "Register `stg_syscfg_103` reader"]
pub type R = crate::R<StgSyscfg103Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_415_384` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge415_384R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_415_384(&self) -> U0PcieTestOutBridge415_384R {
        U0PcieTestOutBridge415_384R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 412\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_103::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg103Spec;
impl crate::RegisterSpec for StgSyscfg103Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_103::R`](R) reader structure"]
impl crate::Readable for StgSyscfg103Spec {}
#[doc = "`reset()` method sets stg_syscfg_103 to value 0"]
impl crate::Resettable for StgSyscfg103Spec {
    const RESET_VALUE: u32 = 0;
}
