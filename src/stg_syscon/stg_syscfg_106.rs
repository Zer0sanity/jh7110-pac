#[doc = "Register `stg_syscfg_106` reader"]
pub type R = crate::R<StgSyscfg106Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_511_480` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge511_480R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_511_480(&self) -> U0PcieTestOutBridge511_480R {
        U0PcieTestOutBridge511_480R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 424\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_106::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg106Spec;
impl crate::RegisterSpec for StgSyscfg106Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_106::R`](R) reader structure"]
impl crate::Readable for StgSyscfg106Spec {}
#[doc = "`reset()` method sets stg_syscfg_106 to value 0"]
impl crate::Resettable for StgSyscfg106Spec {
    const RESET_VALUE: u32 = 0;
}
