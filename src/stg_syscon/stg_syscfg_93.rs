#[doc = "Register `stg_syscfg_93` reader"]
pub type R = crate::R<StgSyscfg93Spec>;
#[doc = "Field `u0_pcie_test_out_bridge_95_64` reader - PCIE Test OUT Bridge (little-endian)"]
pub type U0PcieTestOutBridge95_64R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PCIE Test OUT Bridge (little-endian)"]
    #[inline(always)]
    pub fn u0_pcie_test_out_bridge_95_64(&self) -> U0PcieTestOutBridge95_64R {
        U0PcieTestOutBridge95_64R::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 372\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_93::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg93Spec;
impl crate::RegisterSpec for StgSyscfg93Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_93::R`](R) reader structure"]
impl crate::Readable for StgSyscfg93Spec {}
#[doc = "`reset()` method sets stg_syscfg_93 to value 0"]
impl crate::Resettable for StgSyscfg93Spec {
    const RESET_VALUE: u32 = 0;
}
