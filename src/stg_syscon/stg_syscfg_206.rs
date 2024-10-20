#[doc = "Register `stg_syscfg_206` reader"]
pub type R = crate::R<StgSyscfg206Spec>;
#[doc = "Field `u1_pcie_test_out_bridge_255_224` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U1PcieTestOutBridge255_224R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_test_out_bridge_255_224(&self) -> U1PcieTestOutBridge255_224R {
        U1PcieTestOutBridge255_224R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 824\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_206::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg206Spec;
impl crate::RegisterSpec for StgSyscfg206Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_206::R`](R) reader structure"]
impl crate::Readable for StgSyscfg206Spec {}
#[doc = "`reset()` method sets stg_syscfg_206 to value 0"]
impl crate::Resettable for StgSyscfg206Spec {
    const RESET_VALUE: u32 = 0;
}