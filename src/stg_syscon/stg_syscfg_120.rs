#[doc = "Register `stg_syscfg_120` reader"]
pub type R = crate::R<StgSyscfg120Spec>;
#[doc = "Field `u0_pcie_test_out_447_416` reader - PCIE Test OUT (little-endian)"]
pub type U0PcieTestOut447_416R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_447_416(&self) -> U0PcieTestOut447_416R {
        U0PcieTestOut447_416R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 480\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_120::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg120Spec;
impl crate::RegisterSpec for StgSyscfg120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_120::R`](R) reader structure"]
impl crate::Readable for StgSyscfg120Spec {}
#[doc = "`reset()` method sets stg_syscfg_120 to value 0"]
impl crate::Resettable for StgSyscfg120Spec {
    const RESET_VALUE: u32 = 0;
}
