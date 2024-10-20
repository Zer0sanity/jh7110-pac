#[doc = "Register `stg_syscfg_207` reader"]
pub type R = crate::R<StgSyscfg207Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_287_256` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge287_256R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_287_256(&self) -> U1PcieTestOutBridge287_256R {
        U1PcieTestOutBridge287_256R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 828\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_207::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg207Spec;
impl crate::RegisterSpec for StgSyscfg207Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_207::R`](R) reader structure"]
impl crate::Readable for StgSyscfg207Spec {}
#[doc = "`reset()` method sets stg_syscfg_207 to value 0"]
impl crate::Resettable for StgSyscfg207Spec {
    const RESET_VALUE: u32 = 0;
}
