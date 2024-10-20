#[doc = "Register `stg_syscfg_97` reader"]
pub type R = crate::R<StgSyscfg97Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_223_192` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge223_192R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_223_192(&self) -> U0PcieTestOutBridge223_192R {
        U0PcieTestOutBridge223_192R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 388\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_97::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg97Spec;
impl crate::RegisterSpec for StgSyscfg97Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_97::R`](R) reader structure"]
impl crate::Readable for StgSyscfg97Spec {}
#[doc = "`reset()` method sets stg_syscfg_97 to value 0"]
impl crate::Resettable for StgSyscfg97Spec {
    const RESET_VALUE: u32 = 0;
}
