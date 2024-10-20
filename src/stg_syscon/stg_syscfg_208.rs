#[doc = "Register `stg_syscfg_208` reader"]
pub type R = crate::R<StgSyscfg208Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_319_288` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge319_288R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_319_288(&self) -> U1PcieTestOutBridge319_288R {
        U1PcieTestOutBridge319_288R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 832\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_208::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg208Spec;
impl crate::RegisterSpec for StgSyscfg208Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_208::R`](R) reader structure"]
impl crate::Readable for StgSyscfg208Spec {}
#[doc = "`reset()` method sets stg_syscfg_208 to value 0"]
impl crate::Resettable for StgSyscfg208Spec {
    const RESET_VALUE: u32 = 0;
}
