#[doc = "Register `stg_syscfg_98` reader"]
pub type R = crate::R<StgSyscfg98Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_255_224` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge255_224R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_255_224(&self) -> U0PcieTestOutBridge255_224R {
        U0PcieTestOutBridge255_224R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 392\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_98::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg98Spec;
impl crate::RegisterSpec for StgSyscfg98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_98::R`](R) reader structure"]
impl crate::Readable for StgSyscfg98Spec {}
#[doc = "`reset()` method sets stg_syscfg_98 to value 0"]
impl crate::Resettable for StgSyscfg98Spec {
    const RESET_VALUE: u32 = 0;
}
