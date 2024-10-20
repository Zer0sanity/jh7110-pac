#[doc = "Register `stg_syscfg_101` reader"]
pub type R = crate::R<StgSyscfg101Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_351_320` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge351_320R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_351_320(&self) -> U0PcieTestOutBridge351_320R {
        U0PcieTestOutBridge351_320R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 404\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_101::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg101Spec;
impl crate::RegisterSpec for StgSyscfg101Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_101::R`](R) reader structure"]
impl crate::Readable for StgSyscfg101Spec {}
#[doc = "`reset()` method sets stg_syscfg_101 to value 0"]
impl crate::Resettable for StgSyscfg101Spec {
    const RESET_VALUE: u32 = 0;
}
