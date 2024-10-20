#[doc = "Register `stg_syscfg_205` reader"]
pub type R = crate::R<StgSyscfg205Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_223_192` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge223_192R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_223_192(&self) -> U1PcieTestOutBridge223_192R {
        U1PcieTestOutBridge223_192R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 820\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_205::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg205Spec;
impl crate::RegisterSpec for StgSyscfg205Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_205::R`](R) reader structure"]
impl crate::Readable for StgSyscfg205Spec {}
#[doc = "`reset()` method sets stg_syscfg_205 to value 0"]
impl crate::Resettable for StgSyscfg205Spec {
    const RESET_VALUE: u32 = 0;
}
