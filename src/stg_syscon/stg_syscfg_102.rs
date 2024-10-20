#[doc = "Register `stg_syscfg_102` reader"]
pub type R = crate::R<StgSyscfg102Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_383_352` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge383_352R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_383_352(&self) -> U0PcieTestOutBridge383_352R {
        U0PcieTestOutBridge383_352R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 408\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_102::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg102Spec;
impl crate::RegisterSpec for StgSyscfg102Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_102::R`](R) reader structure"]
impl crate::Readable for StgSyscfg102Spec {}
#[doc = "`reset()` method sets stg_syscfg_102 to value 0"]
impl crate::Resettable for StgSyscfg102Spec {
    const RESET_VALUE: u32 = 0;
}
