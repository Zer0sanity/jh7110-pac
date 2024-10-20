#[doc = "Register `stg_syscfg_91` reader"]
pub type R = crate::R<StgSyscfg91Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_31_0` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_31_0(&self) -> U0PcieTestOutBridge31_0R {
        U0PcieTestOutBridge31_0R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 364\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_91::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg91Spec;
impl crate::RegisterSpec for StgSyscfg91Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_91::R`](R) reader structure"]
impl crate::Readable for StgSyscfg91Spec {}
#[doc = "`reset()` method sets stg_syscfg_91 to value 0"]
impl crate::Resettable for StgSyscfg91Spec {
    const RESET_VALUE: u32 = 0;
}
