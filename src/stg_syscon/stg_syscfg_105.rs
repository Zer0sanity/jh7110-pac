#[doc = "Register `stg_syscfg_105` reader"]
pub type R = crate::R<StgSyscfg105Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_479_448` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge479_448R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_479_448(&self) -> U0PcieTestOutBridge479_448R {
        U0PcieTestOutBridge479_448R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 420\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_105::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg105Spec;
impl crate::RegisterSpec for StgSyscfg105Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_105::R`](R) reader structure"]
impl crate::Readable for StgSyscfg105Spec {}
#[doc = "`reset()` method sets stg_syscfg_105 to value 0"]
impl crate::Resettable for StgSyscfg105Spec {
    const RESET_VALUE: u32 = 0;
}
