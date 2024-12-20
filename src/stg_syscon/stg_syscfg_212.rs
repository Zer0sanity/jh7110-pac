#[doc = "Register `stg_syscfg_212` reader"]
pub type R = crate::R<StgSyscfg212Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_447_416` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge447_416R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_447_416(&self) -> U1PcieTestOutBridge447_416R {
        U1PcieTestOutBridge447_416R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 848\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_212::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg212Spec;
impl crate::RegisterSpec for StgSyscfg212Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_212::R`](R) reader structure"]
impl crate::Readable for StgSyscfg212Spec {}
#[doc = "`reset()` method sets stg_syscfg_212 to value 0"]
impl crate::Resettable for StgSyscfg212Spec {
    const RESET_VALUE: u32 = 0;
}
