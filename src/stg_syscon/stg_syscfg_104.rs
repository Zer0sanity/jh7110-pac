#[doc = "Register `stg_syscfg_104` reader"]
pub type R = crate::R<StgSyscfg104Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_447_416` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge447_416R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_447_416(&self) -> U0PcieTestOutBridge447_416R {
        U0PcieTestOutBridge447_416R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 416\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_104::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg104Spec;
impl crate::RegisterSpec for StgSyscfg104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_104::R`](R) reader structure"]
impl crate::Readable for StgSyscfg104Spec {}
#[doc = "`reset()` method sets stg_syscfg_104 to value 0"]
impl crate::Resettable for StgSyscfg104Spec {
    const RESET_VALUE: u32 = 0;
}
