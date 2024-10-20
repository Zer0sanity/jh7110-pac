#[doc = "Register `stg_syscfg_228` reader"]
pub type R = crate::R<StgSyscfg228Spec>;
#[doc = "Field `u1_pcie_test_out_447_416` reader - PCIE Test OUT (little-endian)"]
pub type U1PcieTestOut447_416R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_447_416(&self) -> U1PcieTestOut447_416R {
        U1PcieTestOut447_416R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 912\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_228::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg228Spec;
impl crate::RegisterSpec for StgSyscfg228Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_228::R`](R) reader structure"]
impl crate::Readable for StgSyscfg228Spec {}
#[doc = "`reset()` method sets stg_syscfg_228 to value 0"]
impl crate::Resettable for StgSyscfg228Spec {
    const RESET_VALUE: u32 = 0;
}
