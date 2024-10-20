#[doc = "Register `stg_syscfg_199` reader"]
pub type R = crate::R<StgSyscfg199Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_31_0` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_31_0(&self) -> U1PcieTestOutBridge31_0R {
        U1PcieTestOutBridge31_0R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 796\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_199::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg199Spec;
impl crate::RegisterSpec for StgSyscfg199Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_199::R`](R) reader structure"]
impl crate::Readable for StgSyscfg199Spec {}
#[doc = "`reset()` method sets stg_syscfg_199 to value 0"]
impl crate::Resettable for StgSyscfg199Spec {
    const RESET_VALUE: u32 = 0;
}
