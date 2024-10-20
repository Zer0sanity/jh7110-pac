#[doc = "Register `stg_syscfg_100` reader"]
pub type R = crate::R<StgSyscfg100Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_319_288` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge319_288R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_319_288(&self) -> U0PcieTestOutBridge319_288R {
        U0PcieTestOutBridge319_288R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 400\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_100::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg100Spec;
impl crate::RegisterSpec for StgSyscfg100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_100::R`](R) reader structure"]
impl crate::Readable for StgSyscfg100Spec {}
#[doc = "`reset()` method sets stg_syscfg_100 to value 0"]
impl crate::Resettable for StgSyscfg100Spec {
    const RESET_VALUE: u32 = 0;
}
